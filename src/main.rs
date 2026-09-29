
use clap::Parser;
use crate::telescope::PsfGrid;
use crate::geometry::{generate_notebook, CoordinateSystem, Point, ABSOLUTE_COORDINATES, GRID2D};

pub mod geometry;

pub mod telescope;
pub mod gridded_data;




pub fn main() {
   // test_coordinate_system();
    let mut grid = GRID2D::new(
        [18,18],
        [0.2,0.2],
        Point::new(-0.56, -0.06,&ABSOLUTE_COORDINATES),
        0.1,
        ABSOLUTE_COORDINATES
    );


    let mut nuv_psf = PsfGrid::new(
        "test".to_string(),
        grid,
        "/Users/mayabasu/Desktop/NUV Out-of-Band-2/300 nm".to_string(),
        ("XFLD".to_string(), "YFLD".to_string()));
    
    nuv_psf.load_data_frames(128,128);


}






