use std::path::absolute;
use crate::gridded_data::units::*;
use crate::geometry::{Point, ABSOLUTE_COORDINATES, GRID1D, GRID2D};
use crate::geometry::Grid1DUnits::nm;
use crate::telescope::{Input, SpatialEffect, Spectrum};
use crate::uvex::config::Config;
use crate::uvex::uvex::{Channel, UVEX};

pub fn test(){
    let mut config = Config::mini();
    let spectral_grid = GRID1D::new(1000,100.,1000.,0.0001,nm);
    let mut uvex = UVEX::load(config.clone());
    let grid = GRID2D::new([100,100],[1.,1.],Point::zero(&ABSOLUTE_COORDINATES),0.0001,ABSOLUTE_COORDINATES);
    let background = SpatialEffect::constant(grid,0.).unwrap();
    let input = Input{
        points: vec![Point::new(-0.56, -0.06,&ABSOLUTE_COORDINATES)],
        weights: vec![1.],
        spec_refs: vec![0],
        spectra: vec![Spectrum::new_flat(spectral_grid, 1.0, SpectralUnits::f_lambda)],
    };

    uvex.run(background, Channel::NUV, input);

}