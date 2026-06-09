#[derive(Debug)]
#[derive(Clone)]
pub enum Units{
    Flux, //flux in photons per cm^2 per second per Angstrom
    AB_MAG,
    Photons,
    Electrons,
}


impl Units{
    pub fn convert(&self, end_unit:Units){
        match self {
            Units::Flux => {}
            Units::AB_MAG => {}
            Units::Photons => {}
            Units::Electrons => {}
        }
    }
}