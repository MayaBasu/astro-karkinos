use std::iter::Flatten;
use std::path::PathBuf;
use std::slice::Iter;
use serde::{Deserialize, Serialize};
use uvex_fitrs::{Fits, FitsData, FitsDataArray, Hdu, HeaderValue};
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;
use astroimsim_geometry::grid2d::InterpolationData;
use ndarray::Array2;
use ndarray_conv::{get_fft_processor, ConvExt, ConvFFTExt, ConvMode, PaddingMode};
use std::time::{Duration, Instant};

use convolutions_rs::convolutions::*;
use ndarray::*;
use convolutions_rs::Padding;
use convolve2d::{convolve2d, DynamicMatrix, Matrix};

#[derive(Debug,Clone,Serialize)]
pub struct PSF {
    pub path: PathBuf,
    pub data: Vec<Vec<f64>>,
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


pub enum FITSType{
    thirtytwo,
    sixtyfour,
}

impl PSF {
    pub fn load_file(file: PathBuf, center:(Load, Load), size:(Load, Load), x_num:usize, y_num:usize) -> PSF {
        println!("Loading {:?} into a DataFrame ",file);
        let fits = Fits::open(file.clone()).expect("Failed to open FITS file");
        let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");
        let (mut data,shape) = match primary_hdu.read_data() {
            FitsData::FloatingPoint32(FitsDataArray { shape, data }) => (data.iter().map(|x| *x as f64).collect(),shape),
            FitsData::FloatingPoint64(FitsDataArray { shape, data }) => (data,shape),
            _ => panic!("Could not unpack PSF data")
        }; //TODO add support for f64 etc

        let normalization:f64 = data.iter().sum();
        let data:Vec<f64> = data.iter().map(|x|x/normalization).collect();
        println!("PSF has been normalized to {:?}",data.iter().sum::<f64>());

        assert_eq!(shape[0], x_num,"Diva down! Tried to load a file with data of the wrong x size"); //check that the data is the expected size
        assert_eq!(shape[1], y_num,"Diva down! Tried to load a file with data of the wrong y size");
        //TODO is this [x,y] or [y,x]

        let center_x:f64 = PSF::load(center.0, &primary_hdu);
        let center_y:f64 = PSF::load(center.1, &primary_hdu);
        let size_x:f64 = PSF::load(size.0, &primary_hdu);
        let size_y:f64 = PSF::load(size.1, &primary_hdu);
        let data = data.chunks(x_num).map(|i| i.to_vec()).collect();
      //  println!("{:?}",data);
        PSF {
            path: file,
            data,
            x_pixels: x_num,
            y_pixels: y_num,
            center: Point::new(center_x,center_y,Coordinates::ABSOLUTE),
            size: (size_x,size_y),
        }
    }



    pub fn write_file(&self, path:&str,center_keys:(String,String)){

        let data:Vec<f64> = self.data.clone().iter().map(|x|x.to_owned()).flatten().collect();
        let data= data.iter().map(|x| *x).collect();
        println!("Trying to write to {path}, {:?}",data);
        let mut primary_hdu = Hdu::new(&[64, 64], data);
        println!("Done making hdu");
        let (x,y) = self.center.to_absolute().values();
        println!("x,y,{x} {y}");
        primary_hdu.insert(center_keys.0,x.to_string().as_str() );
        primary_hdu.insert(center_keys.1,y.to_string().as_str() );
        println!("printed keys");
        //primary_hdu.insert(size_keys.0, self.size.0);
        //primary_hdu.insert(size_keys.1, self.size.1);
        Fits::create(path, primary_hdu).expect("Failed to create");
        println!("done??")


    }

    pub fn snap_to_grid(&self, grid: &GRID2D) -> usize{
        let index = grid.snap(self.center.clone());
        index
    }


    pub fn repack_data(flat_data: Vec<f64>) -> Vec<Vec<f64>>{
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

    pub fn convolve(&self, kernel:&Vec<Vec<f64>>) -> Vec<f64>{
        let kernel:Vec<Vec<f64>> = kernel
            .iter()
            .rev()
            .map(|v|v.iter().map(|x|*x).rev().collect()).collect();
        let flat_data:Vec<f64> = self.data.iter().flatten().map(|x|*x).collect();
        let flat_kernel:Vec<f64> = kernel.iter().flatten().map(|x|*x).collect();
        let data = DynamicMatrix::new(self.x_pixels, self.y_pixels, flat_data).unwrap();
        let kernel = DynamicMatrix::new(kernel[0].len(), kernel.len(), flat_kernel).unwrap();
        let output = convolve2d(&data, &kernel);
        output.get_data().to_vec()
    }






}


