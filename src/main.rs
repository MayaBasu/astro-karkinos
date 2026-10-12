
use clap::Parser;
use egui::CursorIcon::Default;
use crate::geometry::{Grid1DUnits, Regular};
use crate::telescope::{Detector, PsfGrid, SpatialEffect, SpectralResponse, Spectrum};
use crate::geometry::{generate_notebook, CoordinateSystem, Point, ABSOLUTE_COORDINATES, GRID1D, GRID2D};
use crate::geometry::Grid1DUnits::nm;
use crate::gridded_data::{DATA1D, DATA2D};
use crate::gridded_data::DetectorUnits::{AverageElectrons, Electrons};
use crate::uvex::demo::{compare, demo};

pub mod geometry;
pub mod uvex;
pub mod telescope;
pub mod gridded_data;




pub fn main() {


    let grid = GRID2D::new([2,2],[1.,1.],
    Point::new(0.1,-0.1,&ABSOLUTE_COORDINATES),0.001,ABSOLUTE_COORDINATES);

    for point in grid.locate_grid_points(){
        for point in point{
            println!("{:?}", (point.x.value(),point.y.value()))
        }
    }
    let detectorgrid = GRID2D::new([5,5],[1.,1.],
                           Point::new(0.,0.,&ABSOLUTE_COORDINATES),0.001,ABSOLUTE_COORDINATES);

    let corner = detectorgrid.locate(0);
    //println!("{:?}", grid.is_point_inside_cells(&corner));
    let effect = SpatialEffect::new(grid, vec![vec![1.,2.],vec![1.,2.]]).unwrap();


    let grid = GRID2D::new([2,2],[1.,1.],
                           Point::new(2.0,1.0,&ABSOLUTE_COORDINATES),0.001,ABSOLUTE_COORDINATES);


    let effect2 = SpatialEffect::new(grid, vec![vec![1.,2.],vec![3.,4.]]).unwrap();

    let mut detector = Detector::initialize(detectorgrid,Electrons);
    //println!("{:?}",detector.data);
    detector.add(&effect);
    //detector.add(&effect2);
    detector.write_to_fits("tester.fits");


    //effect.unwrap().write_to_fits("test.fits");





    //let path = "/Users/mayabasu/PycharmProjects/make_inputs/inputs/blackbody_2000";

  //  let input_spectrum = Spectrum::load(path,0.0001).expect("Failed to parse file");


    //compare(input_spectrum);

    //let sums = demo("/Users/mayabasu/PycharmProjects/make_inputs/inputs/blackbody_6000",6000);


    //println!("2000, before: {:?}",sums.0);
    //println!("2000, after: {:?}",sums.1);
    let sums = demo("/Users/mayabasu/PycharmProjects/make_inputs/inputs/blackbody_2000",2000);
    //println!("6000, before: {:?}",sums.0);
    //println!("6000, after: {:?}",sums.1);



    // ([4.895859787633742e-7, 0.00011106758732255989, 0.002547734183133085, 0.08406616582973903, 0.9132745428138266], [0.012604229661030296, 0.00391622271841477, 0.05142988791885012, 0.15290649797431102, 0.7791431617273937])
    //out of band
    // ([4.895859787633742e-7, 0.00011106758732255989, 0.002547734183133085, 0.08406616582973903, 0.9132745428138266], [0.012604229661030296, 0.00391622271841477, 0.05142988791885012, 0.15290649797431102, 0.7791431617273937])

    //in band
    // ([0.02362058240590498, 0.08267456689505587, 0.13850333035228973, 0.33208669046516043, 0.4231148298815891], [0.9946769158904017, 0.004014553483792915, 0.0008042865859994181, 0.0004137413733034587, 9.050266650235897e-5])

}






