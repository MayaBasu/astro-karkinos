use plotpy::{Curve, Plot};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub enum Coordinates {
    ABSOLUTE,
    RELATIVE(CoordinateSystem),
}

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct CoordinateSystem{
    pub x_axis: (f64, f64),
    pub y_axis: (f64,f64),
    pub center: (f64,f64),
    pub label: String,
}


impl Coordinates{
    pub fn plot(&self, plot:&mut Plot, color:String){
        match &self{
            Coordinates::ABSOLUTE => {
                let absolute = CoordinateSystem{
                    x_axis: (1.0, 0.0),
                    y_axis: (0.0, 1.0),
                    center: (0.0, 0.0),
                    label: "Absolute".to_string(),
                };
                absolute.plot(plot,color)
            }
            Coordinates::RELATIVE(c) => {c.plot(plot,color)}
        }
    }
}

impl CoordinateSystem{
    pub fn new(x_axis:(f64,f64),y_axis:(f64,f64),center: (f64,f64),label:String) -> CoordinateSystem{
        CoordinateSystem{
            x_axis,
            y_axis,
            center,
            label,
        }
    }
    pub fn point_from_absolute(&self, point:Point) -> Point{
        match point.coordinates{
            Coordinates::ABSOLUTE => {
                let det =  self.y_axis.1*self.x_axis.0-self.y_axis.0*self.x_axis.1 ;

                let proj_x = ((point.x -self.center.0)*self.y_axis.1-(point.y -self.center.1)*self.y_axis.0  )/det;
                let proj_y = (-(point.x -self.center.0)*self.x_axis.1+(point.y -self.center.1)*self.x_axis.0 )/det;

                Point::new(proj_x,proj_y,Coordinates::RELATIVE(self.clone()))
            }
            Coordinates::RELATIVE(_) => {panic!("tried to from_absolute a point in a not absolute coordinate system :( ")}
        }

    }



    pub fn plot(&self,plot: &mut Plot,color:String){
        let mut x_axis = Curve::new();

        x_axis.set_line_width(2.0);
        x_axis.set_line_color(&*color);
        x_axis.set_line_style("dashed");
        x_axis.set_label(format!("x axis for {:?}",self.label).as_str());

        x_axis.points_begin();
        x_axis.points_add(self.center.0,self.center.1);
        x_axis.points_add(self.center.0 + (self.x_axis.0), self.center.1 + (self.x_axis.1));
        x_axis.points_end();


        let mut y_axis = Curve::new();
        y_axis.set_line_width(1.0);
        y_axis.set_line_color(&*color);
        x_axis.set_line_style("solid");
        y_axis.set_label(format!("y axis for {:?}",self.label).as_str());

        y_axis.points_begin();
        y_axis.points_add(self.center.0,self.center.1);
        y_axis.points_add(self.center.0  + (self.y_axis.0),self.center.1  +  (self.y_axis.1));
        y_axis.points_end();

        plot.add(&x_axis);
        plot.add(&y_axis);
    }
}


#[derive(Clone, Debug,Serialize,Deserialize)]

pub struct Point{
    pub x: f64,
    pub y: f64,
    pub coordinates: Coordinates
}

impl Point{
    pub fn new(x:f64,y:f64,coordinates: Coordinates) -> Point{
        Point{x,y,coordinates}
    }
    pub fn to_absolute(&self) -> Point{
        match &self.coordinates{
            Coordinates::ABSOLUTE => {
                //println!("already in absolute");
                self.clone()}
            Coordinates::RELATIVE(coordinate_system) => {
                let absolute_x =  self.x * coordinate_system.x_axis.0 + self.y *coordinate_system.y_axis.0 + coordinate_system.center.0;
                let absolute_y = self.x * coordinate_system.x_axis.1 + self.y * coordinate_system.y_axis.1 + coordinate_system.center.1;
                Point::new(absolute_x,absolute_y,Coordinates::ABSOLUTE)
            }
        }
    }
    pub fn convert(&self, coordinate_system: &Coordinates) -> Point{
        let absolute = self.to_absolute();
        match coordinate_system{
            Coordinates::ABSOLUTE => { absolute }
            Coordinates::RELATIVE(coordinate_system  ) => {
                coordinate_system.point_from_absolute(absolute)
            }
        }
    }
    pub fn values(&self)-> (f64,f64){
        (self.x,self.y)
    }

    pub fn plot(&self, plot:&mut Plot,color:String){

        let mut point = Curve::new();
        point.set_line_style("none")
            .set_marker_color(&*color)
            .set_marker_every(1)
            .set_marker_size(7.0)
            .set_marker_style("*");
        point.points_begin();
        let (x,y) = self.to_absolute().values();
        point.points_add(x,y);
        point.points_end();
        plot.add(&point);

    }
}














