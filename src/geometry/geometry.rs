use std::fmt::{Display, Formatter};
use std::ops::{Div, Mul, Neg};
use plotpy::{Curve, Plot};
use serde::{Deserialize, Serialize};

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



impl ::core::ops::Add for Regular {
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
impl ::core::ops::Sub for Regular {
    type Output = Regular;
    fn sub(self, rhs: Regular) -> Regular {
        Regular {
            value:   self.value - rhs.value
        }
    }
}
impl ::core::ops::Mul for Regular {
    type Output = Regular;
    fn mul(self, rhs: Self) -> Self::Output {
        Regular{
            value: rhs.value*self.value
        }
    }
}


impl ::core::ops::Div for Regular {
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

    pub fn plot(&self, plot: &mut Plot, color: String, label:String) {
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
}


#[derive(Clone, Debug, Serialize, Deserialize, Copy)]

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

        let det =
            self.coordinate_system.y_axis[1] * self.coordinate_system.x_axis[0] -
            self.coordinate_system.y_axis[0] * self.coordinate_system.x_axis[1];

        let proj_x =
            ((absolute_x - self.coordinate_system.center[0]) * self.coordinate_system.y_axis[1] -
            (absolute_y - self.coordinate_system.center[1]) * self.coordinate_system.y_axis[0]) / det;

        let proj_y =
            (-(absolute_x - self.coordinate_system.center[0]) * self.coordinate_system.x_axis[1] +
            (absolute_y - self.coordinate_system.center[1]) * self.coordinate_system.x_axis[0]) / det;

        Point::new_from_regular(proj_x, proj_y, &new_coordinate_system)

    }

    pub fn to_absolute(&self)->Point{
        self.transform_to(&ABSOLUTE_COORDINATES)
    }


    pub fn values(&self) -> (f64, f64) {
        (self.x.value(), self.y.value())
    }

    pub fn plot(&self, plot: &mut Plot, color: String) {
        let mut point = Curve::new();
        point
            .set_marker_color(&*color)
            .set_marker_size(7.0)
            .set_marker_style("*");

        let (x, y) = self.transform_to(&ABSOLUTE_COORDINATES).values();

        point.points_begin();
        point.points_add(x, y);
        point.points_end();

        plot.add(&point);
    }
}


impl core::ops::Sub for Point{
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


impl core::ops::Sub for &Point{
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

impl core::ops::Div for &Point{
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

impl core::ops::Add for Point{
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

impl core::ops::Mul<usize> for Regular{
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

impl core::ops::Div<Regular> for Point {
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

impl core::ops::Div<f64> for Point{
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
















