
use std::fmt::{Display, Formatter};

use crate::geometry::geometry::*;
use plotpy::{Curve, Plot, Text};
use rand::RngExt;
use crate::geometry::GriddingError;
#[derive(PartialEq,Debug)]
pub enum Location{
    Higher,
    Lower,
    Snapped(usize),
    Between(usize,usize,f64)
}

impl PartialEq for GRID1D{
    fn eq(&self, other: &Self) -> bool {


        let snap_precision = f64::max(self.snap_precision.value(),other.snap_precision.value());
        //TODO: make this less arbitrary
        if (self.snap_precision.value() -other.snap_precision.value())/f64::min(self.snap_precision.value(),other.snap_precision.value())>0.1{
            println!("1D Grids have different snap precision values, {:?} v.s {:?}",self.snap_precision.value(), other.snap_precision.value());
            return false
        }
        //TODO: add in automatic unit conversion
        if (self.minimum_value()-other.minimum_value()).abs() > snap_precision*2.0{
            println!("1D Grids have minimum values differing by more than twice the greatest of their snap precisions");
            return false
        }
        if (self.maximum_value()-other.maximum_value()).abs() > snap_precision*2.0{
            println!("1D Grids have maximum values differing by more than twice the greatest of their snap precisions");
            return false
        }
        if (self.step_size()-other.step_size()).abs() > snap_precision*2.0{
            println!("1D Grids have step sizes differing by more than twice the greatest of their snap precisions");
            return false
        }


        if (self.num_steps.abs_diff(other.num_steps)) >0 {
            println!("Grids have different numbers of steps");
            return false
        }
        if (self.units != other.units){
            println!("Grids must have the same units, automatic conversion is not yet implemented");
            return false
        }
        true


    }
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
    units: Grid1DUnits,
    step_size: Regular,
    num_steps: usize,
    minimum_value: Regular,
    maximum_value: Regular,
    pub snap_precision: Regular,

}

#[derive(Clone,Debug,Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Grid1DUnits {
    nm,
    mm,
    angstroms,
}

impl Display for Grid1DUnits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self{
            Grid1DUnits::nm => {write!(f, "nm")}
            Grid1DUnits::mm => {write!(f, "mm")}
            Grid1DUnits::angstroms => {write!(f,"angstroms")}
        }

    }
}


impl GRID1D {
    pub fn new_from_regular(
        number_of_points: usize,
        minimum_value: Regular,
        maximum_value: Regular,
        relative_snap_precision: Regular,
        units: Grid1DUnits,

    ) -> GRID1D {
        assert!(number_of_points > 1,
                "number_of_points must be greater than 1");
        assert!(minimum_value < maximum_value,
                "minimum_value must be strictly less than maximum value");
        assert!(relative_snap_precision < Regular::try_from(0.5),
                "relative snap precision must be less than 0.5");

        let step_size = (maximum_value-minimum_value)/ Regular::try_from((number_of_points-1) as f64);
        let snap_precision = relative_snap_precision * step_size;
        let num_steps = number_of_points-1;

        GRID1D {
            num_steps,
            step_size,
            minimum_value,
            maximum_value,
            snap_precision,
            units
        }
    }



    pub fn new(
        number_of_points: usize,
        minimum_value: f64,
        maximum_value: f64,
        relative_snap_precision: f64,
        units: Grid1DUnits,

    ) -> GRID1D {
        Self::new_from_regular(number_of_points,
        Regular::try_from(minimum_value),
        Regular::try_from(maximum_value),
        Regular::try_from(relative_snap_precision),
        units)
    }
    pub fn num_points(&self)-> usize{
        self.num_steps + 1
    }



    pub fn from_values(points: Vec<f64>, relative_snap_precision: f64, units: Grid1DUnits) -> GRID1D {
        let relative_snap_precision = Regular::try_from(relative_snap_precision);
        let num = points.len();
        assert!(num >= 2, "Need at least 2 points to make a grid");
        let points:Vec<Regular> = points.iter().map(|f|Regular::try_from(*f)).collect();
        let high = points[num - 1];
        let low = points[0];
        assert!(low < high, "First point is not lower than last point");
        let expected_interval = (high - low) / Regular::try_from(num as f64 - 1.0);
        let snap_precision = relative_snap_precision*expected_interval;
        (0..num - 1).for_each(|i| {
            assert!(points[i + 1] > points[i],
                    "Failed to make grid because points must monotonically increase");
            assert!(((points[i + 1] - points[i]) - expected_interval).abs() < 2*snap_precision,
                    " Failed to make grid because points must be evenly spaced")
        });
        GRID1D {
            units,
            step_size: expected_interval,
            num_steps:num-1,
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

    pub fn unit(&self)-> Grid1DUnits {
        self.units
    }

    pub fn locate_grid_point(&self, grid_number: usize) -> f64 {
        assert!((grid_number <= self.num_points() - 1) && (grid_number >= 0), "Grid number must be between 0 and num_points-1 inclusive");
        (self.minimum_value + self.step_size * grid_number as f64).value()
    }

    pub fn locate_grid_points(&self)->Vec<f64>{
        (0..self.num_points()).map(|i|self.locate_grid_point(i)).collect()
    }

    pub fn grid_width(&self) -> Regular {
        self.maximum_value-self.minimum_value
    }

    pub fn random_point(&self) -> Regular {
        let mut rng = rand::rng();
        let scale: f64 = rng.random();
        self.minimum_value + self.grid_width() * Regular::try_from(scale)
    }

    pub fn locate_regular(&self, value:Regular, unit: Grid1DUnits) -> Location{
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

        println!("{:?} \
        {:?} \
        {:?}  {:?}", lower, upper, delta,self.step_size);

        let scaled_residual = ((delta-lower*self.step_size)/self.step_size).value();

        Location::Between(
            lower.value() as usize,
            upper.value() as usize,
            scaled_residual
        )

    }

    pub fn locate(&self, value:f64, unit: Grid1DUnits) -> Location{
        self.locate_regular(Regular::try_from(value),unit)

    }


    pub fn snap(&self, value:Regular, units: Grid1DUnits) -> Result<usize, GriddingError>{
        match self.locate_regular(value, units){
            Location::Higher => {Err(GriddingError)}
            Location::Lower => {Err(GriddingError)}
            Location::Snapped(location) => {Ok(location)}
            Location::Between(_, _, _) => {Err(GriddingError)}
        }
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
        for point in 0..self.num_points() {
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
    pub fn plot_extra_point(&self, value: Regular, units: Grid1DUnits, plot:&mut Plot){

        let mut frame_indices = Vec::new();
        match self.locate_regular(value,units) {
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
    pub fn minimum_value(&self)->f64{
        self.minimum_value.value()
    }
    pub fn maximum_value(&self)->f64{
        self.maximum_value.value()
    }
    pub fn step_size(&self)->f64{
        self.step_size.value()
    }

}


#[cfg(test)]
pub mod tests{
    use crate::geometry::Grid1DUnits::mm;
    use crate::geometry::{GriddingError, Regular};
    use super::GRID1D;
    use super::Grid1DUnits;
    use super::Location;


    #[test]
    fn test_1d_grid() {
        let grid1d_1 = GRID1D::new(6, -1.5, 3.5, 0.001, Grid1DUnits::mm);
        let points = vec![-1.5002, -0.4999, 0.5008, 1.5009, 2.4997, 3.4997];
        let grid1d_2 = GRID1D::from_values(points, 0.001, Grid1DUnits::mm);


        let point_1 = -1.;
        let point_2 = 3.50001;
        let point_3 = -3.;
        let point_4 = 5.;

        assert_eq!(grid1d_1.locate(point_1, Grid1DUnits::mm), Location::Between(0, 1, 0.5));
        assert_eq!(grid1d_1.locate(point_2, Grid1DUnits::mm), Location::Snapped(5));
        assert_eq!(grid1d_1.locate(point_3, Grid1DUnits::mm), Location::Lower);
        assert_eq!(grid1d_1.locate(point_4, Grid1DUnits::mm), Location::Higher);
        assert_eq!(grid1d_1, grid1d_2);
        assert_eq!(grid1d_1.num_steps, 5);
        assert_eq!(grid1d_1.num_points(), 6);
        assert_eq!(grid1d_1.unit(),Grid1DUnits::mm);
        assert_eq!(grid1d_1.locate_grid_point(2),0.5);
        assert_eq!(grid1d_1.locate_grid_points(),[-1.5, -0.5, 0.5, 1.5, 2.5, 3.5]);
        assert_eq!(grid1d_1.grid_width().value(), 5.0);
        assert!(grid1d_1.random_point().value()>=grid1d_1.minimum_value());
        assert!(grid1d_1.random_point().value()<=grid1d_1.maximum_value());
        //-1.5002, -0.4999, 0.5008, 1.5009, 2.4997, 3.4997
        assert_eq!(grid1d_1.snap(Regular::try_from(0.5),mm),Ok(2));
        assert_eq!(grid1d_1.snap(Regular::try_from(-1.5002),mm),Ok(0));
        assert_eq!(grid1d_1.snap(Regular::try_from(-1.51),mm),Err(GriddingError));
        assert_eq!(grid1d_1.snap(Regular::try_from(3.51),mm),Err(GriddingError));
        assert_eq!(grid1d_1.snap(Regular::try_from(1.6),mm),Err(GriddingError));




        let grid1 = GRID1D::new(8,-1.2,3.5,0.001,Grid1DUnits::nm);

        assert_eq!(grid1, grid1);

        let grid2 = GRID1D::new(9,-1.2,3.5,0.001,Grid1DUnits::nm);
        assert_ne!(grid1,grid2);

        let grid2 = GRID1D::new(8,-0.2,3.5,0.001,Grid1DUnits::nm);
        assert_ne!(grid1,grid2);

        let grid2 = GRID1D::new(8,-1.2,3.9,0.001,Grid1DUnits::nm);
        assert_ne!(grid1,grid2);

        let grid2 = GRID1D::new(8,-1.2,3.5,0.0001,Grid1DUnits::nm);
        assert_ne!(grid1,grid2);
        let grid2 = GRID1D::new(8,-1.2,3.5,0.001,Grid1DUnits::mm);
        assert_ne!(grid1,grid2);


    }
}






