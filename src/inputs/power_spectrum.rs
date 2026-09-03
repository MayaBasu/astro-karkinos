use std::fs::File;
use std::io::{BufReader, Write};
use crate::telescope_components::spectral_response::*;
use crate::geometry::*;


pub const kB_CGS:f64 = 1.380649 *10e-16; //erg K−1
pub const h_CGS:f64 = 6.626069 *10e-27; //erg s
pub const c_CGS:f64 = 2.997925 *10e10; // cm s−1
//could have this as enum without struct and then have the units be in the parenthesis instead of the struct

#[derive(Debug,Clone)]
pub struct PowerSpectrum {
    pub grid1d: GRID1D,
    pub data: Vec<(usize, f64)>, //a vector of values (point number on grid, data value at that point)
    pub units: SpectrumUnits,
    pub label: String,
   // pub dat_path: &'static str, //path to .dat file with data
}




impl PowerSpectrum { //https://vitaly.neustroev.net/useful-info/conversions/
    
    pub fn from_data(grid1d: GRID1D, data:Vec<f64>, units:SpectrumUnits, label:String)-> PowerSpectrum{
        assert_eq!(grid1d.num(), data.len());
        let data:Vec<(usize,f64)> = data.iter().enumerate()
            .map(|(i,d)|{(i,*d)}).collect();
        PowerSpectrum{
            grid1d,
            data,
            units,
            label,
        }
        
        
    }

    pub fn write_to_dat(&mut self,path:&str,header:&str){
        let mut file = File::create(path).expect("Could not create file");
        file.write_all(format!(
            "{:?} \n {:?} \n {:?} \n {:?} \n \n \n",
            self.label, self.units, self.grid1d, header).as_bytes()).expect("Failed to write header");

        for (point,value) in &self.data{
            let location = self.grid1d.location(*point);
            file.write_all(format!("\n{:?}   {:?}", location, value).as_bytes()).unwrap();
        }

    }
    pub fn convert_to_cgs(&mut self)  {
      //  println!("Converting {:?} to CGS",self.sum());

        let mut cgs_data = Vec::with_capacity(self.data.len());
        for (point,value) in &self.data{
            let lambda = self.grid1d.location(*point);
           // println!("lambda is {:?}",lambda);
            let cgs_value = match self.units {
                SpectrumUnits::F_nu => {value}
                SpectrumUnits::AbMagnitude => &{
                    10f64.powf((value + 48.6) / (-2.5)) },
                SpectrumUnits::Janskys => &{
                    (10f64).powi(-23)*value },
                SpectrumUnits::f_lambda => &{
                    6.63e-27*value*lambda},
                SpectrumUnits::F_lambda=>&{
                    3.34e-19*lambda*lambda*value}
            };
            cgs_data.push((*point,*cgs_value))
        }
        self.data = cgs_data;
        self.units = SpectrumUnits::F_nu;
      //  self.sum();
    }
    pub fn convert_to(&mut self,unit:&SpectrumUnits) {
        self.convert_to_cgs();
     //   println!("Converting to else from ");
        //self.sum();
        let mut converted_values = Vec::with_capacity(self.data.len());
        for (point, value) in &self.data {
            let lambda = self.grid1d.location(*point);
            //println!("lambda is {:?}",lambda);
            let converted_value  = match unit {
                SpectrumUnits::F_nu => {value},
                SpectrumUnits::F_lambda => &{3.00e18 * value / (lambda.powi(2))},
                SpectrumUnits::f_lambda => &{1.51e26 * value / lambda},
                SpectrumUnits::AbMagnitude => &{-2.5 * value.log10() - 48.6},
                SpectrumUnits::Janskys => &{(10f64).powi(23) * value},
            };
            converted_values.push((*point,*converted_value));
        }
        self.data = converted_values;
        self.units = unit.clone();
       // self.sum();
    }

    pub fn flat_AB(ab_mag:f64,grid1d:GRID1D,label:String)-> PowerSpectrum {
        let units = SpectrumUnits::AbMagnitude;
        let mut data = Vec::with_capacity(grid1d.num());
        for point in 0..grid1d.num(){
            data.push((point,ab_mag))
        }
        PowerSpectrum{
            grid1d,
            data,
            units,
            label,
        }
    }

    pub fn flat(f_lambda:f64,grid1d:GRID1D,label:String)-> PowerSpectrum {
        let units = SpectrumUnits::f_lambda;
        let mut data = Vec::with_capacity(grid1d.num());
        for point in 0..grid1d.num(){
            data.push((point,f_lambda))
        }
        PowerSpectrum{
            grid1d,
            data,
            units,
            label,
        }
    }

    pub fn black_body(temp_kelvin:f64, grid1d:GRID1D, label:String)-> PowerSpectrum{
        let units = SpectrumUnits::F_lambda;
        let mut data = Vec::with_capacity(grid1d.num());
        for point in 0..grid1d.num(){
            let wavelength = grid1d.location(point);
            let scale_factor = 2.0*std::f64::consts::PI*h_CGS*c_CGS.powi(2)/(wavelength.powi(5));
            let exponent = h_CGS*c_CGS/(wavelength*kB_CGS*temp_kelvin);
            let F_lambda = scale_factor*(1.0/(exponent.exp()-1.0));
            data.push((point,F_lambda))
        }
        PowerSpectrum{
            grid1d,
            data,
            units,
            label,
        }
    }
    
    pub fn apply_spectral_response(&mut self, mut spectral_response_curve: &SpectralResponseCurve){
       let mut spectral_response_curve = spectral_response_curve.clone();
           spectral_response_curve.re_grid(&self.grid1d);
        //TODO assert_eq!(spectral_response_curve.grid1d,self.grid1d);
        //TODO make this more efficient
        let mut new_values = Vec::with_capacity(self.data.len());
        for ((self_point, self_value),(response_point,response_value)) in self.data.clone().into_iter().zip(spectral_response_curve.data.clone()){
            assert_eq!(self_point,response_point);
          //  println!("{:?}",self_point);
          //  println!("{:?} {:?} {:?}",self_value,response_value,self_value*response_value );
            new_values.push((self_point,self_value*response_value))
        }
        self.data = new_values;
        
    }
    pub fn total_average_photon_flux(&mut self,area:f64)->f64{
        //self.convert_to(&SpectrumUnits::f_lambda);
        let sum:f64 = self.data.iter()
            .map(|(_point_num,value)|value).sum();
        sum*area
         //TODO NUM OR NUM + !??
    }

    pub fn sum(&self)-> f64{
        let sum = self.data.iter().map(|(i,v)| v).sum();
     //   println!("SUM is {:?}, units are {:?}",sum, self.units);
        //println!("{:?}",self.data);
        sum
    }



}



#[derive(Debug, Clone, PartialEq)]
pub enum SpectrumUnits {
    F_nu, //ergs per cm^2 per s^1 per Hz
    F_lambda,//ergs per cm^2 per s per angstrom
    f_lambda, //Photons per cm^2 per second per angstrom
    AbMagnitude, //-2.5*log10(f_nu ) - 48.6 in CGS
    Janskys, // ??? who knows
}


#[derive(Clone, Debug)]
pub struct Bands {
    pub fuv:f64,
    pub nuv:f64,
}

impl Bands  {
    pub fn new(fuv:f64,nuv:f64)->Bands{
        Bands{fuv,nuv}
    }
}