use std::fmt::{Debug, Display, Formatter};

pub trait Unit:Display + Copy + Clone + Debug + PartialEq{
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitlessUnit {
    NormalizedFraction
}

impl Unit for UnitlessUnit{

}

impl Display for UnitlessUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Fractional")
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SpectralUnits {
    F_nu, //ergs per cm^2 per s^1 per Hz
    F_lambda, //ergs per cm^2 per s per angstrom
    f_lambda, //Photons per cm^2 per second per angstrom
    AbMagnitude, //-2.5*log10(f_nu ) - 48.6 in CGS
    Janskys, // ??? who knows
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DetectorUnits {
    Electrons,
    AverageElectrons
}

impl Unit for DetectorUnits{
    
}

impl Display for DetectorUnits{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self{
            DetectorUnits::Electrons => {write!(f, "Electrons")}
            DetectorUnits::AverageElectrons => {write!(f, "Average Electrons")}
        }
    }
}
impl Display for SpectralUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            SpectralUnits::F_nu => { write!(f, "F_nu") }
            SpectralUnits::F_lambda => { write!(f, "F_lambda") }
            SpectralUnits::f_lambda => { write!(f, "f_lambda") }
            SpectralUnits::AbMagnitude => { write!(f, "AbMag") }
            SpectralUnits::Janskys => { write!(f, "Janskys") }
        }
    }
}

impl Unit for SpectralUnits{}

