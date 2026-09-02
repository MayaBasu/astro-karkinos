use std::time::Instant;
use crate::geometry;
use uvex_fitrs::{Fits, Hdu};
use crate::point_sources::FullSpectrumSourceList;
use std::fs;
use crate::geometry::CoordinateSystem;
use crate::grid2d::GRID2D;
use crate::spatial_effect::SpatialEffect;

pub enum EffectType{
    Exposure(f64),
    Once
}

#[derive(Clone,Debug)]
pub struct Detector {
    pub label: String,
    pub grid: GRID2D,
    pub data: Vec<Vec<[f64;2]>>,
}

impl Detector {


    pub fn new(label:String, grid: GRID2D)->Detector{
        Detector{
            label,
            grid,
            data:vec![vec![]]
        }

    }

    pub fn write(&mut self, directory_path:String, label:usize){


        let width = self.data[0].len();
        let height = self.data.len();
        let shape = [width, height];

        let mut fuv_path = directory_path.clone();
        fuv_path.push_str(&format!("/fuv_{label}.fits"));
        
        let mut nuv_path = directory_path.clone();
            nuv_path.push_str(&format!("/nuv_{label}.fits"));
        
        let fuv_counts:Vec<f64> = self.data.iter().flatten().map(|a|a[0]).collect();
        let nuv_counts:Vec<f64> = self.data.iter().flatten().map(|a|a[1]).collect();
        
        let mut fuv_counts_primary_hdu = Hdu::new(&shape, fuv_counts);
        let mut nuv_counts_primary_hdu = Hdu::new(&shape, nuv_counts);
        
        Fits::create(fuv_path, fuv_counts_primary_hdu).expect("Failed to create");
        Fits::create(nuv_path, nuv_counts_primary_hdu).expect("Failed to create");

    }

    pub fn show(path:&str){
        //Users/mayabasu/Desktop/Output/fuv/1.fits
    }

    pub fn create_constant_background(&mut self, fuv_background_brightness:f64,nuv_background_brightness:f64){
        let mut data = Vec::with_capacity(self.grid.y_num);
        for row in 0..self.grid.y_num{
            let mut row_vec = Vec::with_capacity(self.grid.x_num);
            for column in 0..self.grid.x_num{
                row_vec.push([fuv_background_brightness,nuv_background_brightness])
            }
            data.push(row_vec);
        }
        self.data= data;
    }
    pub fn multiply_effect(&mut self, effect:&SpatialEffect,index:usize, effect_type: EffectType){
        assert_eq!(effect.grid.num_points, self.grid.num_points, "Grids are not equal");
        for row in 0..self.grid.y_num{
            for column in 0..self.grid.x_num{
                match effect_type{
                    EffectType::Exposure(time) => {self.data[column][row][index] = self.data[column][row][index]*effect.data[column][row]*time;}
                    EffectType::Once => {self.data[column][row][index] = self.data[column][row][index]*effect.data[column][row];}
                }

            }
        }
    }

    pub fn add_effect(&mut self, effect:&SpatialEffect,index:usize,effect_type: EffectType){
        assert_eq!(effect.grid.num_points, self.grid.num_points, "Grids are not equal");
        for row in 0..self.grid.y_num{
            for column in 0..self.grid.x_num{
                match effect_type{
                    EffectType::Exposure(time) => {self.data[column][row][index] = self.data[column][row][index]+effect.data[column][row]*time;}
                    EffectType::Once => {self.data[column][row][index] = self.data[column][row][index]+effect.data[column][row];}
                }

            }
        }
    }
    
    pub fn multiply_interpolated_effect(&mut self, effect:&SpatialEffect, index:usize){
        for row in 0..self.grid.y_num {
            for column in 0..self.grid.x_num {
                let grid_number = self.grid.grid_number(column, row);
                let location = self.grid.locate(grid_number);
                let effect = effect.get_data(&location);
                self.data[column][row][index] = self.data[column][row][index] * effect;
            }
        }
    }

}

pub struct DetectorArray{
    pub label: String,
    pub detectors: Vec<Detector>,
    pub coordinate_system: CoordinateSystem
}





