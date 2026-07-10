use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::coordinate_system::Coordinates::RELATIVE;
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::*;
use astroimsim_geometry::points::Point;

use crate::spatial_effect::SpatialEffect;

pub fn test_2d_interpolation() {
    let coordinates = CoordinateSystem::new(
        (0.0, 1.0),
        (1.0, 0.0),
        (0.0, 0.0),
        "test coords",
        "red",
    );

    let test_grid = GRID2D::new_empty((100, 100), (0.010, 0.010), (10.0, 10.0), 0.45, RELATIVE(coordinates));
    let mut matrix = Vec::new();
    for row in 0..test_grid.y_num{
        let mut row_vec = Vec::new();
        for column in 0..test_grid.x_num{
            let point = test_grid.locate(test_grid.grid_number(column,row));
            row_vec.push(test_function(&point))

        }
        matrix.push(row_vec)
    }

    let data_grid = SpatialEffect::from_matrix("data grid", test_grid.clone(), "NA", matrix);

    println!("data grid is :{:?}",data_grid);


    for i in 0..100{
        let point = test_grid.random();
        let datum1 = data_grid.get_data(&point);
        let datum2 = test_function(&point);
        println!(" {:?} {:?}   {:?}  {:?}",test_grid.fit_grid(&point), datum1,datum2, 100.0*(datum2-datum1)/datum2)
    }
}

pub fn test_function(point: &Point)-> f64{
    let (x,y) = point.to_absolute().values();
    let result = x.powi(3) + y.powi(2);
    if result.is_nan(){
        println!("{:?} gave NaN",(x,y))
    }
    result
}