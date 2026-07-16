use std::path::PathBuf;
use std::str::FromStr;
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
    let kernel = vec![
        vec![0.0,-1.0,0.0],
        vec![-1.0,0.0,1.0],
        vec![0.0,1.0,0.0],
    ];
    let output = psf.convolve(kernel);
    println!("{:?}", output)
}




