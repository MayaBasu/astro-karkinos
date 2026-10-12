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

    pub fn add(&mut self,effect:&SpatialEffect){
        for grid_point in 0..effect.grid().num_points(){
            let location = effect.grid().locate(grid_point);
            if self.grid.is_point_inside(&location){
                let value = effect.value(&location);
                let (x_mod,y_mod,x_res,y_res) = self.grid.fit_grid(&location,true).unwrap();
                self.data[y_mod][x_mod] += value;
            }

        }
    }


    pub(crate) fn combine_2_add(&mut self, effect:&SpatialEffect) {
        let mut points_inside = 0;
        let mut nonzero = 0;
        for y in 0..self.grid().y_num(){
            for x in 0..self.grid().x_num(){
                let grid_number = self.grid().grid_number(x,y);
                let point = self.grid().locate(grid_number);
                let value = if effect.grid().is_point_inside_cells(&point){

                    points_inside += 1;
                    effect.value(&point)

                }else{
                    0.0
                };

                if value != 0.0{
                    nonzero += 1;
                }

                self.data[y][x] += value;



            }
        }
        println!("{:?} points inside {:?}", points_inside,nonzero);

    }

}









