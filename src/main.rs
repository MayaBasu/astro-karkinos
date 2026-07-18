use std::path::PathBuf;
use std::str::FromStr;
use convolve2d::Matrix;
use astroimsim_data::prelude::{Load, PSF};
use astroimsim_data::psf::Load::FromValue;

pub mod psf;
pub mod psf_grid;
pub mod detector;
pub mod point_sources;
pub mod spatial_effect;
pub mod test;



pub fn main() {

    let psf = PSF::load_file(PathBuf::from_str("/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF/UVEX_FUV_PSF_1um_F222.fits").unwrap(),
                             (FromValue(0.0), FromValue(0.0)),(FromValue(1.0), FromValue(1.0)),64,64);
  //  println!("psf data {:?}",psf.data);
    psf.write_file("/Users/mayabasu/Desktop/blurred_psf/lskejf.fits",("XPOS","YPOS"))
    /*
    let kernel = vec![
        vec![9.0,-1.0,2.0],
        vec![-1.0,8.0,1.0],
        vec![1.0,1.0,4.0],
    ];

     */



}




