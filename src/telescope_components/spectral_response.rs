use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

use crate::geometry::*;

use crate::inputs::power_spectrum::PowerSpectrum;


#[derive(Debug,Clone)]
pub struct SpectralResponseCurve {
    pub grid1d: GRID1D,
    pub data: Vec<(usize, f64)>, //a vector of values (point number on grid, data value at that point)
    //TODO: Remove this
    pub label: String,
    //TODO: Remove this
    pub dat_path: String, //None, list (combination), or multiline, or single line regualr
    pub units:Units
}


impl SpectralResponseCurve {

    pub fn new(label:String, grid1d: GRID1D,dat_path:String,n:usize,delineator:&str,units:Units)-> SpectralResponseCurve{
        let mut new = SpectralResponseCurve{
            grid1d,
            data:vec![],
            label,
            units,
            dat_path
        };
        new.load_data(n,delineator,units);
        new

    }


    pub fn get_data(&self, index: usize) -> f64 {
        assert_eq!(self.data[index].0, index);
        self.data[index].1.clone()
    }

    pub fn write_to_dat(&mut self, path:&str, header:&str, grid:&GRID1D){

        self.re_grid(grid);
        let mut file = File::create(path).expect("Could not create file");
        file.write_all(format!(
            "{:?} \n {:?} \n {:?} \n {:?} \n \n \n",
            self.label, self.dat_path, self.grid1d, header).as_bytes()).expect("Failed to write header");

        for (point,value) in &self.data{
            let location = self.grid1d.locate_grid_point(*point);
            file.write_all(format!("\n{:?}   {:?}", location, value).as_bytes()).unwrap();
        }

    }
    
    pub fn load_data(&mut self,n:usize,delineator:&str,units:Units) {
        assert_eq!(0, self.data.len(), "Loading data {:?} into would overwrite current data", self.label);
        println!("Loading {:?} into {:?}", self.dat_path, self.label);
        let start = Instant::now();
        let file = File::open(self.dat_path.clone()).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut data: Vec<Vec<f64>> = Vec::new();
        for line in reader.lines(){

            let mut parsable = true;
            let line = line.expect("Failed to read line");

            let line = line.trim()
                .split(&delineator)
                .map(|a|
                         {//println!("{:?}",a.trim());
                    a.trim().parse::<f64>()})
                .map(|result|
                    match result {
                        Ok(value) => {//println!("{:?}",value);
                            value},
                        Err(v) => {//println!("COULDN:T PARSE {:?}",v);
                             parsable = false; 0.0},
                    }).collect();
           // println!("{:?}",line);
            if parsable{
                data.push(line)
            }
           // println!("{:?} ",parsable);
        }
        println!("Parsed {:?} lines in {:?} ms", data.len(), start.elapsed().as_millis());
       // println!("{:?}",data);
       // println!("{:?}",self.grid1d);
        assert_eq!(data.len(), self.grid1d.num_points(), "Retrieved a different number of records than expected");


        let mut same_num_data_points_per_record = true;
        for i in 0..data.len() - 1 {
            if data[i].len() - data[i + 1].len() != 0 {
                same_num_data_points_per_record = false;
            }
        }
        if !same_num_data_points_per_record {
            println!("Warning! There are different numbers of records for each line. This may mess up plotting or indicate a loading error. Will be plotting with {:?}", data[0].len() - 1)
        } //TODO run this function witha  "verbose" to list out the differences
 
       // println!("The first record is {:?} and the last is {:?}, snapping to grid: {:?}", data[0], data[data.len() - 1], self.grid1d);
        let mut snapped_data = Vec::new();
        for values in data {
            //TODO reimplement scale here

            let location =Regular::try_from(values[0]);
            let data_point = values[n];

            let index = self.grid1d.snap(location,units).expect("Could not grid point");
            snapped_data.push((index,data_point)) //TODO this must change to plot multiple
        }
        self.data = snapped_data;
    }

    pub fn re_grid(&mut self, new_grid: &GRID1D) {
        assert!(new_grid.snap_precision <= self.grid1d.snap_precision, "Snap precision of new grid must be less than or equal to that of the original grid");
        self.data.sort_by_key(|x| x.0); //TODO move this into a validation function
        let mut new_data = Vec::new();
        for point in 0..new_grid.num_points() {
            let new_location = Regular::try_from(new_grid.locate_grid_point(point));
            let value = match self.grid1d.locate_regular(new_location,self.units){
                Location::Higher => {self.data[self.data.len() - 1].1}
                Location::Lower => {self.data[0].1}
                Location::Snapped(index) => {self.data[index].1}
                Location::Between(lower, upper, lower_weight) => {
                    let lower = self.data[lower].1;
                    let upper = self.data[upper].1;
                    let upper_weight = Regular::try_from(1.0-lower_weight);
                    (upper*upper_weight+Regular::try_from(lower*lower_weight)).value()
                }
            };
            new_data.push((point, value))
        }
        self.data = new_data;
        self.grid1d = new_grid.clone();
    }
//TODO implement partial equality for 1d Grids

    pub fn compose(mut responses:Vec<SpectralResponseCurve>) -> SpectralResponseCurve{
        for i in 0..responses.len()-1{
            assert!(responses[i].units == responses[i+1].units, "Can not compose responses with different units")
        }
        //TODO need to verify that the units are handeled
        let grid = responses[0].grid1d.clone();
        let mut new_data = responses[0].data.clone();
        //TODO should add in verification
        let mut labels = Vec::new();
        for response in responses[1..].iter_mut() {
            response.re_grid(&grid);
            labels.push(response.label.clone());
            for index in 0..grid.num_points(){
                let value = response.get_data(index);
                new_data[index].1 *= value;
            }
        }
        SpectralResponseCurve{
            grid1d: grid.clone(),
            data: new_data.clone(),
            label: "Composition of some stuff (TODO: ADD WHICH STUFF))".to_string(),
            units: responses[0].units,
            dat_path: "N/A".to_string(), //TODO add alterantive data
        }

    }

    pub fn self_compose(&mut self,number_of_times:i32){
        for mut data_point in &mut self.data{
            data_point.1 = data_point.1.powi(number_of_times);
        }
    }



}