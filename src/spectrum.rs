use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use astroimsim_geometry::grid1d::GRID1D;
use plotpy::{Curve, Plot};
use crate::units::{DataTypes, Electrons, Photons, Response, SpectralDensity};

pub struct Spectrum{
    pub grid1d: GRID1D,
    pub data: Vec<(usize, f64)>, //a vector of values (point number on grid, data value at that point)
    pub units: SpectrumUnits,
    pub label: &'static str,
    pub dat_path: &'static str, //path to .dat file with data
}


impl Spectrum{ //https://vitaly.neustroev.net/useful-info/conversions/
    fn convert_to_cgs(&mut self)  {
        let mut cgs_data = Vec::with_capacity(self.data.len());
        for (point,value) in &self.data{
            let lambda = self.grid1d.location(*point);
            let cgs_value = match self.units {
                SpectrumUnits::F_nu => {value}
                SpectrumUnits::AbMagnitude => &{
                    10f64.powf((value + 48.6) / (-2.5)) },
                SpectrumUnits::Janskys => &{
                    (10f64).powi(-23)*value },
                SpectrumUnits::f_lambda => &{
                    6.63*10e-27*value*lambda},
                SpectrumUnits::F_lambda=>&{
                    3.34*10e-19*lambda*lambda*value}
            };
            cgs_data.push((*point,*cgs_value))
        }
        self.data = cgs_data;
        self.units = SpectrumUnits::F_nu
    }
    pub fn convert_to(&mut self,unit:&SpectrumUnits) {
        self.convert_to_cgs();
        let mut converted_values = Vec::with_capacity(self.data.len());
        for (point, value) in &self.data {
            let lambda = self.grid1d.location(*point);
            let converted_value  = match self.units {
                SpectrumUnits::F_nu => {value},
                SpectrumUnits::F_lambda => &{3.00 * 10e18 * value / (lambda.powi(2))},
                SpectrumUnits::f_lambda => &{1.51 * 10f64.powi(26) * value / lambda},
                SpectrumUnits::AbMagnitude => &{-2.5 * value.log10() - 48.6},
                SpectrumUnits::Janskys => &{(10f64).powi(23) * value},
            };
            converted_values.push((*point,*converted_value));
        }
        self.data = converted_values;
    }

    pub fn flat_AB(ab_mag:f64,grid:GRID1D)-> Spectrum{

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