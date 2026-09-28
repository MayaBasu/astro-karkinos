
use std::fs::{File};
use std::io::{BufWriter, Read, Write};
use crate::geometry::*;

use plotpy::Plot;
use markdown2pdf;
use markdown2pdf::config::ConfigSource;

use markdown2pdf::fonts::FontConfig;


pub fn generate_notebook() {
    let file = File::create("notebook.md").expect("Could not create geometry documentation notebook");
    let mut w = BufWriter::new(file);
    writeln!(w, "# RustSim Geometry Module").unwrap();

    writeln!(w, "The geometry module of the RustSim ecosystem handles coordinates, points in those coordinate systems, and regular one and two dimensional grids.\
      Within a telescope model, the geometry module serves the following purposes: \
    \n").unwrap();

    writeln!(w,"\
    1. Allows the conversion between a location in the spacecraft field of view (FOV) and a location on a detector plane. \
    This is needed to map a star from a location in the sky to a signal positioned on the detector and is a transformation \
    effected by distorsions and reflections from optical components.\
    \n2. Points in these coordinate systems are needed to represent the locations of data such as simulated or \
    measured point spread functions (PSFs).\
    \n3. Spectral data such as filter curves are loaded into one dimensional regular grids, and spatial data such \
    as vinietting profiles and spatially varying PSFs are loaded onto two dimensional grids.\
    Loading one or two dimensional data allows us to interpolate the data to intermediate points. For example, for a given star \
    we can interpolate between PSF shapes known at a finite set of locations to estimate a PSF at the exact location of the star. ").unwrap();

    writeln!(w, "## Coordinate Systems").unwrap();

    writeln!(w,"\
    The field of view of the telescope and detector planes are examples of objects which require coordinates systems.\
    A coordinate system is a collection of an x-axis, a y-axis, and a center.\
    The usual cartesian coordinate system with center: [0.,0.], x-axis: [1.,0.], and y-axis: [0.,1.] \
    serves as the base coordinate system \
    with respect to which all other coordinates systems are defined.\
     This coordinate system is called absolute coordinates, and all points in any coordiante system \
are plotted in absolute coordinates. \
To create a new coordinate system we need to specify it's center and x and y axis in terms of absolute coordinates.\
     The center is given by an array [x_0,y_0]\
      which give the x and y position of the center of the new coordinate system in terms of absolute coordinates.\
The x and y axis are each given by an array of absolute coordinates [x,y] \
If we specify an axis via [x,y] this means that the unit vector of our new coordinate system will start at [x_0,y_0]\
and end at [x_0 + x, y_0 + y].
A coordinate system does not have to be orthonormal, \
the only restriction is that the values used to specify it are Regular (not Nan or inf) and that the length \
of each axis is greater than 0. As an example we define the following two coordinate systems: \n \n \n \n").unwrap();




let coord_sys_1 = CoordinateSystem::new([2.,0.], [0.,4.], [1.,1.5]);
let coord_sys_2 = CoordinateSystem::new([-3.4,-1.3],[3.,4.1],[-1.2,-0.3]);


let mut plot = Plot::new();
ABSOLUTE_COORDINATES.plot(&mut plot, "black", "absolute coordinates");
coord_sys_1.plot(&mut plot, "green", "coord_sys_1");
coord_sys_2.plot(&mut plot, "purple", "coord_sys_2");
plot.legend();
plot.set_title("Example Coordinate Systems");
    plot.set_figure_size_inches(10.,10.);
    plot.grid_and_labels("x","y");
plot.save("src/geometry/geometry_documentation_images/ex1").expect("failed to plot coordinates");



    
    
    
    writeln!(w,"\
```rust
let coord_sys_1 = CoordinateSystem::new([2.,0.], [0.,4.], [1.,1.]);
let coord_sys_2 = CoordinateSystem::new([-3.4,-1.3],[3.,4.1],[-1.2,-0.3]);
let mut plot = Plot::new();
coord_sys_1.plot(&mut plot, 'green', 'coord_sys_1');
coord_sys_2.plot(&mut plot, 'purple', 'coord_sys_2');
plot.legend();
plot.set_title(\"Example Coordinate Systems\");
plot.show('plot_path').expect('failed to plot coordinates')
```").unwrap();

    writeln!(w,"Running this we get the following plot at 'plot_path':\n").unwrap();

    writeln!(w,"![coordiante test](src/geometry/geometry_documentation_images/ex1.png)").unwrap();

    writeln!(w,"## Points").unwrap();
    let point_0 = Point::new(1.,1.,&ABSOLUTE_COORDINATES);
    let point_1 = Point::new(1.,1., &coord_sys_1);
    let point_2 = Point::new(1.,1.,&coord_sys_2);


    point_0.plot(&mut plot, "black", "absolute: (1,1)");
    point_1.plot(&mut plot, "green", "coord_sys_1: (1,1)");
    point_2.plot(&mut plot, "purple", "coord_sys_2: (1,1)");
    plot.legend();
    plot.save("src/geometry/geometry_documentation_images/ex2").expect("failed to plot coordinates");





    writeln!(w, "To define a point we specify it's x and y values with respect to any coordinate system.\
     Here we make three different points each at position (1,1) in their respective coordinate systems:").unwrap();
     writeln!(w,"```rust \
let point_0 = Point::new(1.,1.,&ABSOLUTE_COORDINATES);
let point_1 = Point::new(1.,1., &coord_sys_1);
let point_2 = Point::new(1.,1.,&coord_sys_2);
point_0.plot(&mut plot, 'black', 'absolute: (1,1)');
point_1.plot(&mut plot, 'green', 'coord_sys_1: (1,1)');
point_2.plot(&mut plot, 'purple', 'coord_sys_2: (1,1)');
```").unwrap();

    writeln!(w,"\n![coordiante test](src/geometry/geometry_documentation_images/ex2.png)").unwrap();

    writeln!(w,"\n Once defined in one coordinate system, points can be transformed into other coordinate systems with the '\
     'transform_to' method. '.to_absolute()' is shorthand for .transform_to(&ABSOLUTE_COORDINATES). \
      Based on our definitions of 'coord_sys_1' and 'coord_sys_2' we expect that 'point_1' and 'point_2' will land at (3.,5.5) and (-1.6,2.5) in \
       absolute coordinates respectively. 'point_0' should be unchanged. We can verify this by using the .hypot method to return the euclidean norm of the difference \
        between the expected and actual coordinates:").unwrap();

    let expected_0 = Point::new(1.,1.,&ABSOLUTE_COORDINATES);
    let expected_1  =Point::new(3.,5.5,&ABSOLUTE_COORDINATES);
    let expected_2 = Point::new(-1.6,2.5,&ABSOLUTE_COORDINATES);

    //Two equivalent expressions
    assert!((point_1.transform_to(&ABSOLUTE_COORDINATES)-expected_1).hypot() < 0.00001);
    assert!((point_1.to_absolute()-expected_1).hypot() < 0.00001);

    assert!((point_0.to_absolute()- expected_0).hypot() < 0.00001);
    assert!((point_2.to_absolute()-expected_2).hypot() < 0.00001);


    writeln!(w,"\n\
    ```rust
let expected_0 = Point::new(1.,1.,&ABSOLUTE_COORDINATES);
let expected_1  =Point::new(3.,5.5,&ABSOLUTE_COORDINATES);
let expected_2 = Point::new(-1.6,2.5,&ABSOLUTE_COORDINATES);

//Two equivalent expressions:
assert!((point_1.transform_to(&ABSOLUTE_COORDINATES)-expected_1).hypot() < 0.00001);
assert!((point_1.to_absolute()-expected_1).hypot() < 0.00001);

assert!((point_0.to_absolute()- expected_0).hypot() < 0.00001);
assert!((point_2.to_absolute()-expected_2).hypot() < 0.00001);
```");

    writeln!(w,"\n\
Points can be transformed from one coordiante system to any other coordinate system with the .transform_to() method.
    ").unwrap();
let point = Point::new(0.5,1.3, &coord_sys_1);
let transformed_point = point_1.transform_to(&coord_sys_2);


    writeln!(w,"\n ```rust
let point = Point::new(0.5,1.3, &coord_sys_1);
//hsekljfr
let transformed_point = point_1.transform_to(&coord_sys_2);
```").expect("hyh");

    writeln!(w, "\n \n ## Regular Grids").unwrap();
    writeln!(w, "\n \n Data used in the telescope simulator will need to be \
    loaded into regular grids. For example, we require spectral responses to \
    have been sampled at regular spectral intervals. Spatial data such as \
    vinietting also must be given at regular intervals. Spectral data is loaded into a\
    grid1d object, and spatial data is loaded into a grid2d object. Each spatial grid must be defined\
    with respect to some coordinate system and each spectral grid must be definined with\
    respect to a unit system").unwrap();
    writeln!(w, "\n \n ### One Dimentional Grids").unwrap();
    writeln!(w,"\n \n A one dimentional grid can be created with the following information:\n\
    \n1. The number of points in the grid.\
    \n2. The minimum value present in the grid.\
    \n3. The maximum value present in the grid,\
    \n4. The relative snap precision of the grid.\
    \n5. Units of the grid values.\
    \n\
\n \n The relative snap precision is the error to which the grid is tolorant to, given as a fraction of the grid step size.
The snap precision used in calculations is the relative snap precision multiplied by the step size. For example,
suppose that a grid has step size of 2nm with a grid point located at 1nm. If this grid had a relative snap precision
of 0.001 then it's snap precision would be 0.002nm. A data point located at 1.0001nm would then be 'gridded' to the 1nm data point,
since it is 'close enough' to 1nm. If the relative snap precision was 0.00001 however, such a point would not be 'gridded' to the 1nm data point.
The snap precision is the tolorance of the regular grid to numerical error. If you have a data set of values which should be a regular grid, but due to floating point
error are not exactly regular you can raise the relative snap precision until the grid accepts them as regularly spaced within the expected
tolerance. However, keep the precision of the grid as small as possible for best results when interpolating data. Grids can be defined with
the .new() method, or by pasing a vector of points:
    ");
    let grid1d_1 = GRID1D::new(6, -1.5, 3.5, 0.001, Grid1DUnits::mm);
    let points = vec![-1.5002,-0.4999,0.5008,1.5009,2.4997,3.4997];
    let grid1d_2 = GRID1D::from_values(points, 0.001, Grid1DUnits::mm);
    assert_eq!(grid1d_1,grid1d_2);

    writeln!(w,"\n\n\
    ```rust 


let grid1d_1 = GRID1D::new(6, -1.5,3.5,0.001,Units::mm);
let points = vec![-1.5002,-0.4999,0.5008,1.5009,2.4997,3.4997];
let grid1d_2 = GRID1D::from_values(points,0.001,Units::mm);
assert_eq!(grid1d_1,grid1d_2);
```").unwrap();

    writeln!(w,"The grid1d struct has is built with interpolation in mind. Given any point represended by a float and it's associated unitsl,\
    we can use the .locate() method to find where it is in the struct.").unwrap();




    writeln!(w,"\
    ```\
let point_1 = -1.;
let point_2 = 3.50001;
let point_3 = -3.;
let point_4 = 5.;

match grid1d_1.locate(point_1,Units::mm) {{
    Location::Higher => {{
        println!('point is more than snap_precision higher than any grid point')
    }}
    Location::Lower => {{
        println!('point is more than snap_precision lower than any grid point')
    }}
    Location::Snapped(i) => {{
        println!('point is less than snap precision away from grid point i')
    }}
    Location::Between(i, j, f) => {{
        println!('point is between grid points {{i}} and {{j}}')
        println!('the distance between point and grid point {{i}} is {{f}} as a fraction of step size')
    }}
}};

assert_eq!(grid1d_1.locate(point_1,Units::mm),Location::Between(0,1,0.5));
assert_eq!(grid1d_1.locate(point_2,Units::mm),Location::Snapped(5));
assert_eq!(grid1d_1.locate(point_3,Units::mm),Location::Lower);
assert_eq!(grid1d_1.locate(point_4,Units::mm),Location::Higher);
```").unwrap();

    writeln!(w, " \n This location function is what becomes one dimensional interpolation when \
    we move to data grids. Here the Between enum holds a scaled residue which is the fraction of the grid's\
    step width that the point sits at away from the lower grid index it is between.").unwrap();


    writeln!(w," ### Two Dimensional Grids").unwrap();

    writeln!(w, " \n \n \n Two dimensional grids are the structure behind spatially varying data.\
    A two dimensional grid is specified by:
    \n 1. [x_num, y_num]: number of grid points in the x and y directions\
    \n 2. [x_step_size, y_step_size]: distance between grid points in the x and y directions\
    \n 3. center: Point struct which indicates the location of the grid. The point can be in any coordinates\
    \n 4. relative_snap_precision: Just as for one dimensional grids.\
    \n 5. coordinate_system: coordinate system of the grid in which the step sizes are calculated in \
    \
    \n \n").unwrap();


    writeln!(w,"```\
let grid1  = GRID2D::new(
    [10,10],
    [0.5,0.5],
    Point::new(0.,0.,&coord_sys_1),
    0.0001,
    coord_sys_1);
let grid2 = GRID2D::new(
    [7,5],
    [0.75,1.],
    Point::new(-16.,-16.5, &ABSOLUTE_COORDINATES),
    0.0001,
     coord_sys_2
);
let mut plot = Plot::new();
grid1.plot(&mut plot, 'green', 'grid1');
grid2.plot(&mut plot, 'purple', 'grid2')

plot.save('plot_path').expect('Could not plot grids');
```
   ").unwrap();


    writeln!(w,"![coordiante test](src/geometry/geometry_documentation_images/ex3.png)").unwrap();










    let grid0 = GRID2D::new(
        [20,20],
        [1.,1.],
        Point::new(0.,0.,&ABSOLUTE_COORDINATES),
        0.0001,
        ABSOLUTE_COORDINATES
    );
    let grid1  = GRID2D::new(
        [10,10],
        [0.5,0.5],
        Point::new(0.,0.,&coord_sys_1),
        0.0001,
        coord_sys_1);
    let grid2 = GRID2D::new(
        [7,5],
        [0.75,1.],
        Point::new(-16.,-16.5, &ABSOLUTE_COORDINATES),
        0.0001,
         coord_sys_2
    );
    let mut plot = Plot::new();
  //  grid0.plot(&mut plot, "black", "grid0");
    grid1.plot(&mut plot, "green", "grid1");
    grid2.plot(&mut plot, "purple", "grid2");
    println!("POINTS");
    println!("\n {:?}", grid2.locate(0).y);
    println!("\n {:?}", grid2.locate(7).y);
    println!("\n {:?}", grid2.locate(14).y);
    println!("\n {:?}", grid2.locate(21).y);


    plot.save("src/geometry/geometry_documentation_images/ex3.png").expect("Could not plot grids");

    writeln!(w, "\n Once a two dimensional grid is defined we can use the .fit_grid() method to fit \
    the grid to an arbitrary point. On sucess the method returns a tuple (x_index,y_index, x,y).\
     The method takes a boolean value fractional_residue. If true, the function returns the residue of the \
     point psotion relative to the grid point (x_index,y_index) as a fraction of step size. If false, x,y are \
     float values in the coordinates of the grid. The fit grid function expects that the given point\
     is within the grid (up to the grid snap_precision), If you wish to obtain the fit of a p[rojection");


    grid1.fit_grid(&Point::new(0.5,0.5,&coord_sys_1),true);
    writeln!(w, "");
    


    w.flush().expect("jalsejkf");
    // Convert Markdown string to PDF with proper error handling
    let mut mkdn = String::new();
    File::open("notebook.md").expect("trouble reading").read_to_string(&mut mkdn);

  

    let font_config = FontConfig::new()
        .with_default_font("Georgia");
    markdown2pdf::parse_into_file(mkdn.clone(), "output.pdf", ConfigSource::File("config.toml"),Some(&font_config)).expect("huhhh");




    //markdown2pdf::parse_into_file(mkdn, "output3.pdf", ConfigSource::Embedded(EMBEDDED), Some(&font_config)).expect("ahhhh");









}
pub fn test_coordiantes()-> (CoordinateSystem,CoordinateSystem){
    let mut plot = Plot::new();
    let basic_coordinates = CoordinateSystem::new([1.0,0.0],[0.0,1.0],[0.0,0.0]);
    let strange_coordinates = CoordinateSystem::new([0.7,0.3],[1.4,-1.3],[0.5,0.5]);
    basic_coordinates.plot_string(&mut plot, "orange".to_string(), "basic".to_string());
    strange_coordinates.plot_string(&mut plot, "purple".to_string(), "strange".to_string());
    plot.set_title("Defining and Plotting Coordinate Systems");
    plot.legend();
    plot.save("tests/coordinate_test").expect("could not save figure");
    (basic_coordinates,strange_coordinates)

}
/*
pub fn test_grids(coords1:CoordinateSystem,coords2:CoordinateSystem)->(GRID2D,GRID2D){


    let mut plot = Plot::new();
    let grid1 = GRID2D::new_empty([10,10], [1.0,1.0], [0.0,0.0],0.001, Coordinates::RELATIVE(coords1.clone()));
    coords1.plot(&mut plot, "green".to_string(), "".to_string());
    grid1.plot_outline(&mut plot, "green".to_string(), "".to_string());
    grid1.plot_grid_points(&mut plot);
    plot.set_title("Regular Grid on a Coordinate System");
    plot.legend();
    plot.save("tests/grid1_test").expect("could not save figure");



    let mut plot = Plot::new();
    let grid2 = GRID2D::new_empty([10,10], [1.0,1.0], [0.0,0.0],0.001, Coordinates::RELATIVE(coords2.clone()));
    coords2.plot(&mut plot, "purple".to_string(),"".to_string());
    grid2.plot_outline(&mut plot, "purple".to_string(),"".to_string());
    grid2.plot_grid_points(&mut plot,"".to_string());
    plot.set_title("Regular Grid on a Coordinate System");
    plot.legend();
    plot.save("tests/grid2_test").expect("could not save figure");


    (grid1,grid2)
}



pub fn test_grids_with_points(coords1:CoordinateSystem,coords2:CoordinateSystem){

    let point_1 = Point::new(1.0,1.0,&coords1.clone());
    let point_2 = Point::new(1.1,3.0,&coords1.clone());
    let point_3 = Point::new(2.7,-2.5,&coords1.clone());
    let point_4 = Point::new(-4.5,-2.5,&coords1.clone());




    let mut plot = Plot::new();
    let grid1 = GRID2D::new_empty([10,10],
                                  [1.0,1.0],
                                  Point::new(0.0,0.0,&coords1),
                                  0.001,
                                  coords1);
    coords1.plot(&mut plot,"green".to_string(),"".to_string());
    grid1.plot_outline(&mut plot,"green".to_string(),"".to_string());
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



