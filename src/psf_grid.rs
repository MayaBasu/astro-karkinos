use std::fs;
use astroimsim_geometry::coordinate_system::CoordinateSystem;
use astroimsim_geometry::grid2d::{Corners, GRID2D};
use astroimsim_geometry::points::Point;
use crate::psf::{DataFile, PSF, Load};


pub struct PsfGrid {
    data: Vec<(usize,PSF)>,
    pub(crate) grid: GRID2D,
    valid: bool
}

impl PsfGrid{
    pub fn new(grid: GRID2D) -> PsfGrid{
        PsfGrid{
            data: vec![],
            grid: grid,
            valid:false,

        }
    }
    pub fn load_data_frames(&mut self, directory_path:&str, center_fits_keys:(&str, &str), pixels:(usize, usize), size:(f64, f64)){
        println!("Loading data frames into grid. This overwrites any data previously loaded");
        let mut data = vec![];
        let paths = fs::read_dir(directory_path).unwrap();
        let mut counter = 0;
        for path in paths {
            println!("loading {:?}",path);
            counter += 1;
            let path = path.unwrap().path();
            let (x_pixels,y_pixels) = pixels;
            let data_file = DataFile{
                description: counter.to_string(),
                path,
                x_pixels,
                y_pixels,
            };
            let frame = PSF::load_file(data_file,
                                       (Load::FromKey(center_fits_keys.0.to_string()), Load::FromKey(center_fits_keys.1.to_string())),
                                       (Load::FromValue(size.0),Load::FromValue(size.0)));
            let frame_index = frame.snap_to_grid(&self.grid);
            data.push((frame_index,frame))
        }
        data.sort_by_key(|x|x.0);
        self.data = data;
        println!("Loaded {counter} files into a Grid Struct from {directory_path}");
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


    pub fn interpolated_psf(&self, point:&Point) -> Vec<Vec<f32>>{
        //println!("Converting from {:?}", point);

        //  println!("To {:?}",(x,y));

        let ((Q12, Q22, Q21, Q11),(c11,c12,c21,c22),normalization) = self.interpolation_coefficients(point.clone());

        let q11 = self.data[Q11].clone();
        let q12 = self.data[Q12].clone();
        let q21 = self.data[Q21].clone();
        let q22 = self.data[Q22].clone();

        assert_eq!(q11.0,Q11);
        assert_eq!(q12.0,Q12);
        assert_eq!(q21.0,Q21);
        assert_eq!(q22.0,Q22);

        let interpolated_data:Vec<f32> =
            q11.1.data.into_iter().flatten().zip(
                q12.1.data.into_iter().flatten().zip(
                    q21.1.data.into_iter().flatten().zip(
                        q22.1.data.into_iter().flatten()))).map(
                |(q11,(q12,(q21,q22)))| {
                    (q11*c11 as f32 + q12*c12 as f32 + q21*c21 as f32 + q22*c22 as f32)/normalization as f32
                }).collect();
        PSF::repack_data(interpolated_data)



    }



    pub fn interpolation_coefficients(&self, point:Point) -> ((usize,usize,usize,usize),(f64,f64,f64,f64),f64){
        if self.valid == false{
            panic!("Must validate Grid before attempting to interpolate")
        };

        match self.grid.find_corners(point.clone()){
            Corners::Four(Q12, Q22, Q21, Q11) => { // using the wikipedia convention https://en.wikipedia.org/wiki/Bilinear_interpolation

                let Q11point = self.grid.locate(Q11);
                let Q22point = self.grid.locate(Q22);


                let (x1,y1) = (Q11point.x,Q11point.y);
                let (x2,y2) = (Q22point.x,Q22point.y);
                let (x,y) = point.convert(&self.grid.coordinates).values();
                let c11 = (x2-x)*(y2-y);
                let c12 = (x2-x)*(y-y1);
                let c21 = (x-x1)*(y2-y);
                let c22 = (x-x1)*(y-y1);
                println!("The coefficients are {:?}",(c11,c12,c21,c22));
                let normalization = (x2-x1)*(y2-y1);
                ((Q12, Q22, Q21, Q11),(c11,c12,c21,c22),normalization)

            }
            Corners::Two(Q1, Q2) => {((Q1, Q2, Q1, Q2),(1.0,1.0,1.0,1.0),1.0)} //TODO !!!!
            Corners::One(Q1) => {((Q1, Q1, Q1, Q1),(1.0,1.0,1.0,1.0),1.0)}
        }

    }

    pub fn grid_psf(&self, index:usize)-> Vec<Vec<f32>>{
        let (i, psf) = self.data[index].clone();
        assert_eq!(index,i);
        psf.data

    }

}
