use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use astroimsim_geometry::grid1d::{Location1D, Neighbors, GRID1D};
use astroimsim_geometry::grid2d::GRID2D;
use eframe::wgpu::naga::Sampling::Either;
use plotpy::{Curve, Plot};
use crate::dat_file_reader::FrequencyFile;

pub struct DATAGRID1D{
    pub grid1d: GRID1D,
    pub data: Vec<(usize,Vec<f64>)>, //a vector of values (point number on grid, data values at that point)
    pub data_shape: (usize,usize), //shape of data at each grid point
    pub label: &'static str,
}

pub struct DATAGRID2D{
    pub grid1d: GRID2D,
    pub data: Vec<(usize,Vec<f64>)>, //a vector of values (point number on grid, data values at that point)
    pub data_shape: (usize,usize), //shape of data at each point
    pub label: &'static str,
}



impl DATAGRID1D {
    pub fn get_data(&self, index: usize) -> Vec<f64> {
        assert_eq!(self.data[index].0, index);
        self.data[index].1.clone()
    }

    pub fn load_dat_file(mut self, path: &str, delineator: &str,plot: bool) {
        assert_ne!(0, self.data.len(), "Loading data {:?} into would overwrite current data", self.label);
        println!("Loading {:?} into {:?}", path, self.label);
        let start = Instant::now();
        let file = File::open(path.clone()).expect("Failed to open file");
        let reader = BufReader::new(file);
        let data:Vec<Vec<f64>>  =  reader.lines().map(|line|{
            let line = line.expect("Failed to read line");
            let mut parsable = true;
            let parsed: Vec<f64> = line
                .trim()
                .split(delineator)
                .trim()
                .parse::<f64>()
                .partition_map(|result|
                match result{
                    Ok(value) => value,
                    Err(_) => parsable = false,
                }).collect();
            if parsable{ //check there were no errors in parsing
                if parsed.len() == self.data_shape.0*self.data_shape.1 + 1{
                    parsed //add to list if there is the expected number of data points
                }
            }
        }).collect();
        println!("Parsed {:?} lines in {:?}", data.len(),start.elapsed().as_millis());
        assert_eq!(data.len(), self.grid1d.num(), "Retrieved a different number of records than expected");


        let mut same_num_data_points_per_record = true;
            for i in 0..data.len() - 1 {
                if data[i].len() - data[i + 1].len() != 0 {
                    same_num_data_points_per_record = false;
                }
            }
            if !same_num_data_points_per_record {
                println!("Warning! There are different numbers of records for each line. This may mess up plotting or indicate a loading error. Will be plotting with {:?}", data[0].len() - 1)
            } //TODO run this function witha  "verbose" to list out the differences
            if plot {
                let mut plot = Plot::new();
                for i in 1..data[0].len(){
                    let mut curve = Curve::new();
                    curve.set_line_width(2.0);
                    curve.points_begin();
                    for point in &data.clone() {
                        curve.points_add(point[0], point[i]);
                    }
                    curve.points_end();
                    plot.add(&curve).grid_and_labels("x", "y");
                }
                plot.show("ksenf").expect("hHHHHHH");
            }
            println!("The first record is {:?} and the last is {:?}, snapping to grid: {:?}", data[0], data[data.len() - 1], self.grid1d);
            let mut snapped_data = Vec::new();
            for datum in data {
                let location = datum[0];
                let index = self.grid1d.snap(location as f64);
                snapped_data.push((index, datum)) //TODO this must change to plot multiple
            }
            self.data = snapped_data;
        }
    }
    pub fn re_grid(&mut self, new_grid: GRID1D) -> FrequencyFile {
        assert!(new_grid.snap_precision <= self.grid1d.snap_precision, "Snap precision of new grid must be less than or equal to that of the original grid");
        self.data.sort_by_key(|x| x.0); //TODO move this into a validation function
        let mut new_data = Vec::new();
        for point in 0..new_grid.num() {
            let new_location = new_grid.location(point);

            let value = match self.grid1d.inside_or_outside(new_location) {
                Location1D::TooHigh => {
                    println!("new gridding {:?},location is {:?}, too high", point, new_location);
                    let mut datum = self.data[self.data.len() - 1].1.clone();
                    datum[0] = new_location as f32;
                    datum
                }
                Location1D::TooLow => {
                    println!("new gridding {:?},location is {:?}, too low", point, new_location);
                    let mut datum = self.data[0].1.clone();
                    datum[0] = new_location as f32;
                    datum
                }
                Location1D::JustRight => {
                    match self.grid1d.find_neighbors(new_location) {
                        Neighbors::Two(lower_index, upper_index) => {
                            println!("new gridding {:?},location is {:?}, just right, two neiborhs: {:?}", point, new_location, (lower_index, upper_index));
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
}