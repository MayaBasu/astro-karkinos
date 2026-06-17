use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_data::point_source;

pub mod psf;
pub mod psf_grid;


fn main() {
    //point_source::PointSource::new_AB("sdf",1.0);
    let fuv_path = "/Users/mayabasu/Desktop/uvex_psf_files/FUV PSF";
    let fuv = empty_fuv();
    let mut fuv_psf_grid = psf_grid::PsfGrid::new(fuv);
    fuv_psf_grid.load_data_frames(fuv_path, ("XFLD", "YFLD"), (64, 64), (6.4, 6.4));
    use rand_distr::{Binomial, Distribution};

    
}


pub fn empty_fuv() -> GRID2D {

    let coord = CoordinateSystem{
        x_axis: (1.0,0.0),
        y_axis: (0.0,1.0),
        center: (0.0, 0.0),
        color: "red".to_string(),
        label: "fuv".to_string(),
    };
    let mut grid = GRID2D::new_empty(
        (18,18), //x_num
        (0.2,0.2), //x_step_size
        (-0.56, -0.06), //y_num
        0.1, //y_step_size
        Coordinates::RELATIVE(coord)
    );
    grid.label = "fuv".to_string();
    grid
}