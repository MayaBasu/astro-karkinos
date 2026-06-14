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
    /*

        pub fn downsample(&self, x_offset:f64, y_offset:f64, scale:f64)-> Vec<Vec<f32>>{
            //scale is the wdith of one big pixel in terms of little pixels
            //how many pixels from the lower corner is the corner of the psf
            //TODO PSF must be square
            assert_eq!(self.x_pixels % 2 , self.y_pixels % 2 );

            match (self.x_pixels % 2 == 0 ){
                true => {}
                false => {
                    //for an odd number of pixels in the psf the center of the psf and the center of the point
                }
            }

        }

     */
    /*

    pub fn calculate_pixel_locations(&self) {
        let indexes: Vec<Vec<(usize,usize)>> = self.data.iter().enumerate().map(|(y_index,row)|
            row.iter().enumerate().map(|(x_index, value)| (x_index,y_index)).collect::<Vec<(usize,usize)>>()).collect();
    }

     */
    /*

    pub fn convolve(&self,point_source: PointSource) -> SourceList{
        let bottom_left_corner = (self.center.0 - self.size.0/2.0,self.center.1 - self.size.1/2.0);
        let pixel_width = self.size.0/self.x_pixels as f64;
        let pixel_height = self.size.1/self.y_pixels as f64;
        let mut output_sources = SourceList::new_empty(self.x_pixels*self.y_pixels);
        for y_index in 0..self.y_pixels{
            for x_index in 0..self.x_pixels{
                let x = bottom_left_corner.0 + pixel_width*(x_index as f64 + 0.5);
                let y  = bottom_left_corner.1 + pixel_height*(y_index as f64 + 0.5);
                let value = self.data[y_index][x_index] as f64 * point_source.luminosity ;
                let source = PointSource::new(x,y,point_source.spectrum.clone(),value);
                output_sources.add_source(source);

            }
        }
        output_sources
    }

     */
}




