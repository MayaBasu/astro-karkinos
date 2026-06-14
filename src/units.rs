use rand_distr::{Poisson, Distribution, Binomial};

pub const kB_CGS:f64 = 1.380649 *10e-16; //erg K−1
pub const h_CGS:f64 = 6.626069 *10e-27; //erg s
pub const c_CGS:f64 = 2.997925 *10e10; // cm s−1
//could have this as enum without struct and then have the units be in the parenthesis instead of the struct

#[derive(Debug,Clone,PartialEq)]
pub enum DataTypes {
    SpectralDensity(SpectralDensity),
    Photons(Photons),
    Electrons(Electrons),
    Response(Response),
}
#[derive(Debug,Clone,PartialEq)]
pub struct SpectralDensity{
    pub values: Vec<f64>,
    pub units: SpectrumUnits
}
#[derive(Debug,Clone,PartialEq)]
pub struct Photons{
    pub values: Vec<usize>
}
#[derive(Debug,Clone,PartialEq)]
pub struct Electrons{
    pub values: Vec<usize>
}
#[derive(Debug,Clone,PartialEq)]
pub struct Response{
    pub values:Vec<f64>
}






impl SpectralDensity { //https://vitaly.neustroev.net/useful-info/conversions/
    pub fn to_cgs(&self,lambda:f64) -> SpectralDensity {
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
        SpectralDensity{values:cgs_values,units:SpectrumUnits::F_nu}
    }
    pub fn convert_to(&self,unit:&SpectrumUnits,lambda:f64)-> SpectralDensity {
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
        SpectralDensity {values:converted_values,units:unit.clone()}
    }

    pub fn photonify(&self,lambda:f64,duration:f64)-> Photons{
        //simulate getting a random sample of a poisson distribution of photons
        //during a duration specified in seconds
        let photon_rates = self.convert_to(&SpectrumUnits::f_lambda,lambda);
        let photon_rates = match photon_rates.units{
            SpectrumUnits::f_lambda => {photon_rates.values}
            _ =>{panic!("Unreachable")}
        };
        let mut photonified = Vec::with_capacity(self.values.len());
        for photon_rate in photon_rates{
            let average_photons  = photon_rate*duration;
            let poisson_distribution = Poisson::new(average_photons).unwrap();
            let photons: usize = poisson_distribution.sample(&mut rand::rng()) as usize;
            photonified.push(photons)
        }
        Photons{
            values:photonified,
        }
    }
}


impl Photons{
    pub fn electronify(&self,qe:f64)-> Electrons{
        let mut electron_numbers = Vec::with_capacity(self.values.len());
        for photon_number in &self.values{
            let binomial = Binomial::new(*photon_number as u64, qe).unwrap();
            electron_numbers.push(binomial.sample(&mut rand::rng()) as usize);
        }
        Electrons{
            values:electron_numbers
        }
    }
}


