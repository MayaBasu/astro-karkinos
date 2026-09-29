use crate::geometry::*;
use crate::gridded_data::{DetectorUnits, SpectralUnits, DATA2D};
use crate::telescope::effects::SpatialEffect;


pub struct Detector {
    pub grid: GRID2D,
    pub data: Vec<Vec<f64>>,
    pub unit: DetectorUnits
}

impl DATA2D<DetectorUnits> for Detector{
    fn grid(&self) -> &GRID2D {
        &self.grid
    }
    fn data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }

    fn mutable_data(&mut self) -> &mut Vec<Vec<f64>> {
        &mut self.data
    }

    fn unit(&self) -> &DetectorUnits {
        &self.unit
    }
}



impl Detector {
    //TODO: combine initialize with first pass?
    pub fn initialize(grid: GRID2D, unit: DetectorUnits) -> Detector{
        let data:Vec<Vec<f64>> = (0..grid.y_num()).map(|_| {
            (0..grid.x_num()).map(|_|{
                0.0
            }).collect()
        }).collect();
        Detector{
            grid,
            data,
            unit,
        }
    }

}









