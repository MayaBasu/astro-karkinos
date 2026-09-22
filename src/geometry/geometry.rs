use std::fmt::{Display, Formatter};
use std::ops::{Div, Mul, Neg};
use plotpy::{Curve, Plot};
use serde::{Deserialize, Serialize};
use core::ops::*;

/*
Regular is a type which is ensured to never be NaN or infinite.
Division of Regular values fails if the divisor is 0.
*/

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct Regular{value: f64}

impl Regular{
    
    pub fn abs(&self)->Regular{
        Regular{value:self.value.abs()}
    }
    pub fn try_from(float:f64) -> Regular{
        assert!(!float.is_nan(), "Tried to convert float to NonNanfinite type, but float is NaN");
        assert!(float.is_finite(), "Tried to convert float to NonNaNfinite type, but float is infinite");
        Regular{value:float}
    }
    pub fn value(&self)->f64{
        self.value
    }
    pub fn floor(&self)->Regular{
        Regular{value:self.value().floor()}
    }
    pub fn round(&self)-> Regular{
        Regular{value:self.value().round()}
    }
    pub fn ceil(&self)->Regular{
        Regular{value:self.value().ceil()}
    }
}



impl Add for Regular {
    type Output = Regular;
    fn add(self, rhs: Regular) -> Regular {
        Regular {
            value: rhs.value + self.value
        }
    }
}
impl Display for Regular {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value().fmt(f)
    }
}
impl Sub for Regular {
    type Output = Regular;
    fn sub(self, rhs: Regular) -> Regular {
        Regular {
            value:   self.value - rhs.value
        }
    }
}
impl Mul for Regular {
    type Output = Regular;
    fn mul(self, rhs: Self) -> Self::Output {
        Regular{
            value: rhs.value*self.value
        }
    }
}

impl Mul<[Regular;2]> for Regular {
    type Output = [Regular;2];
    fn mul(self, rhs:[Regular;2])-> Self::Output{
        [rhs[0]*self,rhs[1]*self]
    }
}
impl Div for Regular {
    type Output = Regular;
    fn div(self, rhs: Self) -> Self::Output {
        //check for division by zero
        assert!(rhs.value.is_normal(), "Division by zero error");
        Regular{
            value: self.value/rhs.value
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Copy)]
pub struct CoordinateSystem {
    x_axis: [Regular;2],
    y_axis: [Regular;2],
    center: [Regular;2],
    
}

pub const ABSOLUTE_COORDINATES: CoordinateSystem =  CoordinateSystem{
    x_axis: [Regular{value:1.0},Regular{value:0.0}],
    y_axis: [Regular{value:0.0},Regular{value:1.0}],
    center: [Regular{value:0.0},Regular{value:0.0}],
};


impl CoordinateSystem {
    pub fn new(x_axis: [f64;2], y_axis: [f64;2], center: [f64;2]) -> CoordinateSystem {
        assert!(x_axis[0].powi(2) + x_axis[1].powi(2) > 0.0, "Euclidian length of x_axis must be greater than zero");
        assert!(y_axis[0].powi(2) + y_axis[1].powi(2) > 0.0, "Euclidian length of y_axis must be greater than zero");

        CoordinateSystem {
            x_axis: [Regular::try_from(x_axis[0]),Regular::try_from(x_axis[1])],
            y_axis: [Regular::try_from(y_axis[0]),Regular::try_from(y_axis[1])],
            center: [Regular::try_from(center[0]),Regular::try_from(center[1])],
        }
    }

    pub fn plot_string(&self, plot: &mut Plot, color: String, label:String) {
        let mut x_axis = Curve::new();

        x_axis.set_line_width(2.0);
        x_axis.set_line_color(&*color);
        x_axis.set_line_style("dashed");
        x_axis.set_label(format!("x axis for {:?}", label).as_str());

        x_axis.points_begin();
        x_axis.points_add(self.center[0].value, self.center[1].value);
        x_axis.points_add((self.center[0] + (self.x_axis[0])).value,(self.center[1] + (self.x_axis[1])).value);
        x_axis.points_end();


        let mut y_axis = Curve::new();
        y_axis.set_line_width(1.0);
        y_axis.set_line_color(&*color);
        x_axis.set_line_style("solid");
        y_axis.set_label(format!("y axis for {:?}", label).as_str());

        y_axis.points_begin();
        y_axis.points_add(self.center[0].value, self.center[1].value);
        y_axis.points_add((self.center[0] + (self.y_axis[0])).value, (self.center[1] + (self.y_axis[1])).value);
        y_axis.points_end();

        plot.add(&x_axis);
        plot.add(&y_axis);
    }

    pub fn plot(&self, plot:&mut Plot, color:&str,label:&str){
        self.plot_string(plot,color.to_string(),label.to_string())
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, Copy,PartialEq)]

pub struct Point {
    pub x: Regular,
    pub y: Regular,
    pub coordinate_system: CoordinateSystem
}

impl Mul<f64> for Regular {
    type Output = Regular;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs = Regular::try_from(rhs);
        self*rhs
    }
}



impl Point {

    pub fn hypot(&self)->f64{
        (self.x.value().powi(2) + self.y.value().powi(2)).sqrt()
    }

    pub fn add_to_curve(&self, curve:&mut Curve){
        let val = self.to_absolute();
        curve.points_add(val.x.value(),val.y.value());
    }
    
    
    pub fn round(&self)->Point{
        Point{
            x:self.x.round(),
            y:self.y.round(),
            coordinate_system: self.coordinate_system
        }
    }
    pub fn new(x: f64, y: f64, coordinate_system: &CoordinateSystem) -> Point {
        Point { x: Regular::try_from(x), y:Regular::try_from(y), coordinate_system: coordinate_system.clone() }
    }
    pub fn new_from_regular(x:Regular,y:Regular, coordinate_system: &CoordinateSystem)->Point{
        Point {x,y,coordinate_system: coordinate_system.clone()}
    }
    pub fn floor(&self)->Point{
        Point{
            x: self.x.floor(),
            y: self.y.floor(),
            coordinate_system: self.coordinate_system
        }
    }

    pub fn transform_to(&self, new_coordinate_system: &CoordinateSystem)-> Point{

        let absolute_x =
            self.x * self.coordinate_system.x_axis[0] +
            self.y * self.coordinate_system.y_axis[0] +
            self.coordinate_system.center[0];

        let absolute_y =
            self.x * self.coordinate_system.x_axis[1] +
            self.y * self.coordinate_system.y_axis[1] +
            self.coordinate_system.center[1];

        println!("absolue x absolute y {absolute_x} {absolute_y}");

        let det =
            new_coordinate_system.y_axis[1] * new_coordinate_system.x_axis[0] -
                new_coordinate_system.y_axis[0] * new_coordinate_system.x_axis[1];

        let proj_x =
            ((absolute_x - new_coordinate_system.center[0]) * new_coordinate_system.y_axis[1] -
            (absolute_y - new_coordinate_system.center[1]) * new_coordinate_system.y_axis[0]) / det;

        let proj_y =
            (-(absolute_x - new_coordinate_system.center[0]) * new_coordinate_system.x_axis[1] +
            (absolute_y - new_coordinate_system.center[1]) * new_coordinate_system.x_axis[0]) / det;

        Point::new_from_regular(proj_x, proj_y, &new_coordinate_system)

    }

    pub fn to_absolute(&self)->Point{
        self.transform_to(&ABSOLUTE_COORDINATES)
    }


    pub fn values(&self) -> (f64, f64) {
        (self.x.value(), self.y.value())
    }

    pub fn plot(&self, plot: &mut Plot, color: &str,label:&str) {
        let mut point = Curve::new();
        point
            .set_line_color(color)
            .set_marker_color(color)
            .set_marker_size(7.0)
            .set_marker_style("*")
            .set_line_style("none");
        if label.len() > 0{
            point.set_label(label);

            println!("Adding label");
        };
        println!("{:?} {:?} {:?}",self.x,self.y,self);

        let (x, y) = self.transform_to(&ABSOLUTE_COORDINATES).values();
        println!("{:?} {:?}",x,y);



        point.points_begin();
        point.points_add(x, y);
        point.points_end();



        plot.add(&point);
        plot.legend();
    }
}


impl Sub for Point{
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = rhs.transform_to(&self.coordinate_system);
        Point{
            x: self.x-rhs.x,
            y: self.y-rhs.y,
            coordinate_system: self.coordinate_system,
        }
    }
}
impl Sub for &Point{
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        let rhs = rhs.transform_to(&self.coordinate_system);
        Point{
            x: self.x-rhs.x,
            y: self.y-rhs.y,
            coordinate_system: self.coordinate_system,
        }
    }
}

impl Div for &Point{
    type Output = Point;

    fn div(self, rhs: &Point) -> Self::Output {
        Point{
            x: self.x/rhs.x,
            y: self.y/rhs.y,
            coordinate_system: self.coordinate_system,
        }
    }
}

impl Mul for Point{
    type Output = Point;
    fn mul(self, rhs:Point)->Self::Output{
        let rhs = rhs.transform_to(&self.coordinate_system);
        Point{
            x: self.x*rhs.x,
            y: self.y*rhs.y,
            coordinate_system: self.coordinate_system,
        }
    }

}

impl Div for Point{
    type Output = Point;

    fn div(self, rhs: Point) ->Self::Output{
        let rhs = rhs.transform_to(&self.coordinate_system);
        Point{
            x: self.x/rhs.x,
            y: self.y/rhs.y,
            coordinate_system: self.coordinate_system,
        }
    }

}
impl Add for Point{
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let rhs = rhs.transform_to(&self.coordinate_system);
        Point{
            x: self.x+rhs.x,
            y: self.y+rhs.y,
            coordinate_system: self.coordinate_system,
        }
    }
}
impl Neg for Regular{
    type Output = Regular;

    fn neg(self) -> Self::Output {
        Regular{value:-self.value}
    }
}
impl Mul<Regular> for Point{
    type Output = Point;

    fn mul(self, rhs: Regular) -> Point {
        Point{
            x: self.x*rhs,
            y: self.y*rhs,
            coordinate_system: self.coordinate_system,
        }
    }
}

impl Mul<usize> for Regular{
    type Output = Regular;

    fn mul(self, rhs: usize) -> Self::Output {
        Regular{
            value: self.value*rhs as f64,
        }
    }
}

impl Mul<Regular> for usize{
    type Output = Regular;

    fn mul(self, rhs: Regular) -> Self::Output {
        let val = Regular::try_from(self as f64);
        val*rhs
    }
}

impl Div<Regular> for Point {
    type Output = Point;

    fn div(self, rhs: Regular) -> Self::Output {
        assert_ne!(rhs.value(),0.0,"Can not divide by 0");
        Point{
            x: self.x/rhs,
            y: self.y/rhs,
            coordinate_system: self.coordinate_system,
        }
    }
}

impl Div<f64> for Point{
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        assert_ne!(rhs,0.0,"Can not divide by 0");
        let rhs = Regular::try_from(rhs);
        Point{
            x: self.x/rhs,
            y: self.y/rhs,
            coordinate_system: self.coordinate_system,
        }
    }
}






#[cfg(test)]
mod tests {
    use astro_karkinos::geometry::Location;
    use crate::geometry::{grid1d, Units, GRID1D};
    use super::*;

    #[test]
    #[should_panic = "Can not make a coordinate system with x_axis zero length"] // This also works
    fn zero_length_x_axis() {
        CoordinateSystem::new([0.,0.],[0.,1.],[0.,0.]);
    }
    #[test]
    #[should_panic = "Can not make a coordinate system with y_axis zero length"] // This also works
    fn zero_length_y_axis() {
        CoordinateSystem::new([1.,0.],[0.,0.],[0.,0.]);
    }


    #[test]
    fn to_absolute_test(){

        let coord_sys_1 = CoordinateSystem::new([2.,0.], [0.,4.], [1.,1.5]);
        let coord_sys_2 = CoordinateSystem::new([-3.4,-1.3],[3.,4.1],[-1.2,-0.3]);

        let point_0 = Point::new(1.,1.,&ABSOLUTE_COORDINATES);
        let point_1 = Point::new(1.,1., &coord_sys_1);
        let point_2 = Point::new(1.,1.,&coord_sys_2);

        let expected_0 = Point::new(1.,1.,&ABSOLUTE_COORDINATES);
        let expected_1  =Point::new(3.,5.5,&ABSOLUTE_COORDINATES);
        let expected_2 = Point::new(-1.6,2.5,&ABSOLUTE_COORDINATES);

        //Equivalent expressions
        assert!((point_1.transform_to(&ABSOLUTE_COORDINATES)-expected_1).hypot() < 0.00001);
        assert!((point_1.to_absolute()-expected_1).hypot() < 0.00001);

        assert!((point_0.to_absolute()- expected_0).hypot() < 0.00001);
        assert!((point_2.to_absolute()-expected_2).hypot() < 0.00001);

    }


    #[test]
    fn coordinate_change_test(){

        let coord_sys_1 = CoordinateSystem::new([2.,0.], [0.,4.], [1.,1.5]);
        let coord_sys_2 = CoordinateSystem::new([-3.4,-1.3],[3.,4.1],[-1.2,-0.3]);
        
        let point = Point::new(0.5,1.3, &coord_sys_1);
        let transformed_point = point.transform_to(&coord_sys_2);
        //Check the transformation worked correctly
        let x = transformed_point.x * coord_sys_2.x_axis[0]+
            transformed_point.y*coord_sys_2.y_axis[0] + coord_sys_2.center[0];

        let y = transformed_point.x * coord_sys_2.x_axis[1]+
            transformed_point.y*coord_sys_2.y_axis[1] + coord_sys_2.center[1];

        assert!((Point::new_from_regular(x,y,&ABSOLUTE_COORDINATES)-point.to_absolute()).hypot() < 0.00001);


    }
    
    #[test]
    fn test_1d_grid() {
        let grid1d_1 = GRID1D::new(5, -1.5, 3.5, 0.001, Units::mm);
        let points = vec![-1.5002, -0.4999, 0.5008, 1.5009, 2.4997, 3.4997];
        let grid1d_2 = GRID1D::from_values(points, 0.001, Units::mm);


        let point_1 = -1.;
        let point_2 = 3.50001;
        let point_3 = -3.;
        let point_4 = 5.;

        assert_eq!(grid1d_1.locate(point_1,Units::mm),grid1d::Location::Between(0,1,0.5));
        assert_eq!(grid1d_1.locate(point_2,Units::mm),grid1d::Location::Snapped(5));
        assert_eq!(grid1d_1.locate(point_3,Units::mm),grid1d::Location::Lower);
        assert_eq!(grid1d_1.locate(point_4,Units::mm),grid1d::Location::Higher);
        assert_eq!(grid1d_1, grid1d_2);
    }

}









