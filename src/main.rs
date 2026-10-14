
use clap::Parser;
use egui::CursorIcon::Default;
use plotpy::Plot;
use crate::geometry::{Grid1DUnits, Regular};
use crate::telescope::{make_input, Detector, Input, PsfGrid, SpatialEffect, SpectralResponse, Spectrum};
use crate::geometry::{generate_notebook, CoordinateSystem, Point, ABSOLUTE_COORDINATES, GRID1D, GRID2D};
use crate::geometry::Grid1DUnits::nm;
use crate::gridded_data::{DATA1D, DATA2D};
use crate::gridded_data::DetectorUnits::{AverageElectrons, Electrons};
use crate::uvex::config::Config;
use crate::uvex::config::Toggle::off;
use crate::uvex::uvex::{Channel, UVEX};

pub mod geometry;
pub mod uvex;
pub mod telescope;
pub mod gridded_data;




pub fn main() {

    let mut config = Config::default();

    config.nuv_dead_pixels.0 = off;
    config.nuv_dark_current.0 = off;
    config.nuv_read_noise.0 = off;
    config.nuv_flatfield_illumination.0 = off;
    //println!("{:?}", config);


/*
    let input = make_input(
        "/Users/mayabasu/Desktop/catalogs/field.dat",
    "/Users/mayabasu/Desktop/catalogs/parsed");

 */
    let spectrum = GRID1D::new(1000, 100.0, 1000.0, 0.0000001, Grid1DUnits::nm);
let points = vec![Point::new(-0.5,0.0,&ABSOLUTE_COORDINATES)];
    let weights = vec![1.];
    let spec_refs = vec![0];
    let spectra = vec![Spectrum::new_flat(spectrum,1.0,crate::gridded_data::units::SpectralUnits::f_lambda)];
    let input = Input{
        points,
        weights,
        spec_refs,
        spectra,
    };




    let mut plot = Plot::new();




    for i in 0..input.points.len(){

        if i %20 == 0{
            let point:Point = input.points[i];
            point.plot(&mut plot, "red", "");
        }



    }














    let background_values = vec![vec![0.,0.],
                                 vec![0.,0.]];
    let background_grid = GRID2D::new(
        [2,2],
        [4.,4.],
        Point::new(-0.56, -0.06, &ABSOLUTE_COORDINATES),
        0.000000000001,
        ABSOLUTE_COORDINATES);

    let background = SpatialEffect::new(background_grid, background_values).unwrap();



    let mut uvex = UVEX::load(config);

   // let psf_grid:PsfGrid = uvex.nuv_psfs[0].clone();
    //psf_grid.gaussian_blur("/Users/mayabasu/Desktop/NUV Out-of-Band-2/300nmblur".to_string(), 20.0);
    //println!("{:?}", uvex.config);

    uvex.run(background, Channel::NUV, input, 10.0);

    for detector in uvex.detectors{
        detector.grid.plot_outline_2(&mut plot, "red".to_string(), "det".to_string())
    }
    plot.show("sekjhfsef").unwrap();



    //let mut plot = Plot::new();









    /*

    let grid1 = GRID2D::new(
    [2,2],
    [1.,1.],
    Point::zero(&ABSOLUTE_COORDINATES),
    0.000001,
    ABSOLUTE_COORDINATES,
    );

    let data_1 = vec![vec![1.,2.],vec![1.,2.]];
    let data_grid_1 = SpatialEffect::new(grid1, data_1).unwrap();
    println!("{:?}", data_grid_1.data());

    let grid2 = GRID2D::new(
        [3,3],
        [0.5,0.5],
        Point::zero(&ABSOLUTE_COORDINATES),
        0.000001,
        ABSOLUTE_COORDINATES,
    );

    let regridded = data_grid_1.re_grid_values(grid2);
    println!("{:?}", regridded);

     */



    /*


   // config.fuv_dead_pixels.0 = off;
   // config.fuv_dark_current.0 = off;
   // config.fuv_read_noise.0 = off;

   let mut config = Config::default();

    let mut uvex = UVEX::load(config);


    let background_values = vec![vec![1.,1.],
                    vec![1.,1.]];
    let background_grid = GRID2D::new(
        [2,2],
        [4.,4.],
        Point::new(-0.56, -0.06, &ABSOLUTE_COORDINATES),
        0.000000000001,
        ABSOLUTE_COORDINATES);

    let background = SpatialEffect::new(background_grid, background_values).unwrap();

    


    uvex.run(background, Channel::FUV, vec![], 10.0);


    let mut plot = Plot::new();

    uvex.detector_layout.plot(&mut plot, "green", "layout");
    background_grid.plot(&mut plot, "purple", "background_grid");
    UVEX::psf_grid().plot(&mut plot, "red", "grid");
    for detector in uvex.detectors{
        detector.grid.plot_outline_2(&mut plot, "orange".to_string(), "detector".to_string())
    }
    plot.show("hiekd").unwrap();

     */








}






