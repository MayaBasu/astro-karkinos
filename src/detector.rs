use std::time::Instant;
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use uvex_fitrs::{Fits, Hdu};
use crate::point_sources::FullSpectrumSourceList;
use crate::psf_grid::PsfGrid;
use std::fs;

#[derive(Clone,Debug)]
pub struct Detector {
    pub label: &'static str,
    pub grid: GRID2D,
    pub data: Vec<Vec<[f64;4]>>,
}

impl Detector {

    pub fn write(&mut self,label:usize){


        let width = self.data[0].len();
        let height = self.data.len();
        let shape = [width, height];

        let mut fuv_counts_path = "/Users/mayabasu/Desktop/Output/fuv/".to_string();
            fuv_counts_path.push_str(label.to_string().as_str());
        fuv_counts_path.push_str(".fits");

        let mut nuv_counts_path = "/Users/mayabasu/Desktop/Output/nuv/".to_string();
            nuv_counts_path.push_str(label.to_string().as_str());
        nuv_counts_path.push_str(".fits");

        let mut fuv_ave_path = "/Users/mayabasu/Desktop/Output/fuv/counts_".to_string();
            fuv_ave_path.push_str(label.to_string().as_str());
            fuv_ave_path.push_str(".fits");


        let mut nuv_ave_path = "/Users/mayabasu/Desktop/Output/nuv/counts".to_string();
            nuv_ave_path.push_str(label.to_string().as_str());
                nuv_ave_path.push_str(".fits");


        let fuv_ave:Vec<f64> = self.data.iter().flatten().map(|a|a[0]).collect();
        let nuv_ave:Vec<f64> = self.data.iter().flatten().map(|a|a[1]).collect();
        let fuv_counts:Vec<f64> = self.data.iter().flatten().map(|a|a[2]).collect();
        let nuv_counts:Vec<f64> = self.data.iter().flatten().map(|a|a[3]).collect();

        let mut fuv_primary_hdu = Hdu::new(&shape, fuv_ave);
        let mut nuv_primary_hdu = Hdu::new(&shape, nuv_ave);
        let mut fuv_counts_primary_hdu = Hdu::new(&shape, fuv_counts);
        let mut nuv_counts_primary_hdu = Hdu::new(&shape, nuv_counts);
        // Insert values in header later
        //primary_hdu.insert("KEYSTR", "My string");
        println!("{:?}",fuv_ave_path);
        Fits::create(fuv_ave_path, fuv_primary_hdu).expect("Failed to create");
        Fits::create(nuv_ave_path, nuv_primary_hdu).expect("Failed to create");
        Fits::create(fuv_counts_path, fuv_counts_primary_hdu).expect("Failed to create");
        Fits::create(nuv_counts_path, nuv_counts_primary_hdu).expect("Failed to create");





    }

    pub fn show(path:&str){
        //Users/mayabasu/Desktop/Output/fuv/1.fits

    }
}

pub struct DetectorArray{
    pub label: String,
    pub detectors: Vec<Detector>,
    pub coordinate_system: CoordinateSystem
}





