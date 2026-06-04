use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::Instant;
use plotpy::{Curve, Plot};
use astroimsim_geometry::grid1d::{Location1D, Neighbors, GRID1D};

#[derive(Clone, Debug)]
pub struct FrequencyFile {
    pub path: &'static str,
    pub grid1d: GRID1D,
    pub data: Vec<(usize,Vec<f32>)>
}

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


    pub fn load_data(mut self, plot:bool) -> FrequencyFile{
        println!("Loading data file {:?}", self.path);
        let start = Instant::now();
        let file = File::open(self.path.clone()).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut data = Vec::new();

        for (i,line_result) in reader.lines().enumerate() {
            let line = line_result.expect("failed to read line");
            let line = line.trim();
            let line:Vec<&str> = line.split("   ").collect();
            let mut valid_line = true;
            let mut parsed_line = Vec::new();
            for (index, element) in line.iter().enumerate(){
                match element.trim().parse::<f32>() {
                    Ok(val) => {
                        if index ==0{
                            //println!("pushing {:?}",val*scale);
                            parsed_line.push(val*self.grid1d.scale as f32)
                        }else{
                            parsed_line.push(val)
                        }}
                    Err(_) => {valid_line = false}
                };
            }
            if !valid_line{
                println!("line {:?} invalid: {:?}",i,parsed_line)
            }else{
                /*
                if parsed_line.len() == floats_per_record{

                 */
                data.push(parsed_line)
                /*
                }else{
                    println!("{:?}",parsed_line);
                    panic!("Parsed a different number of floats in a line than expected")
                }

                 */
            }
        }
        /*
        let mut result = [[0.0;floats_per_record]; records];
        for record in 0..records {
            let mut record_as_array = [0.0;floats_per_record];
            for float in 0..floats_per_record {
                record_as_array[float] = data[record][float]
            }
            result[record] = record_as_array;
        }

         */


        println!("Parsed {:?} records in {:?} ms",data.len(), start.elapsed().as_millis());
        assert_eq!(data.len(),self.grid1d.num(),"Retrieved a different number of records than expected");
        let mut same_num_data_points_per_record = true;
        for i in 0..data.len()-1{
            if data[i].len() - data[i+1].len() != 0{
                same_num_data_points_per_record = false;
            }
        }
        if !same_num_data_points_per_record{
            println!("Warning! There are different numbers of records for each line. This may mess up plotting or indicate a loading error. Will be plotting with {:?}", data[0].len()-1)
        }//TODO run this function witha  "verbose" to list out the differences

        if plot{
            let mut plot = Plot::new();
            for i in 1..data[0].len() {
                let mut curve = Curve::new();
                curve.set_line_width(2.0);
                curve.points_begin();
                for point in data.clone() {
                    // println!("{:?}", point);

                    curve.points_add(point[0], point[i]);
                }
                curve.points_end();
                plot.add(&curve).grid_and_labels("x", "y");
            }
            plot.show("ksenf").expect("hHHHHHH");
        }
        println!("The first record is {:?} and the last is {:?}, snapping to grid: {:?}",data[0],data[data.len()-1],self.grid1d);
        let mut snapped_data = Vec::new();
        for datum in data{
            let location = datum[0];
            let index = self.grid1d.snap(location as f64);

            snapped_data.push((index,datum)) //TODO this must change to plot multiple
        }
        self.data = snapped_data;
        self

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



