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
    pub fn load_file(file: &str, center:(Load, Load), size:(Load, Load),grid:GRID2D) -> PSF {
        println!("Loading {:?} into a DataFrame ",file);
        let fits = Fits::open(file.clone()).expect("Failed to open FITS file");
        let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");
        let (mut data,shape) = match primary_hdu.read_data() {
            FitsData::FloatingPoint32(FitsDataArray { shape, data }) => (data,shape),
            _ => panic!("Could not unpack PSF data")
        }; //TODO add support for f64 etc

        assert_eq!(shape[0], grid.x_num,"Diva down! Tried to load a file with data of the wrong x size"); //check that the data is the expected size
        assert_eq!(shape[1], grid.y_num,"Diva down! Tried to load a file with data of the wrong y size");


        let center_x:f64 = PSF::load(center.0, &primary_hdu);
        let center_y:f64 = PSF::load(center.1, &primary_hdu);
        let size_x:f64 = PSF::load(size.0, &primary_hdu);
        let size_y:f64 = PSF::load(size.1, &primary_hdu);
        let data = data.chunks(grid.x_num).map(|i| i.to_vec()).collect();
        println!("{:?}",data);
        PSF {
            path: file.parse().unwrap(),
            data,
            x_pixels: grid.x_num,
            y_pixels: grid.y_num,
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


use std::time::Instant;

use astroimsim_spectra::spectral_response::SpectralResponseCurve;


#[derive(Clone,Debug)]
pub struct SpatialEffect {
    pub label: &'static str,
    pub grid: GRID2D,
    pub data: Vec<Vec<f64>>, //data must be fractional - implement percent later?
    pub fits_path: &'static str,
}

impl SpatialEffect{
    pub fn new_empty(label:&'static str, grid:GRID2D,fits_path:&'static str)-> SpatialEffect{
        SpatialEffect{label,grid,data:vec![],fits_path}
    }

    pub fn from_matrix(label:&'static str, grid:GRID2D,fits_path:&'static str,data:Vec<Vec<f64>>)-> SpatialEffect{
        assert_eq!(grid.y_num,data.len());
        for row in &data{ assert_eq!(grid.x_num, row.len()); }
        SpatialEffect{label,grid,fits_path,data}
    }

    pub fn load_data(&mut self){
        println!("Loading {:?} into {:?}",self.fits_path, self.label);
        let fits = Fits::open(self.fits_path).expect("Failed to open FITS file");
        let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");
        let (mut data,shape) = match primary_hdu.read_data() {
            FitsData::FloatingPoint64(FitsDataArray { shape, data }) => (data,shape),
            _ => {panic!("huh? Couldn't load FITS file")}
        };
        assert_eq!(shape[0],self.grid.x_num,"FITS data had the wrong width");
        assert_eq!(shape[1],self.grid.y_num,"FITS data had the wrong height");
        let data:Vec<Vec<f64>> = data.chunks(self.grid.x_num).map(|v|v.to_vec()).collect();
        self.data = data;

    }
    pub fn get_data_at_grid_index(&self, grid_number:usize)->f64{
        let (x,y) = self.grid.xy_indices(grid_number);
        self.data[y][x]
    }

    pub fn get_data(&self,point:&Point)->f64{
        let ((i12, i22, i21, i11),(c11,c12,c21,c22),normalization) = self.grid.interpolation_coefficients(point);
        let q11 = self.get_data_at_grid_index(i11);
        let q12 = self.get_data_at_grid_index(i12);
        let q21 = self.get_data_at_grid_index(i21);
        let q22 = self.get_data_at_grid_index(i22);
        (q11*c11 + q12*c12 + q21*c21 + q22*c22)/normalization
    }

}










