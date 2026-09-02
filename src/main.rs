use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use convolve2d::Matrix;

use astroimsim_data::prelude::{Load, PSF};
use astroimsim_data::psf::Load::FromValue;
use crate::detector::{Detector, EffectType};

use crate::spatial_effect::SpatialEffect;

pub mod psf;

pub mod detector;
pub mod point_sources;
pub mod spatial_effect;

pub mod geometry;
pub mod grid1d;
pub mod grid2d;
pub mod tests;
pub mod notebook;

pub mod power_spectrum;
pub mod spectral_response;




pub fn main() {
    /*

    let coord = CoordinateSystem{
        x_axis: (1.0,0.0),
        y_axis: (0.0,1.0),
        center: (0.0, 0.0),
        label: "fuv",
    };
    let mut grid = GRID2D::new_empty(
        (18,18), //x_num
        (0.2,0.2), //x_step_size
        (-0.56, -0.06), //y_num
        0.1, //y_step_size
        Coordinates::RELATIVE(coord)
    );
    grid.label = "fuv".to_string();


    let mut fuv_psf = PsfGrid::new(
        "FUV PSF grid".to_string(),
        grid,
        "/Users/mayabasu/Desktop/uvex/FUV_PSF".to_string(),
        ("XFLD".to_string(), "YFLD".to_string()));
    fuv_psf.load_data_frames(64,64);

     */



    
    /*
    let grid = GRID2D::new_empty((3,3),(1.0,1.0),(0.0,0.0),0.01,Coordinates::ABSOLUTE);
    let mut detector = Detector::new("test".to_string(),grid.clone());
    detector.create_constant_background(2.0,1.0);

    let effect = SpatialEffect::from_matrix("test".to_string(),grid,"N/A".to_string(),
                                            vec![vec![1.0,2.0,3.0],
                                                 vec![5.0,2.0,1.0],
                                                 vec![0.0,9.0,1.0]
                                            ]);
    detector.multiply_effect(effect,0,EffectType::Once);
    detector.write(0);
    
     */



/*
    let psf = PSF::load_file(PathBuf::from_str("/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF/UVEX_FUV_PSF_1um_F222.fits").unwrap(),
                             (FromValue(0.0), FromValue(0.0)),(FromValue(1.0), FromValue(1.0)),64,64);

 */

    //  println!("psf data {:?}",psf.data);
    /*
    psf.write_file("/Users/mayabasu/Desktop/blurred_psf/lskejf.fits",("XPOS","YPOS"))



    let kernel = vec![
        vec![9.0,-1.0,2.0],
        vec![-1.0,8.0,1.0],
        vec![1.0,1.0,4.0],
    ];

     */



}






