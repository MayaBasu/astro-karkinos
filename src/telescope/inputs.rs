use rand_distr::num_traits::real::Real;
use crate::geometry::*;
use crate::geometry::FloatError::ValueNotRegular;
use crate::gridded_data::{SpectralUnits, Unit, DATA1D};
use crate::gridded_data::SpectralUnits::*;
use crate::telescope::effects::{SpatialEffect, SpectralResponse};

pub const kB_CGS:f64 = 1.380649 *10e-16; //erg K−1
pub const h_CGS:f64 = 6.626069 *10e-27; //erg s
pub const c_CGS:f64 = 2.997925 *10e10; // cm s−1


pub struct Spectrum{
    data: Vec<f64>,
    grid: GRID1D,
    units: SpectralUnits
}
/*
Spectrum assumptions:
1. One data point per grid point
2. TODO: Maybe check for non negativity?
3. Data values are Regular
 */
impl DATA1D<SpectralUnits> for Spectrum{
    fn grid(&self) -> &GRID1D {
        &self.grid
    }
    fn data(&self) -> &Vec<f64> {
        &self.data
    }
    fn unit(&self) -> &SpectralUnits {
        &self.units
    }
}
impl Spectrum{ //https://vitaly.neustroev.net/useful-info/conversions/
    pub fn new(grid:GRID1D,data:Vec<f64>,units:SpectralUnits)->Spectrum{
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
        spectral_response.regrid(self.grid);
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
    pub fn to_band(self, band_pass: &mut SpectralResponse)->f64{
        self.apply_spectral_response(band_pass).integrate()
    }

}


pub enum SkyObject{
    PointSource(Point, Spectrum),
    Patch(SpatialEffect,Spectrum),
    Background(SpatialEffect,Spectrum),
}



pub trait Input{
    fn spectrum()-> Spectrum;
    fn 
    fn scale() -> f64;
    fn scale_by(&mut self, factor:f64);
    fn find_band(&self, band_pass: SpectralResponse) -> f64;
}

impl Input for SkyObject::PointSource{
    fn spectrum() -> Spectrum {
        todo!()
    }

    fn scale() -> f64 {
        todo!()
    }

    fn scale_by(&mut self, factor: f64) {
        todo!()
    }

    fn find_band(&self, band_pass: SpectralResponse) -> f64 {
        todo!()
    }
}