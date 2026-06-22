use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::grid2d::GRID2D;

pub mod psf;
pub mod psf_grid;
pub mod detector;
mod uvex_telescope;
pub mod point_sources;
pub mod spatial_effect;

fn main() {
    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let flatfield = "/Users/mayabasu/Desktop/uvex_psf_files/FUV_flat_field_illumination.fits";
    uvex_telescope::UVEX::initialize(
        fuv_path,flatfield
    )


    /*
    //point_source::PointSource::new_AB("sdf",1.0);
    let fuv = empty_fuv();
    let mut fuv_psf_grid = psf_grid::PsfGrid::new("fuvpsf", fuv,fuv_path, ("XFLD", "YFLD"));
    let psf_grid = GRID2D::new_empty((64,64),(1.0,1.0),(0.0,0.0),)
    fuv_psf_grid.load_data_frames(fuv_path, , (64, 64), (6.4, 6.4));
    use rand_distr::{Binomial, Distribution};

     */


}


