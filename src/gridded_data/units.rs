use std::fmt::{Debug, Display, Formatter};

pub trait Unit:Display + Copy + Clone + Debug + PartialEq{
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SpectrumUnits{
    F_nu, //ergs per cm^2 per s^1 per Hz
    F_lambda, //ergs per cm^2 per s per angstrom
    f_lambda, //Photons per cm^2 per second per angstrom
    AbMagnitude, //-2.5*log10(f_nu ) - 48.6 in CGS
    Janskys, // ??? who knows
}
impl Display for SpectrumUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SpectrumUnits::F_nu => { write!(f, "F_nu") }
            SpectrumUnits::F_lambda => { write!(f, "F_lambda") }
            SpectrumUnits::f_lambda => { write!(f, "f_lambda") }
            SpectrumUnits::AbMagnitude => { write!(f, "AbMag") }
            SpectrumUnits::Janskys => { write!(f, "Janskys") }
        }
    }
}








impl SpectrumUnits {
    pub fn convert_to_cgs(&mut self, value:f64)->f64  {
        match self{
            SpectrumUnits::F_nu => { value }
            SpectrumUnits::AbMagnitude => {
                10f64.powf((value + 48.6) / (-2.5))
            },
            SpectrumUnits::Janskys => {
                (10f64).powi(-23) * value
            },
            SpectrumUnits::f_lambda => {
                6.63e-27 * value * lambda
            },
            SpectrumUnits::F_lambda => {
                3.34e-19 * lambda * lambda * value
            }
        }
    }
}


impl Unit for SpectrumUnits{

    fn convert_value(&self, new_unit: &Self, value: f64) -> f64 {
        todo!()
    }
}
