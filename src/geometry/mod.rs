pub mod geometry;
pub mod grid1d;
pub mod grid2d;


pub use self::grid1d::*;
pub use self::grid2d::*;
pub use self::geometry::*;


#[cfg(test)]
mod tests {
    use rand_distr::num_traits::float::FloatCore;
    use super::*;
    #[test]
    pub fn test_coordinate_transformations(){
        /*
        Test the conversion between absolute and relative coordinates
         */
        //test as_absolute() function
        let relative_coordinates = CoordinateSystem::new(
            [0.3, -1.3],
            [1.7, 2.8],
            [-0.3, -5.0]);
        let point = Point::new(-1.4,
                               4.0,
                               &relative_coordinates);
        point.to_absolute();
        println!("{:?}", point);
        //assert!(6.08-point.as_absolute().x < 0.0001);
        //assert!(8.02-point.as_absolute().y < 0.0001);
        println!("Test of as_absolute() passed");
        let num = Regular::try_from(4.0);
        println!("{:?}", num)

        //test




    }
}

