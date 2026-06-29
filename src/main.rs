use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::coordinate_system::Coordinates::ABSOLUTE;
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;
use astroimsim_spectra::power_spectrum::PowerSpectrum;
use astroimsim_spectra::power_spectrum::SpectrumUnits::f_lambda;
use crate::countrate_test::test_countrates;
use crate::point_sources::FullSpectrumPointSource;
use crate::uvex_telescope::UVEX;

pub mod psf;
pub mod psf_grid;
pub mod detector;
mod uvex_telescope;
pub mod point_sources;
pub mod spatial_effect;
pub mod countrate_test;
fn main() {


    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let flatfield = "/Users/mayabasu/Desktop/uvex_psf_files/FUV_flat_field_illumination.fits";
    let mut uvex = uvex_telescope::UVEX::initialize(fuv_path,flatfield);
    println!("{:?}",uvex.compare_flatfields(1000))
    /*

    let mut spectrum  = PowerSpectrum::flat_AB(20.0, UVEX::spectral_grid(), "Flat AB spectrum");
    let mut point_source = FullSpectrumPointSource::flat_AB(Point::new(0.0, 0.0, ABSOLUTE), 20.0, UVEX::spectral_grid());
    let (fuv,nuv) = UVEX::fuv_nuv();
    let bands = point_source.to_bands(&fuv,&nuv);
    let nuv = bands.nuv;
    let fuv= bands.fuv;
    println!("{:?}, {:?}", fuv, nuv)

     */
   // test_countrates()



/*
    let FOV = GRID2D::new_empty((2,2),(3.2,3.2),(-0.56, -0.06),0.01,ABSOLUTE);
    let sources = point_sources::FullSpectrumSourceList::full_spectrum_point_source_field(10000, 0.001, 1.0, spectrum, &FOV);

    let mut plot = plotpy::Plot::new();

    for detector in &uvex.detector_array.detectors{
        detector.grid.plot_outline(&mut plot, "purple");


    }

 */

   // plot.show("ksenf").expect("hHHHHHH");

   // uvex.run(sources)
    //println!("sources are {:?}",sources)





    /*
    //point_source::PointSource::new_AB("sdf",1.0);
    let fuv = empty_fuv();
    let mut fuv_psf_grid = psf_grid::PsfGrid::new("fuvpsf", fuv,fuv_path, ("XFLD", "YFLD"));
    let psf_grid = GRID2D::new_empty((64,64),(1.0,1.0),(0.0,0.0),)
    fuv_psf_grid.load_data_frames(fuv_path, , (64, 64), (6.4, 6.4));
    use rand_distr::{Binomial, Distribution};

     */


}


