use std::fmt::{Debug, Display, Formatter};
use crate::gridded_data::units;

pub trait Unit:Display + Copy + Clone + Debug + PartialEq{
    fn from_str(str:&str)->Result<Self,UnitError>;
}

pub enum UnitError{
    ParsingError(String),

}



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitlessUnit {
    NormalizedFraction,
    Percent
}

impl Unit for UnitlessUnit{
    fn from_str(str: &str) -> Result<Self, UnitError> {
        match str {
            "fraction" => Ok(Self::NormalizedFraction),
            "percent" => Ok(Self::Percent),
            s => Err(UnitError::ParsingError(format!("Failed to parse {:?} as UnitlessUnit, options are \
            'fraction', 'percent'",s)))
        }
    }
}

impl Display for UnitlessUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"fractopm")
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SpectralUnits {
    F_nu, //ergs per cm^2 per s^1 per Hz
    F_lambda, //ergs per cm^2 per s per angstrom
    f_lambda, //Photons per cm^2 per second per angstrom
    AbMagnitude, //-2.5*log10(f_nu ) - 48.6 in CGS
    Janskys,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DetectorUnits {
    Electrons,
    AverageElectrons
}

impl Unit for DetectorUnits{
    fn from_str(str: &str) -> Result<Self, UnitError> {
        match str {
            "electrons" => Ok(Self::Electrons),
            "average_electrons" => Ok(Self::AverageElectrons),
            s => Err(UnitError::ParsingError(format!("Failed to parse {:?} as UnitlessUnit, options are \
            'electrons', 'average_electrons'",s)))
        }
    }
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

impl Unit for SpectralUnits{
    fn from_str(str: &str) -> Result<Self, UnitError> {
        match str{
            "F_nu" => Ok(Self::F_nu),
            "f_lambda" => Ok(Self::f_lambda),
            "F_lambda" =>Ok(Self::F_lambda),
            "AbMagnitude" =>Ok(Self::AbMagnitude),
            "Janskys" => Ok(Self::Janskys),
            s => Err(UnitError::ParsingError(format!("Failed to parse {:?} as SpectralUnit, options are \
            'F_nu', 'f_lambda', 'F_lambda', 'AbMagnitude', 'Janskys'",s)))
        }
    }
}

