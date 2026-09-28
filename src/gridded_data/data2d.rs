use std::fs::File;
use std::io::Write;
use crate::geometry::*;
use uvex_fitrs::{Fits, FitsData, FitsDataArray};



pub trait DATA2D<Unit:super::units::Unit>{

    fn grid(&self)-> &GRID2D;
    fn data(&self)-> &Vec<Vec<f64>>;
    fn unit(&self)-> &Unit;
    fn is_data_regular(data:&Vec<Vec<f64>>)->bool{
        match data.iter().flatten().find(|&x|{
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
        let [x,y] = self.grid().xy_indices(index);
        self.data()[y][x]
    }
    fn value(&self,point:&Point)->f64{
        let interpolation_data = self.grid().projected_interpolation_coefficients(point);

        let corners = match interpolation_data.corners{
            Corners::Four(one, two, three, four) => {
                vec![one,two,three,four]}
            Corners::Two(one,two) => {vec![one,two]}
            Corners::One(one) => {vec![one]}
        };
        let sum:f64 = corners
            .into_iter().
            zip(interpolation_data.coefficients)
            .map(|(point,coefficient)|{
                (self.value_at_index(point)*coefficient).value()
            }).sum();
        (Regular::try_from(sum)/interpolation_data.normalization).value()
    }

    fn values(&self, new_grid: GRID2D)-> Vec<Vec<f64>>{
        let points = new_grid.locate_grid_points()
            .iter()
            .flatten()
            .map(|point|self.value(point))
            .collect::<Vec<f64>>();

        points.chunks(new_grid.x_num()).map(|v|v.to_vec())
            .collect()

    }


   

}


   
  



    









