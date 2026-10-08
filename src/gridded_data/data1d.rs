use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::gridded_data::units::UnitError;
use crate::gridded_data::UnitlessUnit;

use crate::geometry::Grid1DUnits::{angstroms, nm};
use crate::gridded_data::SpectralUnits;
use crate::gridded_data::SpectralUnits::f_lambda;

use crate::geometry::*;
use crate::gridded_data::units::Unit;


#[derive(Debug)]
pub enum FileParsingError{
    FormatIncorrect(String),
    UnitParsingError(String)

}

impl From<UnitError> for FileParsingError{
    fn from(value: UnitError) -> Self {
        match value{
            UnitError::ParsingError(s) => {Self::UnitParsingError(s)}
        }
    }
}


pub trait DATA1D<Unit:super::units::Unit>{

    fn new(grid1d: GRID1D, data: Vec<f64>,unit:Unit) -> Self;
    fn grid(&self)-> &GRID1D;
    fn data(&self)-> &Vec<f64>;
    fn unit(&self)-> &Unit;
    fn load(path: &str,snap_precision:f64) -> Result<Self,FileParsingError> where Self: Sized {
        match Self::load_data(path){
            Ok(((grid_unit,grid_data),(unit,data))) => {
                let grid = grid1d::GRID1D::from_values(grid_data,snap_precision,grid_unit);
                Ok(Self::new(grid, data, unit))
            }
            Err(e) => {Err(e)}
        }
    }
    fn regrid(&self, grid1d:&GRID1D)->Self where Self: Sized{
        let new_data = self.values(&grid1d);
        Self::new(*grid1d,new_data,*self.unit())
    }
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
       // assert!(new_grid.snap_precision <= self.grid().snap_precision, "Snap precision of new grid must be less than or equal to that of the original grid");
        //TODO: why did I put this assert! in?
        new_grid.locate_grid_points().iter().map(|x|{
            self.value(*x,new_grid.unit())
        }).collect()
    }

    fn multiply_data(&self, other: &Self) -> Vec<f64> where Self: Sized {
        let regrid = self.regrid(other.grid());
        assert_eq!(regrid.data().len(), other.data().len(), "unreachable");
        let new_data = regrid.data().iter().zip(other.data())
            .map(|(x,y)|(x*y)).collect::<Vec<f64>>();
        assert!(Self::is_data_regular(&new_data));
        new_data
        //TODO: Check

    }

    fn multiply(&self, other: &Self) -> Self;
    fn write_to_dat(&mut self, header:&str, path:&str){
        let mut file = File::create(path).expect("Could not create file");
        file.write_all(header.as_bytes()).expect("Failed to write header");
        file.write_all(format!("\n{:?} | {:?}",self.grid().unit(), self.unit()).as_bytes())
            .expect("Failed to write units to file");
        for (point,value) in (0..self.grid().num_points()).zip(self.data()){
            let location = self.grid().locate_grid_point(point);
            file.write_all(format!("\n{:?} | {:?}", location, value).as_bytes())
                .expect("Failed to write data");
        }
    }
    fn load_data(path:&str)-> Result<((Grid1DUnits,Vec<f64>),(Unit,Vec<f64>)), FileParsingError>{
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut parsed_header = false;
        let mut lines_read = 0;
        let mut comments_ignored = 0;
        let mut spatial_data = Vec::new();
        let mut data = Vec::new();
        let mut grid_unit:Option<Grid1DUnits>=None ;
        let mut data_unit:Option<Unit> = None;

        //TODO deal with 0 line files/uninitialization
        for line in reader.lines(){
            let line = line.unwrap();
            if line.chars().nth(0).unwrap() =='#' {
                comments_ignored += 1;
                continue
            }
            let line = line.trim().split("|").map(|s|s.trim()).collect::<Vec<&str>>();
            if line.len() != 2{
                return Err(FileParsingError::FormatIncorrect("Expected two entries per row separated by '|' ".to_string()))
            }
            if !parsed_header{
                println!("header: {:?}",line);
                let unit  = Grid1DUnits::from_str(line[0])?;
                grid_unit  = Some(unit);
                let unit = Unit::from_str(line[1])?;
                data_unit = Some(unit);
                parsed_header = true;
                continue
            }

            if parsed_header{
                let gridpoint = line[0].parse::<f64>().unwrap();
                let datapoint = line[1].parse::<f64>().unwrap();
                lines_read += 1;
                spatial_data.push(gridpoint);
                data.push(datapoint);
                continue
            }
            else{unreachable!()}
        };

        assert!(lines_read > 1);
        Ok(((grid_unit.unwrap(),spatial_data),(data_unit.unwrap(),data)))
    }
}

