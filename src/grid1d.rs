use plotpy::{Curve, Plot, Text};
use rand::RngExt;
use crate::grid2d::PlotPoint;


pub enum Location1D{
    TooHigh,
    TooLow,
    JustRight,
}

pub enum Neighbors {
    Two(usize,usize),
    One(usize),
}

#[derive(Debug,Clone)]
pub struct GRID1D {
    pub scale: f64,
    pub step_size: f64,
    pub minimum_value: f64,
    pub maximum_value: f64,
    pub snap_precision: f64,
    pub label: String,

}

impl GRID1D {
    pub fn new_empty(
        step_size: f64, //TODO have units for this length
        minimum_value: f64,
        maximum_value: f64,
        snap_precision: f64,
        scale:f64,

    ) -> GRID1D {
        assert!(snap_precision<0.5);
        GRID1D {
            scale,
            step_size,
            minimum_value,
            maximum_value,
            snap_precision,
            label:"".to_string(),
        }
    }


    pub fn new_from_points(points:Vec<f64>,snap_precision:f64,label:String)-> GRID1D{
        //we try to make a regular grid out of a vector of points
        let num = points.len();
        assert!(num>=2, "Need at least 2 points to make a grid");
        let high = points[num-1];
        let low = points[0];
        assert!(low < high, "First point is not lower than last point");
        let expected_interval = (high-low)/(num as f64 -1.0);
        (0..num-1).for_each(|i|{
            assert!(points[i+1]>points[i],
                    "Failed to make grid because points must monotonically increase");
            assert!(((points[i+1]-points[i]) - expected_interval).abs() < snap_precision*expected_interval,
                    " Failed to make grid because points must be evenly spaced")
        });
        GRID1D{
            scale: 1.0,
            step_size: expected_interval,
            minimum_value: low,
            maximum_value: high,
            snap_precision,
            label,
        }
    }

    pub fn pretty_print(&self){
        println!("1D grid {:?} \n\
        min: {} \n\
        max: {} \n\
        step size: {} \n\
        snap precision: {} \n",
        self.label,
        self.minimum_value,
        self.maximum_value,
        self.step_size,
        self.snap_precision)
    }
    pub fn num(&self)-> usize{
        //TODO MUST VERIFY THI IS AN INTEGER
        ( (self.maximum_value-self.minimum_value)/self.step_size ) as usize + 1
    }


    pub fn location(&self, grid_number:usize) -> f64 {
        assert!((grid_number <= self.num()-1)&&(grid_number >= 0));
        self.minimum_value + self.step_size*grid_number as f64
    }

    pub fn size(&self)-> f64{
        self.step_size*(self.num()-1)as f64
    }

    pub fn random(&self)-> f64{
        let mut rng = rand::rng();
        let scale: f64 = rng.random();
        self.minimum_value + self.size()*scale

    }
    pub fn inside_or_outside(&self, point:f64) -> Location1D{
        let epsilon = self.snap_precision;
        let max = self.minimum_value + self.size();
        if (point < self.minimum_value - epsilon) {
            return Location1D::TooLow
        };
        if (point > max + epsilon){
            return Location1D::TooHigh
        }
        Location1D::JustRight
    }

    pub fn fit_grid(&self, point:f64)->(usize,f64){
        //ensure that the point is within the grid
        match self.inside_or_outside(point) {
            Location1D::TooHigh => {panic!("too big to fit")}
            Location1D::TooLow => {panic!("too small to fit")}
            Location1D::JustRight => {}
        }
        //find the nearest point and then return the residuals to it
        let delta = point - self.minimum_value;
        let scaled_residual = delta/self.step_size - (delta/self.step_size).floor();


        let (modulus, residual) = if scaled_residual <= 0.5{
            let modulus = (delta/self.step_size).floor() as usize;
            let residual = scaled_residual*self.step_size;
            (modulus,residual)

        }else{
            let modulus = (delta/self.step_size).floor() as usize + 1;
            let residual = (scaled_residual-1.0)*self.step_size;
            (modulus,residual)
        };

        (modulus, residual)
    }
    //TODO remove redundancey of these two functions

    pub fn snap(&self,point:f64)-> usize{
        let (modulus,residual) = self.fit_grid(point);
        if (residual.abs() >= self.snap_precision){
            panic!("Couldn't snap point")
        };
        modulus
    }


    pub fn find_neighbors(&self, point:f64) -> Neighbors{
        let epsilon = self.snap_precision;
        let (modulus,residual)  = self.fit_grid(point);
        if (residual.abs() <= epsilon){
            return Neighbors::One(self.snap(point))
        };
        //The point must be in the middle
        let (upper,lower) = if residual < 0.0{ (modulus,modulus-1) }else{ (modulus+1, modulus)};
        Neighbors::Two(upper,lower)
    }

    pub fn plot_points(&self, plot:&mut Plot, add_point:PlotPoint){

        let mut frame = Curve::new();
        frame.set_marker_color("pink")
            .set_marker_every(1)
            .set_marker_style(".");

        let mut grid_points = Curve::new();
        grid_points.set_line_style("none")
            .set_label(format!("Grid points: {:?}",self.label).as_str())
            .set_marker_color("blue")
            .set_marker_every(1)
            .set_marker_size(7.0)
            .set_marker_style(".");

        let mut corner = Curve::new();
        corner
            .set_label("Corner")
            .set_line_style("none")
            .set_marker_color("#eeea83")
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_marker_style(".");


        let mut extra_point = Curve::new();
        extra_point.set_marker_color("#eeea83")
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_line_style("none")
            .set_marker_style("*");



        let mut grid_numbers = Text::new();
        grid_numbers.set_color("purple")
            .set_fontsize(5.0);


        grid_points.points_begin();
        for point in 0..self.num(){
            let point_location = self.location(point);
            grid_points.points_add(point_location, 0.0);
            let label = format!("{}",point);
            grid_numbers.draw(point_location, 0.0, label.as_str());
        }
        grid_points.points_end();

        corner.points_begin();
        let corner_location = self.minimum_value;
        let corner_label = format!("Corner: ({:.3},{:.3})",corner_location,0.0);
        corner.points_add(corner_location,0.0).set_label(corner_label.as_str());
        corner.points_end();

        let mut example_point = Vec::new();

        match add_point {
            PlotPoint::No => {}
            PlotPoint::Given(point) => { example_point.push(point.x)}
            PlotPoint::Random => {
                let random = self.random();
                example_point.push((random)); }
        };

        for point in example_point{
            let (_,res) = self.fit_grid(point);
            extra_point.points_begin();
            extra_point.points_add(point,0.0).set_label(format!("x, y residuals: {:.3}", res).as_str());
            extra_point.points_end();

            let corners  = self.find_neighbors(point);
            let mut frame_points = Vec::new();
            match corners{

                Neighbors::Two(a,b) => {
                    frame_points.push(a);
                    frame_points.push(b);
                    println!("{:?}",(a,b));}

                Neighbors::One(a) => { frame_points.push(a); }
            }
            frame.points_begin();
            for point in frame_points{
                println!("point {point}");
                let point = self.location(point);
                frame.points_add(point,0.0);
            }

            frame.points_end();
        }



        plot.add(&grid_numbers);
        plot.add(&grid_points);
        plot.add(&extra_point);
        plot.add(&corner);
        plot.add(&frame)
            .set_figure_size_inches(10.0,10.0)
            .grid_labels_legend("x", "y");


    }

}



