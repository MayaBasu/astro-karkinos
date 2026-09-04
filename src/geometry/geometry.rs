use std::fmt::{Display, Formatter};
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








#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Coordinates {
    ABSOLUTE,
    RELATIVE(CoordinateSystem),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoordinateSystem {
    x_axis: (Regular, Regular),
    y_axis: (Regular, Regular),
    center: (Regular, Regular),
    label: String,
}


impl Coordinates {
    pub fn plot(&self, plot: &mut Plot, color: String) {
        match &self {
            Coordinates::ABSOLUTE => {
                let absolute = CoordinateSystem {
                    x_axis: (Regular::try_from(1.0), Regular::try_from(0.0)),
                    y_axis: (Regular::try_from(0.0), Regular::try_from(1.0)),
                    center: (Regular::try_from(0.0), Regular::try_from(0.0)),
                    label: "Absolute".to_string(),
                };
                absolute.plot(plot, color)
            }
            Coordinates::RELATIVE(c) => { c.plot(plot, color) }
        }
    }
}

impl CoordinateSystem {
    pub fn new(x_axis: (Regular, Regular), y_axis: (Regular, Regular), center: (Regular, Regular), label: String) -> CoordinateSystem {
        assert!(x_axis.0 + x_axis.1);

        CoordinateSystem {
            x_axis,
            y_axis,
            center,
            label,
        }
    }
    pub fn point_from_absolute(&self, point: Point) -> Point {
        match point.coordinates {
            Coordinates::ABSOLUTE => {
                let det = self.y_axis.1 * self.x_axis.0 - self.y_axis.0 * self.x_axis.1;

                let proj_x = ((point.x - self.center.0) * self.y_axis.1 - (point.y - self.center.1) * self.y_axis.0) / det;
                let proj_y = (-(point.x - self.center.0) * self.x_axis.1 + (point.y - self.center.1) * self.x_axis.0) / det;

                Point::new(proj_x, proj_y, Coordinates::RELATIVE(self.clone()))
            }
            Coordinates::RELATIVE(_) => { panic!("tried to from_absolute a point in a not absolute coordinate system :( ") }
        }
    }


    pub fn plot(&self, plot: &mut Plot, color: String) {
        let mut x_axis = Curve::new();

        x_axis.set_line_width(2.0);
        x_axis.set_line_color(&*color);
        x_axis.set_line_style("dashed");
        x_axis.set_label(format!("x axis for {:?}", self.label).as_str());

        x_axis.points_begin();
        x_axis.points_add(self.center.0.value, self.center.1.value);
        x_axis.points_add((self.center.0 + (self.x_axis.0)).value,(self.center.1 + (self.x_axis.1)).value);
        x_axis.points_end();


        let mut y_axis = Curve::new();
        y_axis.set_line_width(1.0);
        y_axis.set_line_color(&*color);
        x_axis.set_line_style("solid");
        y_axis.set_label(format!("y axis for {:?}", self.label).as_str());

        y_axis.points_begin();
        y_axis.points_add(self.center.0.value, self.center.1.value);
        y_axis.points_add((self.center.0 + (self.y_axis.0)).value, (self.center.1 + (self.y_axis.1)).value);
        y_axis.points_end();

        plot.add(&x_axis);
        plot.add(&y_axis);
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]

pub struct Point {
    pub x: Regular,
    pub y: Regular,
    pub coordinates: Coordinates
}

impl Point {
    pub fn new(x: Regular, y: Regular, coordinates: Coordinates) -> Point {
        Point { x, y, coordinates }
    }
    pub fn as_absolute(&self) -> Point {
        match &self.coordinates {
            Coordinates::ABSOLUTE => {
                //println!("already in absolute");
                self.clone()
            }
            Coordinates::RELATIVE(coordinate_system) => {
                let absolute_x = self.x * coordinate_system.x_axis.0 + self.y * coordinate_system.y_axis.0 + coordinate_system.center.0;
                let absolute_y = self.x * coordinate_system.x_axis.1 + self.y * coordinate_system.y_axis.1 + coordinate_system.center.1;
                Point::new(absolute_x, absolute_y, Coordinates::ABSOLUTE)
            }
        }
    }


    pub fn convert(&self, coordinate_system: &Coordinates) -> Point {
        let absolute = self.as_absolute();
        match coordinate_system {
            Coordinates::ABSOLUTE => { absolute }
            Coordinates::RELATIVE(coordinate_system) => {
                coordinate_system.point_from_absolute(absolute)
            }
        }
    }
    pub fn values(&self) -> (Regular, Regular) {
        (self.x, self.y)
    }

    pub fn plot(&self, plot: &mut Plot, color: String) {
        let mut point = Curve::new();
        point.set_line_style("none")
            .set_marker_color(&*color)
            .set_marker_every(1)
            .set_marker_size(7.0)
            .set_marker_style("*");
        point.points_begin();
        let (x, y) = self.as_absolute().values();
        point.points_add(x.value, y.value);
        point.points_end();
        plot.add(&point);
    }
}


pub enum PlotPoint {
    No,
    Given(Point),
    Random,
}

















