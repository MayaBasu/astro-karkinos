use astroimsim_data::point_source;

pub mod poisson;
pub mod psf;
pub mod psf_grid;

pub mod units;

pub mod datagrids;
pub mod datafile;


fn main() {
    point_source::PointSource::new_AB("sdf",1.0);
}