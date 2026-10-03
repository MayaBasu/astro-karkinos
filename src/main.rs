
use clap::Parser;
use crate::geometry::{Grid1DUnits, Regular};
use crate::telescope::{Detector, PsfGrid, SpatialEffect, SpectralResponse, Spectrum};
use crate::geometry::{generate_notebook, CoordinateSystem, Point, ABSOLUTE_COORDINATES, GRID1D, GRID2D};
use crate::gridded_data::DATA2D;
use crate::gridded_data::DetectorUnits::{AverageElectrons, Electrons};

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

    let grid = GRID2D::new(
        [128, 128],
        [1.0,1.0] ,
        Point::new(0.0,0.0,&ABSOLUTE_COORDINATES),
        0.01,
        ABSOLUTE_COORDINATES);


    let mut psf_300 = PsfGrid::new(
        "test".to_string(),
        grid,
        "/Users/mayabasu/Desktop/NUV Out-of-Band-2/300 nm".to_string(),
        ("XFLD".to_string(), "YFLD".to_string()));

    nuv_psf.load_data_frames(128,128);

    let grid = GRID2D::new(
        [128, 128],
        [1.0,1.0] ,
        Point::new(0.0,0.0,&ABSOLUTE_COORDINATES),
        0.01,
        ABSOLUTE_COORDINATES);



    let mut psf_1000 = PsfGrid::new(
        "test".to_string(),
        grid,
        "/Users/mayabasu/Desktop/NUV Out-of-Band-2/300 nm".to_string(),
        ("XFLD".to_string(), "YFLD".to_string()));

    nuv_psf.load_data_frames(128,128);

    let grid = GRID2D::new(
        [128, 128],
        [1.0,1.0] ,
        Point::new(0.0,0.0,&ABSOLUTE_COORDINATES),
        0.01,
        ABSOLUTE_COORDINATES);




    let psf = nuv_psf.grid_psf(10);
    let psf = SpatialEffect::new(grid,
                                 psf);
    psf.unwrap().write_to_fits("test.fits");


    let grid = GRID2D::new(
        [3,3],
        [1.0,1.0] ,
        Point::new(0.0,0.0,&ABSOLUTE_COORDINATES),
        0.01,
        ABSOLUTE_COORDINATES);










    let grid2 = GRID2D::new(
        [2,2],
        [400.0,400.0] ,
        Point::new(0.0,0.0,&ABSOLUTE_COORDINATES),
        0.0001,
        ABSOLUTE_COORDINATES);

    let grid = GRID2D::new(
        [400,400],
        [1.,1.] ,
        Point::new(0.0,0.0,&ABSOLUTE_COORDINATES),
        0.0001,
        ABSOLUTE_COORDINATES);


    let background = SpatialEffect::new(grid2,
                                        vec![vec![0.,1.],
                                             vec![0.3,1.2]]).unwrap();


    let background = SpatialEffect::new(grid, background.re_grid_values(grid));
    background.unwrap().write_to_fits("backgroundtest.fits");


    let grid3 = GRID2D::new(
        [3,3],
        [200.,200.] ,
        Point::new(0.0,0.0,&ABSOLUTE_COORDINATES),
        0.0001,
        ABSOLUTE_COORDINATES);



    let background = SpatialEffect::new(grid3,
                                        vec![vec![0.,3.,1.],
                                             vec![0.3,3.,1.2],vec![0.3,3.,1.2]]).unwrap();


    let background = SpatialEffect::new(grid, background.re_grid_values(grid));
    background.unwrap().write_to_fits("backgroundteste.fits")
















}






