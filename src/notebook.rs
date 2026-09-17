/*
use std::fs::{write, File};
use std::io::{BufWriter, Read, Write};
use crate::geometry::*;

use plotpy::Plot;
use markdown2pdf;
use markdown2pdf::config::ConfigSource;
use std::error::Error;
use markdown2pdf::fonts::FontConfig;
use markdown2pdf::styling::DocumentConfig;

pub fn generate_notebook() {
    let file = File::create("notebook.md").expect("Could not create notebook");
    let mut w = BufWriter::new(file);
    writeln!(w, "# RustSim Geometry Module").unwrap();

    writeln!(w, "The geometry module of the RustSim ecosystem handles coordinates, points in those coordinate systems, and regular one and two dimensional grids.\
     Within a telescope model, the geometry model serves the following purposes: \
    \n").unwrap();
    
    writeln!(w,"\
    1. Allows the conversion between a location in the spacecraft field of view (FOV) and a location on a detector plane. \
    This is needed to map a star from a location in the sky to a signal positioned on the detector and is a transformation effected by distorsions and reflections from optical components.\
    \n2. Points in these coordinate systems are needed to represent the locations of data such as simulated or measured point spread functions (PSFs).\
    \n3. Spectral data such as filter curves are loaded into one dimensional regular grids, and spatial data such as vinietting profiles and spatially varying PSFs are loaded onto two dimensional grids.\
    This framework of regular grids allows the interpolation of ray tracing data. For example, for a given star we can interpolate between known PSF shapes to estimate a PSF at the location of the star. ").unwrap();

    writeln!(w, "# RustSim Geometry Module").unwrap();

   w.flush().expect("jalsejkf");
    // Convert Markdown string to PDF with proper error handling
    let mut mkdn = String::new();
    let markdown = File::open("notebook.md").expect("trouble reading").read_to_string(&mut mkdn);

    println!("{:?} {:?}",mkdn, markdown);

    const EMBEDDED: &str = r#"
        [headings.h1]
        font_size_pt = 18.0
        font_weight = "bold"
    "#;

    let font_config = FontConfig::new()
        .with_default_font("Georgia");
    markdown2pdf::parse_into_file(mkdn.clone(), "output.pdf", ConfigSource::File("config.toml"),Some(&font_config)).expect("huhhh");




    //markdown2pdf::parse_into_file(mkdn, "output3.pdf", ConfigSource::Embedded(EMBEDDED), Some(&font_config)).expect("ahhhh");









}
pub fn test_coordiantes()-> (CoordinateSystem,CoordinateSystem){
    let mut plot = Plot::new();
    let basic_coordinates = CoordinateSystem::new([1.0,0.0],[0.0,1.0],[0.0,0.0]);
    let strange_coordinates = CoordinateSystem::new([0.7,0.3],[1.4,-1.3],[0.5,0.5]);
    basic_coordinates.plot(&mut plot, "orange".to_string(), "basic".to_string());
    strange_coordinates.plot(&mut plot, "purple".to_string(),"strange".to_string());
    plot.set_title("Defining and Plotting Coordinate Systems");
    plot.legend();
    plot.save("tests/coordinate_test").expect("could not save figure");
    (basic_coordinates,strange_coordinates)

}
pub fn test_grids(coords1:CoordinateSystem,coords2:CoordinateSystem)->(GRID2D,GRID2D){


    let mut plot = Plot::new();
    let grid1 = GRID2D::new_empty([10,10], [1.0,1.0], [0.0,0.0],0.001, Coordinates::RELATIVE(coords1.clone()));
    coords1.plot(&mut plot,"green".to_string());
    grid1.plot_outline(&mut plot,"green".to_string());
    grid1.plot_grid_points(&mut plot, PlotPoint::No);
    plot.set_title("Regular Grid on a Coordinate System");
    plot.legend();
    plot.save("tests/grid1_test").expect("could not save figure");



    let mut plot = Plot::new();
    let grid2 = GRID2D::new_empty([10,10], [1.0,1.0], [0.0,0.0],0.001, Coordinates::RELATIVE(coords2.clone()));
    coords2.plot(&mut plot, "purple".to_string());
    grid2.plot_outline(&mut plot, "purple".to_string());
    grid2.plot_grid_points(&mut plot, PlotPoint::No);
    plot.set_title("Regular Grid on a Coordinate System");
    plot.legend();
    plot.save("tests/grid2_test").expect("could not save figure");


    (grid1,grid2)
}

pub fn test_grids_with_points(coords1:CoordinateSystem,coords2:CoordinateSystem){

    let point_1 = Point::new(1.0,1.0,Coordinates::RELATIVE(coords1.clone()));
    let point_2 = Point::new(1.1,3.0,Coordinates::RELATIVE(coords1.clone()));
    let point_3 = Point::new(2.7,-2.5,Coordinates::RELATIVE(coords1.clone()));
    let point_4 = Point::new(-4.5,-2.5,Coordinates::RELATIVE(coords1.clone()));




    let mut plot = Plot::new();
    let grid1 = GRID2D::new_empty((10,10), (1.0,1.0), (0.0,0.0),0.001, Coordinates::RELATIVE(coords1.clone()));
    coords1.plot(&mut plot,"green".to_string());
    grid1.plot_outline(&mut plot,"green".to_string());
    grid1.plot_grid_points(&mut plot, PlotPoint::Given(point_1));
    grid1.plot_grid_points(&mut plot, PlotPoint::Given(point_2));
    grid1.plot_grid_points(&mut plot, PlotPoint::Given(point_3));
    grid1.plot_grid_points(&mut plot, PlotPoint::Given(point_4));

    plot.set_title("Regular Grid on a Coordinate System");
    //plot.legend();
    plot.save("tests/grid1_points_test").expect("could not save figure");



    let mut plot = Plot::new();

    let point_1 = Point::new(1.0,1.0,Coordinates::RELATIVE(coords2.clone()));
    let point_2 = Point::new(1.1,3.0,Coordinates::RELATIVE(coords2.clone()));
    let point_3 = Point::new(2.7,-2.5,Coordinates::RELATIVE(coords2.clone()));
    let point_4 = Point::new(-4.5,-2.5,Coordinates::RELATIVE(coords2.clone()));


    let grid2 = GRID2D::new_empty((10,10), (1.0,1.0), (0.0,0.0),0.001, Coordinates::RELATIVE(coords2.clone()));
    coords2.plot(&mut plot, "purple".to_string());
    grid2.plot_outline(&mut plot, "purple".to_string());

    grid2.plot_grid_points(&mut plot, PlotPoint::Given(point_1));
    grid2.plot_grid_points(&mut plot, PlotPoint::Given(point_2));
    grid2.plot_grid_points(&mut plot, PlotPoint::Given(point_3));
    grid2.plot_grid_points(&mut plot, PlotPoint::Given(point_4));

    plot.set_title("Regular Grid on a Coordinate System");
    //plot.legend();
    plot.save("tests/grid2_points_test").expect("could not save figure");

}

pub fn test_projection(coords1:CoordinateSystem,coords2:CoordinateSystem){

    let point_1 = Point::new(6.0,1.0,Coordinates::RELATIVE(coords1.clone()));
    let point_2 = Point::new(1.1,7.0,Coordinates::RELATIVE(coords1.clone()));
    let point_3 = Point::new(-8.7,-2.5,Coordinates::RELATIVE(coords1.clone()));
    let point_4 = Point::new(-4.5,-9.5,Coordinates::RELATIVE(coords1.clone()));


    let mut plot = Plot::new();
    let grid1 = GRID2D::new_empty((10,10), (1.0,1.0), (0.0,0.0),0.001, Coordinates::RELATIVE(coords1.clone()));
    coords1.plot(&mut plot,"green".to_string());
    grid1.plot_outline(&mut plot,"green".to_string());
    grid1.plot_grid_points(&mut plot, PlotPoint::Given(point_1));
    grid1.plot_grid_points(&mut plot, PlotPoint::Given(point_2));
    grid1.plot_grid_points(&mut plot, PlotPoint::Given(point_3));
    grid1.plot_grid_points(&mut plot, PlotPoint::Given(point_4));

    plot.set_title("Regular Grid on a Coordinate System");
    //plot.legend();
    plot.save("tests/grid1_points_test").expect("could not save figure");



    let mut plot = Plot::new();

    let point_1 = Point::new(6.0,1.0,Coordinates::RELATIVE(coords2.clone()));
    let point_2 = Point::new(1.1,7.0,Coordinates::RELATIVE(coords2.clone()));
    let point_3 = Point::new(-8.7,-2.5,Coordinates::RELATIVE(coords2.clone()));
    let point_4 = Point::new(-4.5,-9.5,Coordinates::RELATIVE(coords2.clone()));



    let grid2 = GRID2D::new_empty((10,10), (1.0,1.0), (0.0,0.0),0.001, Coordinates::RELATIVE(coords2.clone()));
    coords2.plot(&mut plot, "purple".to_string());
    grid2.plot_outline(&mut plot, "purple".to_string());

    grid2.plot_grid_points(&mut plot, PlotPoint::Given(point_1));
    grid2.plot_grid_points(&mut plot, PlotPoint::Given(point_2));
    grid2.plot_grid_points(&mut plot, PlotPoint::Given(point_3));
    grid2.plot_grid_points(&mut plot, PlotPoint::Given(point_4));

    plot.set_title("Regular Grid on a Coordinate System");
    //plot.legend();
    plot.save("tests/grid2_points_test").expect("could not save figure");

}

 */

