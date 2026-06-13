
pub const kB_CGS:f64 = 1.380649 *10e-16; //erg K−1
pub const h_CGS:f64 = 6.626069 *10e-27; //erg s
pub const c_CGS:f64 = 2.997925 *10e10; // cm s−1

#[derive(Debug,Clone)]
#[derive(PartialEq)]
pub enum SpectrumUnits{
    F_nu, //ergs per cm^2 per s^1 per Hz
    F_lambda,//ergs per cm^2 per s per angstrom
    f_lambda, //Photons per cm^2 per second per angstrom
    AbMagnitude, //-2.5*log10(f_nu ) - 48.6 in CGS
    Janskys, // ??? who knows
}

#[derive(Debug,Clone)]
pub struct SpectralDensityData {
    pub values:Vec<f64>,
    pub units: SpectrumUnits,
}

pub enum Data{
    SpectralDensity,
    PhotonCount,
    ElectronCount,
}



impl SpectralDensityData { //https://vitaly.neustroev.net/useful-info/conversions/
    pub fn to_cgs(&self,lambda:f64) -> SpectralDensityData {
        let mut cgs_values = Vec::with_capacity(self.values.len());
        for value in &self.values{
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
            cgs_values.push(*cgs_value)
        }
        SpectralDensityData {values:cgs_values,units:SpectrumUnits::F_nu}
    }
    pub fn convert_to(&self,unit:&SpectrumUnits,lambda:f64)-> SpectralDensityData {
        let f_nu = self.to_cgs(lambda);
        let f_nu_values = match f_nu.units {
            SpectrumUnits::F_nu => { f_nu.values}
            _ => {panic!("Unreachable")}
        };
        let mut converted_values = Vec::with_capacity(self.values.len());
        for f_nu_value in f_nu_values {
            let converted_value = match unit {
                SpectrumUnits::F_nu => { f_nu_value }
                SpectrumUnits::AbMagnitude => {
                    -2.5 * f_nu_value.log10() - 48.6
                }
                SpectrumUnits::Janskys => {
                    (10f64).powi(23) * f_nu_value
                }
                SpectrumUnits::F_lambda => {
                    3.00 * 10e18 * f_nu_value / (lambda.powi(2))
                }
                SpectrumUnits::f_lambda => {
                    1.51 * 10f64.powi(26) * f_nu_value / lambda
                }
            };
            converted_values.push(converted_value);
        }
        SpectralDensityData {values:converted_values,units:unit.clone()}
    }
}


