use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crate::geometry::*;
use crate::gridded_data::units::Unit;


pub trait DATA1D<Unit:super::units::Unit>{
    fn grid(&self)-> &GRID1D;
    fn data(&self)-> &Vec<f64>;
    fn unit(&self)-> &Unit;
    fn is_data_regular(data:&Vec<f64>)->bool{
        match data.iter().find(|&x|{
            !Regular::check(*x)
        }){
            Some(_) => {false}
            _ => {true}
        }
    }
    fn value_at_index(&self, index:usize) -> f64{
        assert!(index<=self.grid().num_points(),
                "{}", format!("Data grid only has {:?} points, attempted to access point {:?}",
                        self.grid().num_points(),
                        index));
        self.data()[index]
    }
    fn value(&self, location: f64, unit:Grid1DUnits) -> f64{
        let data = self.data();
        match self.grid().locate(location,unit){
            Location::Higher => {data[data.len() - 1]}
            Location::Lower => {data[0]}
            Location::Snapped(index) => {data[index]}
            Location::Between(lower, upper, lower_weight) => {
                let lower = data[lower];
                let upper = data[upper];
                let upper_weight = Regular::try_from(1.0-lower_weight);
                (upper*upper_weight+Regular::try_from(lower*lower_weight)).value()
            }
        }
    }
    fn values(&self, new_grid: &GRID1D) ->Vec<f64> {
        assert!(new_grid.snap_precision <= self.grid().snap_precision, "Snap precision of new grid must be less than or equal to that of the original grid");
        //TODO: why did I put this assert! in?
        new_grid.locate_grid_points().iter().map(|x|{
            self.value(*x,new_grid.unit())
        }).collect()
    }
    fn write_to_dat(&mut self, header:&str, path:&str){
        let mut file = File::create(path).expect("Could not create file");
        file.write_all(header.as_bytes()).expect("Failed to write header");
        file.write_all(format!("| \n{:?} | {:?} |",self.grid().unit(), self.unit()).as_bytes())
            .expect("Failed to write units to file");
        for (point,value) in (0..self.grid().num_points()).zip(self.data()){
            let location = self.grid().locate_grid_point(point);
            file.write_all(format!("\n{:?} | {:?}", location, value).as_bytes())
                .expect("Failed to write data");
        }
    }
}

