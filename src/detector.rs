use std::time::Instant;
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use uvex_fitrs::{Fits, Hdu};
use crate::point_sources::FullSpectrumSourceList;
use crate::psf_grid::PsfGrid;
use std::fs;
use crate::spatial_effect::SpatialEffect;

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

    pub fn write(&mut self,label:usize){


        let width = self.data[0].len();
        let height = self.data.len();
        let shape = [width, height];

        let mut fuv_counts_path = "/Users/mayabasu/Desktop/Output/fuv/testtt".to_string();
            fuv_counts_path.push_str(label.to_string().as_str());
        fuv_counts_path.push_str(".fits");

        let mut nuv_counts_path = "/Users/mayabasu/Desktop/Output/nuv/testtt".to_string();
            nuv_counts_path.push_str(label.to_string().as_str());
        nuv_counts_path.push_str(".fits");

        /*

        let mut fuv_ave_path = "/Users/mayabasu/Desktop/Output/fuv/counts_".to_string();
            fuv_ave_path.push_str(label.to_string().as_str());
            fuv_ave_path.push_str(".fits");


        let mut nuv_ave_path = "/Users/mayabasu/Desktop/Output/nuv/counts".to_string();
            nuv_ave_path.push_str(label.to_string().as_str());
                nuv_ave_path.push_str(".fits");




        let fuv_ave:Vec<f64> = self.data.iter().flatten().map(|a|a[0]).collect();
        let nuv_ave:Vec<f64> = self.data.iter().flatten().map(|a|a[1]).collect();

         */


        let fuv_counts:Vec<f64> = self.data.iter().flatten().map(|a|a[0]).collect();
        let nuv_counts:Vec<f64> = self.data.iter().flatten().map(|a|a[1]).collect();




        /*
        let mut fuv_primary_hdu = Hdu::new(&shape, fuv_ave);
        let mut nuv_primary_hdu = Hdu::new(&shape, nuv_ave);

         */

        let mut fuv_counts_primary_hdu = Hdu::new(&shape, fuv_counts);
        let mut nuv_counts_primary_hdu = Hdu::new(&shape, nuv_counts);


        // Insert values in header later
        //primary_hdu.insert("KEYSTR", "My string");
        /*
        println!("{:?}",fuv_ave_path);
        Fits::create(fuv_ave_path, fuv_primary_hdu).expect("Failed to create");
        Fits::create(nuv_ave_path, nuv_primary_hdu).expect("Failed to create");

         */
        Fits::create(fuv_counts_path, fuv_counts_primary_hdu).expect("Failed to create");
        Fits::create(nuv_counts_path, nuv_counts_primary_hdu).expect("Failed to create");





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
    pub fn apply_effect(&mut self, effect:SpatialEffect,index:usize){
        assert_eq!(effect.grid.num_points, self.grid.num_points, "Grids are not equal");
        for row in 0..self.grid.y_num{
            for column in 0..self.grid.x_num{
                self.data[column][row][index] = self.data[column][row][index]*effect.data[column][row];
            }
        }
    }

}

pub struct DetectorArray{
    pub label: String,
    pub detectors: Vec<Detector>,
    pub coordinate_system: CoordinateSystem
}





