#[derive(Debug)]
#[derive(Clone)]
pub enum Units{
    Flux, //flux in photons per cm^2 per second per Angstrom
    AB_MAG,
    Photons,
    Electrons,
}