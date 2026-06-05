use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::Instant;
use plotpy::{Curve, Plot};
use astroimsim_geometry::grid1d::{Location1D, Neighbors, GRID1D};




impl FrequencyFile{
    pub fn new_empty(path: &'static str,
                     step_size: f64, //TODO have units for this length
                     minimum_value: f64,
                     maximum_value: f64,
                     snap_precision: f64,
                     scale:f64) -> FrequencyFile{
        let data:Vec<(usize,Vec<f32>)> = Vec::new();
        FrequencyFile{
            path,
            grid1d: GRID1D::new_empty(step_size,minimum_value,maximum_value,snap_precision,scale),
            data,
        }
    }

    pub const fn new_from_grid(path:&'static str, grid:GRID1D)->  FrequencyFile{
        let data:Vec<(usize,Vec<f32>)> = Vec::new();
        FrequencyFile{
            path,
            grid1d: grid,
            data,
        }
    }


    

    pub fn get_data(&self, index:usize)-> Vec<f32>{
        assert_eq!(self.data[index].0,index);
        self.data[index].1.clone()
    }


    pub fn re_grid(&mut self, new_grid:GRID1D)-> FrequencyFile {
        assert!(new_grid.snap_precision <= self.grid1d.snap_precision, "Snap precision of new grid must be less than or equal to that of the original grid");
        self.data.sort_by_key(|x| x.0); //TODO move this into a validation function
        let mut new_data = Vec::new();
        for point in 0..new_grid.num() {

            let new_location = new_grid.location(point);

            let value = match self.grid1d.inside_or_outside(new_location) {
                Location1D::TooHigh => {println!("new gridding {:?},location is {:?}, too high",point,new_location);
                    let mut datum = self.data[self.data.len()-1].1.clone() ;
                    datum[0] = new_location as f32;
                    datum}
                Location1D::TooLow => { println!("new gridding {:?},location is {:?}, too low",point,new_location);
                    let mut datum = self.data[0].1.clone();
                    datum[0] = new_location as f32;
                    datum}
                Location1D::JustRight => {
                    match self.grid1d.find_neighbors(new_location) {
                        Neighbors::Two(lower_index, upper_index) => {
                            println!("new gridding {:?},location is {:?}, just right, two neiborhs: {:?}",point,new_location,(lower_index, upper_index));
                            let lower = self.grid1d.location(lower_index);
                            let upper = self.grid1d.location(upper_index);
                            let lower_delta = new_location - lower;
                            let upper_delta = upper - new_location;
                            let lower_weight = lower_delta / (lower_delta + upper_delta);
                            let upper_weight = upper_delta / (lower_delta + upper_delta);
                            let upper_data = self.get_data(upper_index);
                            let lower_data = self.get_data(lower_index);
                            upper_data.iter().zip(lower_data.iter()).map(|(a, b)|
                                a * upper_weight as f32 + b * lower_weight as f32).collect()
                        }
                        Neighbors::One(snap) => { self.get_data(snap) }
                    }
                }
            };
            new_data.push((point, value))
        }
        let mut new_frequency_file = (*self).clone();
        new_frequency_file.data = new_data;
        new_frequency_file
    }

    pub fn plot(&self) -> Vec<Vec<Vec<f64>>>{
        let mut curves = Vec::new();
        for i in 1..self.data[0].1.len() {
            let mut curve = Vec::new();

            /*
            curve.set_line_style(line)

            .set_marker_line_width(2.5)
            .set_marker_size(4.0)
                .set_marker_color(color)
                .set_line_color(color)
            .set_marker_style(".");

             */

            // curve.points_begin();
            for point in self.data.clone() {
                // println!("{:?}", point);
                println!("adding {:?}",(point.1[0],point.1[i]));
                curve.push(vec![point.1[0] as f64, point.1[i] as f64]);
            }
            //curve.points_end();
            curves.push(curve);
        }
        println!("{:?}",self.data);
        curves
    }

}



