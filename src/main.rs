
use clap::Parser;
use egui::CursorIcon::Default;
use plotpy::Plot;
use crate::gridded_data::SpectralUnits;
use crate::geometry::{Grid1DUnits, Regular, SPECTRAL_GRID};
use crate::telescope::{make_input, Detector, Input, PsfGrid, SpatialEffect, SpectralResponse, Spectrum};
use crate::geometry::{generate_notebook, CoordinateSystem, Point, ABSOLUTE_COORDINATES, GRID1D, GRID2D};
use crate::geometry::Grid1DUnits::{angstroms, nm, um};
use crate::gridded_data::{DATA1D, DATA2D};
use crate::gridded_data::DetectorUnits::{AverageElectrons, Electrons};
use crate::uvex::config::Config;
use crate::uvex::config::Toggle::off;
use crate::uvex::psf_bin_test;
use crate::uvex::uvex::{Channel, UVEX};

pub mod geometry;
pub mod uvex;
pub mod telescope;
pub mod gridded_data;




pub fn main() {

    let x = 1.;
    println!("{:?} in um",x);
    println!("{:?} in um",x*um.to_um());
    println!("{:?} in a",x*um.to_angstrom());
    println!("{:?} in nm",x*um.to_nm());

    println!("{:?} in um",x*um.to_angstrom()*angstroms.to_um());
    println!("{:?} in um",x*um.to_nm()*nm.to_um());
    println!("{:?} in um",x*nm.to_angstrom()*angstroms.to_nm());
    println!("{:?}",nm.conversion_factor(angstroms));
    println!("{:?}",nm.conversion_factor(um));
    println!("{:?}",um.conversion_factor(nm));
    println!("{:?}",um.conversion_factor(angstroms));
    println!("{:?}",angstroms.conversion_factor(um));
    println!("{:?}",angstroms.conversion_factor(nm));





    let config = Config::default();
    let mut uvex = UVEX::load(config.clone());

    let spectrum = Spectrum::load("/Users/mayabasu/Desktop/onestar/flat.dat", 0.0001).unwrap();
    let spectrum = spectrum.regrid(&SPECTRAL_GRID);
    let small = GRID1D::new(1000,100.,1000.,0.0001,nm);

    println!("spectruma after: {:?}", spectrum.apply_spectral_response(&uvex.nuv).regrid(&small));

    println!("COUNTRATE: {:?}",spectrum.countrate(&uvex.nuv.regrid(&SPECTRAL_GRID)))







    //psf_bin_test::test();

    /*

    let mut config = Config::default();
    config.nuv_dead_pixels.0 = off;
    config.nuv_dark_current.0 = off;
    config.nuv_read_noise.0 = off;
    config.nuv_flatfield_illumination.0 = off;
    let norminput = make_input("/Users/mayabasu/Desktop/testcat/field.dat",
                               "/Users/mayabasu/Desktop/testcat/parsed");
    let mut plot = Plot::new();


    let mut uvex = UVEX::load(config.clone());
    let spec0 = Spectrum::load("/Users/mayabasu/Desktop/testcat/zodiac2.dat",0.0001);
    let spec1 = Spectrum::load("/Users/mayabasu/Desktop/testcat/galactic2.dat",0.0001);

    println!("making background");
    let background = uvex.make_nuv_background([vec![1.0275712,0.53212076],
        vec![1.0267771,0.4238424],
        vec![0.97468865,2.0321445],
        vec![0.9709286,1.0121843]], vec![spec0.unwrap(),spec1.unwrap()]);
    println!("BACKGROUND DATA = {:?}", background);

    let background_grid = GRID2D::new(
        [2,2],
        [4.,4.],
        Point::new(-0.56, -0.06, &ABSOLUTE_COORDINATES),
        0.000000000001,
        ABSOLUTE_COORDINATES);


    let background = SpatialEffect::constant(background_grid,0.).unwrap();

    let one_star_input = make_input("/Users/mayabasu/Desktop/onestar/field.dat",
    "/Users/mayabasu/Desktop/onestar/parsed");


    /*

    let psf_grid:PsfGrid = uvex.nuv_psfs[0].clone();
    psf_grid.gaussian_blur("/Users/mayabasu/Desktop/NUV Out-of-Band-2/300nmblur_100mas".to_string(), 0.1*10./config.clone().arcsec_per_pixel);
    let psf_grid:PsfGrid = uvex.nuv_psfs[1].clone();
    psf_grid.gaussian_blur("/Users/mayabasu/Desktop/NUV Out-of-Band-2/400nmblur_100mas".to_string(), 0.1*10./config.clone().arcsec_per_pixel);
    let psf_grid:PsfGrid = uvex.nuv_psfs[2].clone();
    psf_grid.gaussian_blur("/Users/mayabasu/Desktop/NUV Out-of-Band-2/500nmblur_100mas".to_string(), 0.1*10./config.clone().arcsec_per_pixel);
    let psf_grid:PsfGrid = uvex.nuv_psfs[3].clone();
    psf_grid.gaussian_blur("/Users/mayabasu/Desktop/NUV Out-of-Band-2/700nmblur_100mas".to_string(), 0.1*10./config.clone().arcsec_per_pixel);
    let psf_grid:PsfGrid = uvex.nuv_psfs[4].clone();
    psf_grid.gaussian_blur("/Users/mayabasu/Desktop/NUV Out-of-Band-2/1000nmblur_100mas".to_string(), 0.1*10./config.clone().arcsec_per_pixel);

     */



    uvex.run(background, Channel::NUV, one_star_input);
    for detector in uvex.detectors{
        detector.grid.plot_outline_2(&mut plot, "red".to_string(), "det".to_string())
    }
    plot.show("sekjhfsef").unwrap();

     */




}






