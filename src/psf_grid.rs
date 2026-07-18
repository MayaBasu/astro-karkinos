use std::fs;
use astroimsim_geometry::coordinate_system::CoordinateSystem;
use astroimsim_geometry::grid2d::{Corners, GRID2D};
use astroimsim_geometry::points::Point;
use egui::emath::interpolation_factor;
use uvex_fitrs::{Fits, Hdu};
use crate::psf::{DataFile, PSF, Load};

#[derive(Debug)]
pub struct PsfGrid {
    label:&'static str,
    grid: GRID2D,
    data:Vec<(usize,PSF)>,
    valid: bool,
    directory_path:&'static str,
    center_fits_keys:(&'static str, &'static str),

}

impl PsfGrid{
    pub fn new(label:&'static str,grid: GRID2D,directory_path:&'static str,center_fits_keys:(&'static str, &'static str)) -> PsfGrid{
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
        let paths = fs::read_dir(self.directory_path).unwrap();
        let mut counter = 0;
        for path in paths {
            println!("loading {:?}",path);
            counter += 1;
            let path = path.unwrap().path();

            let frame = PSF::load_file(
                path,
                (Load::FromKey(self.center_fits_keys.0.to_string()), Load::FromKey(self.center_fits_keys.1.to_string())),
                (Load::FromValue(self.grid.x_size),Load::FromValue(self.grid.y_size)),
                x_num,y_num,
            );
            let frame_index = frame.snap_to_grid(&self.grid);
            data.push((frame_index,frame))
        }
        data.sort_by_key(|x|x.0);
        self.data = data;
        println!("Loaded {counter} files into a Grid Struct from {:?}",self.directory_path);
        assert_eq!(counter,self.grid.num_points,"Loaded the wrong number of PSF files");
    }



    pub fn validate(&mut self) -> bool {
        //check to make sure there are the same number of data frames as there are grid points
        if self.data.len() != self.grid.num_points{
            println!("Validation failed: expected {:?} data frames, have {:?}",self.grid.num_points,self.data.len());
            self.valid = false;
            return false
        };
        //check to make sure that every grid point has a data frame
        let mut missing = Vec::new();
        let mut counter = 0;
        for grid_number in 0..self.grid.num_points{
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


    pub fn gaussian_blur(&self, blurred_directory: &'static str, std_in_pixels:f64){
        let mut new_grid = PsfGrid::new("blurred grid", self.grid.clone(), blurred_directory, self.center_fits_keys.clone());
        let kernel = convolve2d::kernel::gaussian(std_in_pixels.ceil() as usize*10,std_in_pixels);
        let (x,y,vec) = kernel.into_parts();
        let square_kernel = vec.chunks_exact(x).map(|x|x.to_vec()).collect();
        for (i,psf) in &self.data{
            let output = psf.convolve(&square_kernel);
            let path = format!("{:?}/{i}_blurred_{std_in_pixels}.fits",self.directory_path);
            let new_psf = PSF{
                path: path.parse().unwrap(),
                data: output.chunks_exact(psf.x_pixels).map(|x|x.to_vec()).collect(),
                x_pixels: psf.x_pixels,
                y_pixels: psf.y_pixels,
                center: psf.center.clone(),
                size: psf.size.clone()
            };
            new_grid.data.push((*i,new_psf));
            psf.write_file(path.as_str(), self.center_fits_keys)

        }

    }

}



