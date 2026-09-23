use crate::geometry::GRID1D;
use crate::gridded_data::DATA1D;
use crate::gridded_data::units::SpectrumUnits;
pub struct SpectralEffect{
    label: String,
    data: DATA1D,
}

impl SpectralEffect{
    pub fn new(data:Vec<f64>, grid:GRID1D, label:String){
        let data = DATA1D::new(grid, data, SpectrumUnits::Fractional);
    }
}