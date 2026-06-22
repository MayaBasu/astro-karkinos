use std::time::Instant;
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use uvex_fitrs::{Fits, Hdu};
use crate::point_sources::FullSpectrumSourceList;
use crate::psf_grid::PsfGrid;

#[derive(Clone,Debug)]
pub struct Detector {
    pub label: &'static str,
    pub grid: GRID2D,
    pub data: Vec<Vec<[f64;4]>>,
}

impl Detector {

    pub fn write(&mut self, path:&str){
        let width = self.data[0].len();
        let height = self.data.len();
        let shape = [width, height,4];
        let data:Vec<f64> = self.data.iter().flatten().flatten().map(|a|*a).collect();
        let mut primary_hdu = Hdu::new(&shape, data);
        // Insert values in header later
        //primary_hdu.insert("KEYSTR", "My string");
        Fits::create(path, primary_hdu).expect("Failed to create");
    }
}

pub struct DetectorArray{
    pub label: String,
    pub detectors: Vec<Detector>,
    pub coordinate_system: CoordinateSystem
}





