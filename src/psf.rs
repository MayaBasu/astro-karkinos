use std::iter::Flatten;
use std::path::PathBuf;
use std::slice::Iter;
use serde::{Deserialize, Serialize};
use uvex_fitrs::{Fits, FitsData, FitsDataArray, Hdu, HeaderValue};
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;
//use crate::point_source::{PointSource, SourceList};

#[derive(Debug,Clone,Serialize)]
pub struct PSF {
    pub path: PathBuf,
    pub data: Vec<Vec<f32>>,
    pub x_pixels: usize,
    pub y_pixels: usize,
    pub center: Point,
    pub size: (f64,f64),
}

pub struct DataFile {
    pub description: String,
    pub path: PathBuf,
    pub x_pixels: usize,
    pub y_pixels: usize,
}


pub enum Load{
    FromKey(String),
    FromValue(f64)
}

impl PSF {
    pub fn load_file(file:DataFile,center:(Load,Load),size:(Load,Load)) -> PSF {
        println!("Loading {:?} into a DataFrame from {:?}",file.description,file.path);
        let fits = Fits::open(file.path.clone()).expect("Failed to open FITS file");
        let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");
        let (mut data,shape) = match primary_hdu.read_data() {
            FitsData::FloatingPoint32(FitsDataArray { shape, data }) => (data,shape),
            _ => panic!("Could not unpack PSF data")
        }; //TODO add support for f64 etc

        assert_eq!(shape[0], file.x_pixels,"Diva down! Tried to load a file with data of the wrong x size"); //check that the data is the expected size
        assert_eq!(shape[1], file.y_pixels,"Diva down! Tried to load a file with data of the wrong y size");


        let center_x:f64 = PSF::load(center.0, &primary_hdu);
        let center_y:f64 = PSF::load(center.1, &primary_hdu);
        let size_x:f64 = PSF::load(size.0, &primary_hdu);
        let size_y:f64 = PSF::load(size.1, &primary_hdu);
        let data = data.chunks(file.x_pixels).map(|i| i.to_vec()).collect();
        println!("{:?}",data);
        PSF {
            path: file.path,
            data,
            x_pixels: file.x_pixels,
            y_pixels: file.y_pixels,
            center: Point::new(center_x,center_y,Coordinates::ABSOLUTE),
            size: (size_x,size_y),
        }
    }
    pub fn snap_to_grid(&self, grid: &GRID2D) -> usize{
        let index = grid.snap(self.center.clone());
        index
    }


    pub fn repack_data(flat_data: Vec<f32>) -> Vec<Vec<f32>>{
        flat_data.chunks(64).map(|i| i.to_vec()).collect()
    }

    fn load(thingy:Load, header:&Hdu)-> f64{
        match thingy{
            Load::FromKey(Key) => {
                match header.value(&Key).expect("failed to get key") {
                    HeaderValue::RealFloatingNumber(value)=> *value,
                    _ => panic!("could not unpack FITS header value")
                }}
            Load::FromValue(value) => value
        }
    }

}




