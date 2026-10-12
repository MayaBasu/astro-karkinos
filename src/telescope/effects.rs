use std::fs::File;
use std::io::{BufRead, BufReader};
use uvex_fitrs::{Fits, FitsData, FitsDataArray};
use crate::gridded_data::{units, UnitError};
use crate::geometry::*;
use crate::geometry::Grid1DUnits::{angstroms, nm};
use crate::gridded_data::{DATA1D, UnitlessUnit, DATA2D, SpectralUnits};
use crate::gridded_data::SpectralUnits::f_lambda;

#[derive(Debug,Clone)]
pub struct SpectralResponse{
    data: Vec<f64>,
    grid1d: GRID1D,
    unit: UnitlessUnit,
}

/*
Spectral Response assumptions:
1. One data point per grid point
2. Data is normalized
3. Data values are Regular
 */



impl DATA1D<UnitlessUnit> for SpectralResponse{
    fn new(grid1d: GRID1D, data: Vec<f64>,unit:UnitlessUnit) -> Self {

        if !Self::is_data_regular(&data){
            panic!("Can not initialize spectral response with non-regular values: +/- inf or NaN")
        }
        assert_eq!(grid1d.num_points(), data.len(),
                   "Failed to initialize new SpectralResponse struct:\
                   Must have exactly one data point per grid point");

        SpectralResponse{
            data,
            grid1d,
            unit
        }
    }
    fn grid(&self) -> &GRID1D {
        &self.grid1d
    }
    fn data(&self) -> &Vec<f64> {
        &self.data
    }
    fn unit(&self) -> &UnitlessUnit {
        &self.unit
    }

    fn multiply(&self, other: &Self) -> Self {
        let data = self.multiply_data(other);
        Self::new(other.grid1d, data, self.unit)
    }


}

impl SpectralResponse{

    pub fn multiply_responses(responses: Vec<SpectralResponse>)->SpectralResponse{
        assert!(responses.len() > 0, "can not multiply 0 responses");
        let mut base:SpectralResponse = responses[0].clone();
        responses.into_iter().for_each(|response|{
            base.multiply(&response);
        });
        base

    }
    pub fn square_wave(grid: GRID1D, lower: f64, upper:f64)->Self{
        let data = grid.locate_grid_points().iter().map(|&p|{
            if p < lower{
                0.
            } else if p > upper{
                0.
            }else{
                1.
            }
        }).collect::<Vec<f64>>();
        Self::new(grid, data, UnitlessUnit::NormalizedFraction)
    }
}

#[derive(Clone, Debug)]
pub struct SpatialEffect{
    pub(crate) grid: GRID2D,
    data:Vec<Vec<f64>>,
}


impl DATA2D<UnitlessUnit> for SpatialEffect{
    fn grid(&self) -> &GRID2D {
        &self.grid
    }
    fn data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }

    fn mutable_data(&mut self) -> &mut Vec<Vec<f64>> {
        &mut self.data
    }

    fn unit(&self) -> &UnitlessUnit {
        &UnitlessUnit::NormalizedFraction
    }
} 

impl SpatialEffect{

    pub fn scale(&mut self, scalar:f64){
        let new_data = self.data.iter().map(|row|{
            row.iter().map(|element|{
                element*scalar
            }).collect()
        }).collect();
        self.data = new_data;
        //TODO should reinitialize but dont want to normalize
    }

    pub fn normalize(data:Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>,FloatError>{
        if !Self::is_data_regular(&data){
            return Err(FloatError::ValueNotRegular)
        }
        let sum:f64 = data.clone().into_iter().flatten().sum();
        if sum == 0.0{
            return Err(FloatError::ValueIsZero)
        }
        Ok(data.into_iter().map(|v|v.into_iter().map(|e|e/sum).collect()).collect())
    }

    pub fn new(grid:GRID2D, data: Vec<Vec<f64>>) -> Result<SpatialEffect,FloatError>{
        let data = Self::normalize(data)?;
       // println!("{:?}",grid);
        assert_eq!(data.len(), grid.y_num());
        data.iter().for_each(|row|
            {assert_eq!(row.len(), grid.x_num())});
        Ok(SpatialEffect{
            grid,
            data,
        })
    }

    fn combine_2_add(&self, effect:&SpatialEffect) ->SpatialEffect {
        let mut data = self.data().clone();
        for y in 0..self.grid().y_num(){
            for x in 0..self.grid().x_num(){
                let grid_number = self.grid().grid_number(x,y);
                let point = self.grid().locate(grid_number);
                let value = if effect.grid().is_point_inside(&point){
                    effect.value(&point)
                }else{
                    0.0
                };
                data[y][x] += value
            }
        }
        SpatialEffect{
            grid:self.grid.clone(),
            data
        }
    }

    pub fn from_fits(grid:GRID2D,path:&str)-> Result<SpatialEffect,FloatError>{
        let fits = Fits::open(path).expect(&format!("Failed to open {path} as FITS file"));
        let primary_hdu= fits.iter().next().expect(&format!("Couldn't find primary HDU in {path}"));

        let (data,shape) = match primary_hdu.read_data() {
            FitsData::FloatingPoint64(FitsDataArray { shape, data }) => (data,shape),
            FitsData::FloatingPoint32(FitsDataArray { shape, data }) => {
                let data = data.into_iter().map(|x|x as f64).collect();
                (data,shape)},
            _ => {panic!("Failed to read data from FITS file: {path}")}
        };
        assert_eq!(shape[0],grid.x_num(),
                   "{}", format!("Expected {:?} points in x dimension, loaded {:?}",grid.x_num(),shape[0]));
        assert_eq!(shape[1],grid.y_num(),
                   "{}", format!("Expected {:?} points in y dimension, loaded {:?}",grid.y_num(),shape[1]));
        let data = data.chunks(grid.x_num()).map(|v|v.to_vec()).collect();
        Self::new(grid,data)

        }


}
