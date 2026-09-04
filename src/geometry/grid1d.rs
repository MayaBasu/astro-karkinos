use std::cmp::{Ordering, PartialOrd};
use crate::geometry::geometry::*;
use plotpy::{Curve, Plot, Text};
use rand::RngExt;


pub enum Location{
    Higher,
    Lower,
    Snapped(usize),
    Between(usize,usize,Regular)
}



#[derive(Debug, Clone,Copy)]
pub struct GRID1D {
    /*
    units: nm, mm
    step_size: Regular, strictly positive
    num_steps > 0, finite
    minimum_value: Regular, less than maximum
    maximum_value: Regular
    snap precision: value which is less than 0.5.
    This is the "absolute snap precision", it is initialized with a relative snap precision
    snap_precision = step_size*relative_snap_precision
     */
    units: Units,
    step_size: Regular,
    num_steps: usize,
    minimum_value: Regular,
    maximum_value: Regular,
    snap_precision: Regular,

}

#[derive(Clone,Debug,Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Units{
    nm,
    mm,
}


impl GRID1D {
    pub fn new(
        number_of_points: usize,
        minimum_value: Regular,
        maximum_value: Regular,
        relative_snap_precision: Regular,
        units: Units,

    ) -> GRID1D {
        assert!(number_of_points > 1,
                "number_of_points must be greater than 1");
        assert!(minimum_value < maximum_value,
                "minimum_value must be strictly less than maximum value");
        assert!(relative_snap_precision < Regular::try_from(0.5),
                "relative snap precision must be less than 0.5");

        let step_size = (maximum_value-minimum_value)/ Regular::from(number_of_points-1);
        let snap_precision = relative_snap_precision * step_size;

        GRID1D {
            num_steps,
            step_size,
            minimum_value,
            maximum_value,
            snap_precision,
            units
        }
    }
    pub fn num_points(&self)-> usize{
        self.num_steps + 1
    }

    pub fn from_values(points: Vec<f64>, relative_snap_precision: Regular, units:Units) -> GRID1D {
        let num = points.len();
        assert!(num >= 2, "Need at least 2 points to make a grid");
        let points:Vec<Regular> = points.iter().map(|f|Regular::try_from(*f)).collect();
        let high = points[num - 1];
        let low = points[0];
        assert!(low < high, "First point is not lower than last point");
        let expected_interval = (high - low) / (num as f64 - 1.0);
        let snap_precision = relative_snap_precision*expected_interval;
        (0..num - 1).for_each(|i| {
            assert!(points[i + 1] > points[i],
                    "Failed to make grid because points must monotonically increase");
            assert!(((points[i + 1] - points[i]) - expected_interval).abs() < snap_precision,
                    " Failed to make grid because points must be evenly spaced")
        });
        GRID1D {
            units,
            step_size: expected_interval,
            num_steps,
            minimum_value: low,
            maximum_value: high,
            snap_precision,
        }
    }

    pub fn pretty_print(&self) {
        println!("1D grid in units of {:?} \n\
        min: {:?} \n\
        max: {:?} \n\
        number of points: {:?} \n
        step size: {:?} \n\
        snap precision: {:?} \n",
        self.units,
        self.minimum_value,
        self.maximum_value,
        self.num_steps,
        self.step_size,
        self.snap_precision)
    }

    pub fn locate_grid_point(&self, grid_number: usize) -> f64 {
        assert!((grid_number <= self.num() - 1) && (grid_number >= 0), "Grid number must be between 0 and num_points-1 inclusive");
        self.minimum_value + self.step_size * grid_number as f64
    }

    pub fn grid_width(&self) -> Regular {
        self.step_size * (self.num() - 1) as f64
    }

    pub fn random_point(&self) -> Regular {
        let mut rng = rand::rng();
        let scale: f64 = rng.random();
        self.minimum_value + self.grid_width() * Regular::try_from(scale)
    }

    pub fn locate(&self, value:Regular, unit:Units )-> Location{
        //TODO: Add in automatic unit conversion
        assert_eq!(self.units, unit, "Convert value to same unit as 1D grid before attempting to locate");

        if value > (self.maximum_value + self.snap_precision) {
            return Location::Higher};
        if value < (self.minimum_value - self.snap_precision) {
            return Location::Lower }

        //point indices below and above the point
        let delta = (value-self.minimum_value);
        let lower = (delta/self.step_size).floor();
        let upper = (delta/self.step_size).ceil();

        if ((upper*self.step_size)-delta) < self.snap_precision{
            return Location::Snapped(upper.value() as usize)};
        if (delta - (lower*self.step_size)) < self.snap_precision{
            return Location::Snapped(lower.value() as usize)
        };

        let scaled_residual = (delta-lower)/self.step_size;

        Location::Between(
            lower.value() as usize,
            upper.value() as usize,
            scaled_residual
        )

    }


    pub fn plot_gridpoints(&self, plot: &mut Plot, label:String, color:String) {

        let mut grid_points = Curve::new();
        grid_points.set_line_style("none")
            .set_label(format!("{:?}", label).as_str())
            .set_marker_color(color.as_str())
            .set_marker_every(1)
            .set_marker_size(7.0)
            .set_marker_style(".");

        let mut grid_numbers = Text::new();
        grid_numbers.set_color("purple")
            .set_fontsize(5.0);

        grid_points.points_begin();
        for point in 0..self.num() {
            let point_location = self.locate_grid_point(point);
            grid_points.points_add(point_location, 0.0);
            let label = format!("{}", point);
            grid_numbers.draw(point_location, 0.0, label.as_str());
        }
        grid_points.points_end();

        plot.add(&grid_numbers);
        plot.add(&grid_points);
            plot.set_figure_size_inches(10.0, 10.0)
            .grid_labels_legend("x", "y");
    }
    pub fn plot_extra_point(&self, value: Regular, units: Units, plot:&mut Plot){

        let mut frame_indices = Vec::new();
        match self.locate(value,units) {
            Location::Higher => { frame_indices.push(self.num_steps)}
            Location::Lower => { frame_indices.push(0)}
            Location::Snapped(i) => { frame_indices.push(i)}
            Location::Between(i, j, _) => {
                frame_indices.push(i);
                frame_indices.push(j)}
        };

        let mut frame = Curve::new();
        frame.set_marker_color("#eeea83")
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_line_style("dotted")
            .set_marker_style("*");

        frame.points_begin();
        for point in frame_indices{
            frame.points_add(self.locate_grid_point(point),0.0);
        }
        frame.points_end();
        plot.add(&frame);

        }

}







