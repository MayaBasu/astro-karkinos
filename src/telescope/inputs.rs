use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use rand_distr::num_traits::real::Real;

use crate::geometry::*;

use crate::gridded_data::{SpectralUnits, Unit, DATA1D};
use crate::gridded_data::SpectralUnits::*;
use crate::telescope::effects::{SpatialEffect, SpectralResponse};

pub const kB_CGS:f64 = 1.380649 *10e-16; //erg K−1
pub const h_CGS:f64 = 6.626069 *10e-27; //erg s
pub const c_CGS:f64 = 2.997925 *10e10; // cm s−1

#[derive(Clone, Debug)]
pub struct Spectrum{
    data: Vec<f64>,
    grid: GRID1D,
    units: crate::gridded_data::units::SpectralUnits
}
/*
Spectrum assumptions:
1. One data point per grid point
2. TODO: Maybe check for non negativity?
3. Data values are Regular
 */
impl DATA1D<SpectralUnits> for Spectrum{
    fn new(grid:GRID1D,data:Vec<f64>,units:SpectralUnits)->Spectrum{
        assert_eq!(grid.num_points(),data.len());
        if !Self::is_data_regular(&data){
            panic!("Failed to initialize new SpectralResponse struct:\
                   data values must not be Nan or inf")
        };
        //TODO: Should I check for non-negativity?
        Spectrum{
            grid,
            data,
            units
        }
    }
    fn grid(&self) -> &GRID1D {
        &self.grid
    }
    fn data(&self) -> &Vec<f64> {
        &self.data
    }
    fn unit(&self) -> &SpectralUnits {
        &self.units
    }

    fn multiply(&self, other: &Self) -> Self {
        let data = self.multiply_data(other);
        Self::new(self.grid, data, *self.unit())
    }
}
impl Spectrum{ //https://vitaly.neustroev.net/useful-info/conversions/

    pub fn new_flat(grid:GRID1D,value:f64,units:SpectralUnits)-> Spectrum{
        let data = (0..grid.num_points()).map(|_|value).collect();
        Spectrum::new(grid,data,units)
    }
    pub fn to_cgs(mut self)->Self{
        let new_data = self.grid.locate_grid_points().iter().zip(self.data).map(|(lambda,value)| {
            match self.units {
                F_nu => { value }
                AbMagnitude => {
                    10f64.powf((value + 48.6) / (-2.5))
                },
                Janskys => {
                    (10f64).powi(-23) * value
                },
                f_lambda => {
                    6.63e-27 * value * lambda
                },
                F_lambda => {
                    3.34e-19 * lambda * lambda * value
                }
            }
        }).collect();
        self = Spectrum::new(self.grid, new_data, F_nu);
        self
    }
    pub fn convert_to(mut self,unit:&SpectralUnits)->Self {
        self = self.to_cgs();
        let new_data = self.grid.locate_grid_points().iter().zip(self.data).map(|(lambda,value)| {
            match unit {
                F_nu => {value},
                F_lambda => {3.00e18 * value / (lambda.powi(2))},
                f_lambda => {1.51e26 * value / lambda},
                AbMagnitude => {-2.5 * value.log10() - 48.6},
                Janskys => {(10f64).powi(23) * value},
            }
        }).collect();
        let result = Spectrum::new(
            self.grid,
            new_data,
            *unit
        );
        result
        


    }
    pub fn new_black_body(grid:GRID1D, temp_kelvin:f64)-> Spectrum{
        let data = grid.locate_grid_points().iter().map(|lambda|{
            let scale_factor = 2.0*std::f64::consts::PI*h_CGS*c_CGS.powi(2)/(lambda.powi(5));
            let exponent = h_CGS*c_CGS/(lambda*kB_CGS*temp_kelvin);
            scale_factor*(1.0/(exponent.exp()-1.0)) }
        ).collect();
        Spectrum::new(grid,data,F_lambda)

    }
    pub fn apply_spectral_response(self, spectral_response:&mut SpectralResponse)->Self {
        let spectral_response = spectral_response.regrid(&self.grid);
        assert_eq!(*spectral_response.grid(),self.grid,"Unreachable: Re-grid attempt failed");
        let new_data = self.data.into_iter().enumerate().map(|(i,x)|{
            x*spectral_response.data()[i]
        }).collect();
        Spectrum::new(self.grid,new_data,self.units)
    }
    pub fn integrate(&self)->f64{
        let average:f64 = self.data.iter().sum::<f64>()/(self.grid.num_points() as f64);
        //TODO: add in automatic unit conversions
        println!("WARNING UNIMPLEMENTED");
        average*self.grid.step_size()
    }

    pub fn sum(&self)->f64{
        self.data.iter().sum()
    }
    pub fn to_band(&self, band_pass: &mut SpectralResponse)->f64{
        self.clone().apply_spectral_response(band_pass).integrate()
    }

}

#[derive(Clone,Debug)]
pub struct PointSource{
    pub(crate) location: Point,
    pub(crate) spectrum: Spectrum,
    
}


pub struct Input{
    pub points: Vec<Point>,
    pub weights: Vec<f64>,
    pub spec_refs: Vec<usize>,
    pub spectra: Vec<Spectrum>,
}

pub fn make_input(field_path:&str, spectrum_dir:&str)-> Input{
    println!("Reading inputs: \n\
     Field -> {} \n\
     Spectra -> {}", field_path,spectrum_dir);
    let arc_seconds_per_deg = 60.*60.;
    let sky_coords = CoordinateSystem::new(
        [1./arc_seconds_per_deg, 0.],
        [0.,1./arc_seconds_per_deg],
        [-0.56, -0.06],
    );

    let (x_list,y_list, weight_list,spec_ref_list) = parse_field_file(field_path);
    let mut counter = 0;
    let mut spectra = Vec::new();
    for i in 0..*spec_ref_list.iter().max().unwrap() + 1 {
        let path = format!("{spectrum_dir}/spectra_{i}.dat");
        //println!("{:?}",path);

        counter += 1;
        let spectrum = Spectrum::load(&path, 0.0001);
        spectra.push(spectrum.unwrap());
    }

    let points:Vec<Point> = x_list.iter().zip(y_list).map(|(&x, y)|{
        Point::new(x,y,&sky_coords)
    }).collect();

    Input{
        points,
        weights: weight_list,
        spec_refs: spec_ref_list,
        spectra,
    }


}

pub fn parse_field_file(path:&str)-> (Vec<f64>,Vec<f64>, Vec<f64>, Vec<usize>){

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut parsed_header = false;
    let mut lines_read = 0;
    let mut comments_ignored = 0;
    let mut x_list = Vec::new();
    let mut y_list = Vec::new();
    let mut spec_ref_list = Vec::new();
    let mut weight_list = Vec::new();



    //TODO deal with 0 line files/uninitialization
    for line in reader.lines(){

        let line = line.unwrap();
        if line.chars().nth(0).unwrap() =='#' {
            comments_ignored += 1;
            continue
        }
        let line = line.trim().split("|").map(|s|s.trim()).collect::<Vec<&str>>();
        if line.len() != 4{
            panic!()
        }
        if !parsed_header{
            // println!("header: {:?}",line);
            assert_eq!( line,["x", "y", "weight", "ref"]);
            parsed_header = true;
            continue
        }

        if parsed_header{
            let x = line[0].parse::<f64>().unwrap();
            let y = line[1].parse::<f64>().unwrap();
            let weight = line[2].parse::<f64>().unwrap();
            let spec_ref = line[3].parse::<f64>().unwrap() as usize;
            x_list.push(x);
            y_list.push(y);
            weight_list.push(weight);
            spec_ref_list.push(spec_ref);
            lines_read += 1;

            continue
        }
        else{unreachable!()}
    };
    assert!(lines_read > 1);
    (x_list,y_list, weight_list,spec_ref_list)
}


