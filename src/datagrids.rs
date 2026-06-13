use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use astroimsim_geometry::grid1d::{Location1D, Neighbors, GRID1D};
use astroimsim_geometry::grid2d::GRID2D;
use eframe::wgpu::naga::SpecialTypes;
use plotpy::{Curve, Plot};
use crate::datafile::{DataFile, FILETYPE};
use crate::units::{c_CGS, h_CGS, kB_CGS, SpectralDensityData, SpectrumUnits};


pub enum FakeCurve{
    BlackBodyKelvin(f64),
    FlatAB(SpectralDensityData)
}
#[derive(Clone,Debug)]
pub enum DataSource{
    File(DataFile), //data file, delineator
    None
}

#[derive(Debug,Clone)]
pub struct DATAGRID1D{
    pub grid1d: GRID1D,
    pub data: Vec<(usize,SpectralDensityData)>, //a vector of values (point number on grid, data values at that point)
    pub data_shape: (usize,usize), //shape of data at each grid point
    pub label: &'static str,
    pub units:SpectrumUnits,
    pub source: DataSource,
}
#[derive(Clone, Debug)]
pub struct DATAGRID2D{
    pub grid1d: GRID2D,
    pub data: Vec<(usize,Vec<f64>)>, //a vector of values (point number on grid, data values at that point)
    pub data_shape: (usize,usize), //shape of data at each point
    pub label: &'static str,
    pub source: DataSource,
}



impl DATAGRID1D {

    pub fn new_empty(grid1d:GRID1D,data_shape:(usize,usize),label:&'static str,units:SpectrumUnits)->DATAGRID1D{
        DATAGRID1D{
            grid1d,
            data:vec![],
            data_shape,
            label,
            units,
            source: DataSource::None,
        }
    }
    pub fn get_data(&self, index: usize) -> SpectralDensityData {
        assert_eq!(self.data[index].0, index);
        self.data[index].1.clone()
    }

    fn load_dat(&mut self, path:&'static str, delineator:String, plot: bool) {
        assert_ne!(0, self.data.len(), "Loading data {:?} into would overwrite current data", self.label);
        println!("Loading {:?} into {:?}", path, self.label);
        let start = Instant::now();
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);
        let data: Vec<Vec<f64>> = reader.lines().map(|line| {
            let line = line.expect("Failed to read line");
            let mut parsable = true;
            let parsed: Vec<f64> = line
                .trim()
                .split(&delineator)
                .map(|a|a.parse::<f64>())
                .map(|result|
                    match result {
                        Ok(value) => value,
                        Err(_) => {parsable = false; 0.0},
                    }).collect();
            if parsable { //check there were no errors in parsing
                if parsed.len() == self.data_shape.0 * self.data_shape.1 + 1 {
                    parsed //add to list if there is the expected number of data points
                }
                else{ vec![] }
            } else{vec![]}
        }).collect();
        println!("Parsed {:?} lines in {:?}", data.len(), start.elapsed().as_millis());
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
            for i in 1..data[0].len() {
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
            snapped_data.push((index, SpectralDensityData {values:datum,units:self.units.clone()})) //TODO this must change to plot multiple
        }
        self.data = snapped_data;
    }

    fn load_data(&mut self){
        match &self.source{
            DataSource::File(file) => {
                match &file.file_type{
                    FILETYPE::DAT(delinator) => {
                        println!("Loading .dat file");
                        self.load_dat(file.path, delinator.clone(), false, /* SpectrumUnits */)
                    }
                    FILETYPE::FITS => {panic!("unimplemented fits loading for 1D")}
                }
            }
            DataSource::None => {panic!("Can't load data from Datasource::None")}
        }
    }
/*
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
    
 */


    pub fn re_grid(&mut self, new_grid: GRID1D) -> DATAGRID1D {
        assert!(new_grid.snap_precision <= self.grid1d.snap_precision, "Snap precision of new grid must be less than or equal to that of the original grid");
        self.data.sort_by_key(|x| x.0); //TODO move this into a validation function
        let mut new_data = Vec::new();
        for point in 0..new_grid.num() {
            let new_location = new_grid.location(point);
            let value = match self.grid1d.inside_or_outside(new_location) {
                Location1D::TooHigh => {
                    println!("new gridding {:?},location is {:?}, too high", point, new_location);
                    self.data[self.data.len() - 1].1.clone()
                    
                }
                Location1D::TooLow => {
                    println!("new gridding {:?},location is {:?}, too low", point, new_location);
                    self.data[0].1.clone()
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
                            assert_eq!(upper_data.units, lower_data.units,"Can not interpolate between points with data of different units. This shouldn't happen.... Something is suspicious with your grid.");
                            let new_values = upper_data.values.iter().zip(lower_data.values.iter()).map(|(a, b)|
                                a * upper_weight + b * lower_weight).collect();
                            SpectralDensityData{values:new_values,units:upper_data.units.clone()}
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



    pub fn generate(&mut self, fake_curve: FakeCurve){
        assert_eq!(self.grid1d.num(),0,"Can't load fake curve onto grid with data already! Try with an empty grid :(");
        match fake_curve {
            FakeCurve::BlackBodyKelvin(T) => {
                for point in 0..self.grid1d.num(){
                    let wavelength = self.grid1d.location(point);
                    let scale_factor = 2.0*std::f64::consts::PI*h_CGS*c_CGS.powi(2)/(wavelength.powi(5));
                    let exponent = h_CGS*c_CGS/(wavelength*kB_CGS*T);
                    let F_lambda = scale_factor*(1.0/(exponent.exp()-1.0));
                    self.data.push((point,SpectralDensityData{values:vec![F_lambda],units:SpectrumUnits::F_lambda}))
                }
            }
            FakeCurve::FlatAB(densities) => {
                
            }
        }
    }
}
