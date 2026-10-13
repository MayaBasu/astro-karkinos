
use clap::Parser;
use egui::CursorIcon::Default;
use plotpy::Plot;
use crate::geometry::{Grid1DUnits, Regular};
use crate::telescope::{Detector, PsfGrid, SpatialEffect, SpectralResponse, Spectrum};
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
    config.fuv_dead_pixels.0 = off;
    config.fuv_dark_current.0 = off;
    config.fuv_read_noise.0 = off;

    let mut uvex = UVEX::load(config);


    let data = vec![vec![0.1,50.,100.],
                    vec![0.1,50.,100.],
                    vec![0.1,50.,100.]];
    let background = SpatialEffect::new(uvex.detector_layout,data).unwrap();

    uvex.run(background, Channel::FUV, vec![], 10.0);


   // let mut plot = Plot::new();

   // uvex.detector_layout.plot(&mut plot, "green", "layout");
    //background_grid.plot(&mut plot, "red", "layout");
    //UVEX::psf_grid().plot(&mut plot, "red", "grid");
   // plot.show("hiekd").unwrap();






}






