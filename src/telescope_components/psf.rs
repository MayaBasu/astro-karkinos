use std::fs;
use crate::geometry::*;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uvex_fitrs::{Fits, FitsData, FitsDataArray, Hdu, HeaderValue};
use ndarray::*;
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
        println!("loaded center");
        let size_x:f64 = PSF::load(size.0, &primary_hdu);
        let size_y:f64 = PSF::load(size.1, &primary_hdu);
        println!("loaded size");
        let data = data.chunks(x_num).map(|i| i.to_vec()).collect();
      //  println!("{:?}",data);
        PSF {
            path: file,
            data,
            x_pixels: x_num,
            y_pixels: y_num,
            center: Point::new(Regular::try_from(center_x),Regular::try_from(center_y),Coordinates::ABSOLUTE),
            size: (size_x,size_y),
        }
    }



    pub fn write_file(&self, path:&str,center_keys:(String,String)){

        let data:Vec<f64> = self.data.clone().iter().map(|x|x.to_owned()).flatten().collect();
        let data= data.iter().map(|x| *x).collect();
        println!("Trying to write to {path}, {:?}",data);
        let mut primary_hdu = Hdu::new(&[64, 64], data);
        println!("Done making hdu");
        let (x,y) = self.center.as_absolute().values();
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
                    HeaderValue::CharacterString(string) => { string.parse().expect("FAILED TO PARSE string") }
                    HeaderValue::RealFloatingNumber(i) => {*i}
                    _ => {panic!("failed to parse header value")}

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


#[derive(Debug,Clone)]
pub struct PsfGrid {
    label:String,
    grid: GRID2D,
    data:Vec<(usize,PSF)>,
    valid: bool,
    directory_path:String,
    center_fits_keys:(String, String),

}

impl PsfGrid{
    pub fn new(label:String,grid: GRID2D,directory_path:String,center_fits_keys:(String, String)) -> PsfGrid{
        PsfGrid{
            label,
            data: vec![],
            grid: grid,
            valid:false,
            directory_path,
            center_fits_keys,
        }
    }
    pub fn load_data_frames(&mut self, x_num:usize,y_num:usize){



        println!("Loading data frames into grid. This overwrites any data previously loaded");
        let mut data = vec![];
        let paths = fs::read_dir(self.directory_path.clone()).unwrap();
        let mut counter = 0;
        for path in paths {
            println!("loading {:?}",path);
            counter += 1;
            let path = path.unwrap().path();

            let frame = PSF::load_file(
                path,
                (Load::FromKey(self.center_fits_keys.0.to_string()), Load::FromKey(self.center_fits_keys.1.to_string())),
                (Load::FromValue(self.grid.x_size()),Load::FromValue(self.grid.y_size())),
                x_num,y_num,
            );

            let frame_index =frame.snap_to_grid(&self.grid);
            data.push((frame_index,frame))
        }
        data.sort_by_key(|x|x.0);
        self.data = data;
        println!("Loaded {counter} files into a Grid Struct from {:?}",self.directory_path);
        assert_eq!(counter,self.grid.num_points(),"Loaded the wrong number of PSF files");


    }



    pub fn validate(&mut self) -> bool {
        //check to make sure there are the same number of data frames as there are grid points
        if self.data.len() != self.grid.num_points(){
            println!("Validation failed: expected {:?} data frames, have {:?}",self.grid.num_points(),self.data.len());
            self.valid = false;
            return false
        };
        //check to make sure that every grid point has a data frame
        let mut missing = Vec::new();
        let mut counter = 0;
        for grid_number in 0..self.grid.num_points(){
            counter += 1;
            if !self.data.iter().any(|(index,_)| *index==grid_number) {
                missing.push(grid_number)
            }
        };
        if missing.len() > 0{
            println!("Validation failed! Missing data frames for the following grid positions: {:?} ",missing);
            self.valid = false;
            false
        }else{
            self.data.sort_by_key(|x|x.0);
            self.valid = true;
            true
        }}


    pub fn interpolated_psf(&self, point:&Point) -> Vec<Vec<f64>>{
        //println!("Converting from {:?}", point);

        //  println!("To {:?}",(x,y));
        let interpolation_data = self.grid.projected_interpolation_coefficients(&point.clone());

        match interpolation_data.corners{
            Corners::Four(Q12, Q22, Q21, Q11) => {
                let q11 = self.data[Q11].clone();
                let q12 = self.data[Q12].clone();
                let q21 = self.data[Q21].clone();
                let q22 = self.data[Q22].clone();

                let c11 = interpolation_data.coefficients[0];
                let c12 = interpolation_data.coefficients[1];
                let c21 = interpolation_data.coefficients[2];
                let c22 = interpolation_data.coefficients[3];

                assert_eq!(q11.0,Q11);
                assert_eq!(q12.0,Q12);
                assert_eq!(q21.0,Q21);
                assert_eq!(q22.0,Q22);

                let interpolated_data:Vec<f64> =
                    q11.1.data.into_iter().flatten().zip(
                        q12.1.data.into_iter().flatten().zip(
                            q21.1.data.into_iter().flatten().zip(
                                q22.1.data.into_iter().flatten()))).map(
                        |(q11,(q12,(q21,q22)))| {
                            (q11*c11  + q12*c12  + q21*c21  + q22*c22 )/interpolation_data.normalization
                        }).collect();
                PSF::repack_data(interpolated_data)

            }
            Corners::Two(Q1, Q2) => {
                let q1 = self.data[Q1].clone();
                let q2 = self.data[Q2].clone();

                let c1 = interpolation_data.coefficients[0];
                let c2 = interpolation_data.coefficients[1];


                assert_eq!(q1.0,Q1);
                assert_eq!(q2.0,Q2);


                let interpolated_data:Vec<f64> =
                    q1.1.data.into_iter().flatten().zip(
                        q2.1.data.into_iter().flatten()).map(
                        |(q1,q2)| {
                            (q1*c1 + q2*c2 )/interpolation_data.normalization
                        }).collect();
                PSF::repack_data(interpolated_data)

            }
            Corners::One(Q) => {
                self.data[Q].clone().1.data
            }
        }
    }




    pub fn grid_psf(&self, index:usize)-> Vec<Vec<f64>>{
        let (i, psf) = self.data[index].clone();
        assert_eq!(index,i);
        psf.data

    }


    pub fn gaussian_blur(&self, blurred_directory:String, std_in_pixels:f64){
        let mut new_grid = PsfGrid::new("blurred grid".to_string(), self.grid.clone(), blurred_directory.clone(), self.center_fits_keys.clone());
        let kernel = convolve2d::kernel::gaussian(std_in_pixels.ceil() as usize*10,std_in_pixels);
        let (x,y,vec) = kernel.into_parts();
        let square_kernel = vec.chunks_exact(x).map(|x|x.to_vec()).collect();
        for (i,psf) in &self.data{
            let output = psf.convolve(&square_kernel);
            let path = format!("{blurred_directory}/{i}_blurred_{std_in_pixels}.fits");
            let new_psf = PSF{
                path: path.parse().unwrap(),
                data: output.chunks_exact(psf.x_pixels).map(|x|x.to_vec()).collect(),
                x_pixels: psf.x_pixels,
                y_pixels: psf.y_pixels,
                center: psf.center.clone(),
                size: psf.size.clone()
            };
            new_grid.data.push((*i,new_psf.clone()));
            new_psf.write_file(path.as_str(), self.center_fits_keys.clone());

        }

    }

}






