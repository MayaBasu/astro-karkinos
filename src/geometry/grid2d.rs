
use crate::geometry::geometry::*;
use plotpy::{Curve, Plot, Text};
use rand::RngExt;

pub enum Corners {
    Four(usize, usize, usize, usize),
    Two(usize, usize),
    One(usize),
}
#[derive(Debug)]
pub enum Location {
    Outside,
    Inside,
}
pub enum Bin {
    Binned(usize, usize),
    Outside,
}


#[derive(Debug, Clone)]
pub struct GRID2D {
    coordinates: Coordinates,

    x_num: usize,
    y_num: usize,

    x_step_size: f64, //In coordinate system
    y_step_size: f64, //In coordinate system

    x_size: f64,
    y_size: f64,

    num_points: usize,

    center: (Regular, Regular),
    corner: (Regular, Regular),

    snap_precision: f64, //In coordinat sytstem

    label: String,

}

impl GRID2D {
    pub fn new_empty((x_num, y_num): (usize, usize),
                     (x_step_size, y_step_size): (f64, f64), //TODO have units for this length
                     center: (f64, f64),
                     snap_precision: f64,
                     coordinates: Coordinates,
    ) -> GRID2D {
        let x_size = x_step_size * (x_num - 1) as f64;
        let y_size = y_step_size * (y_num - 1) as f64;
        let num_points = x_num * y_num;
        let (x_0, y_0) = center;
        let corner = (x_0 - x_size / 2.0, y_0 - y_size / 2.0);
        // println!("Huh?");
        //  println!("CORNER IS {:?} for {x_num}",Point::new(corner.0,corner.1,coordinates.clone()).to_absolute());
        //  println!("Center is {:?}",(x_0,y_0));
        assert!(snap_precision < 0.5);
        GRID2D {
            coordinates,
            x_num,
            x_step_size,
            x_size,
            y_num,
            y_step_size,
            y_size,
            num_points,
            center,
            corner,
            snap_precision: snap_precision * f64::min(y_step_size, x_step_size),
            label: "".to_string(),
        }
    }


    pub fn new_from_width((x_num, y_num): (usize, usize),
                          (x_width, y_width): (f64, f64), //TODO have units for this length
                          center: (f64, f64),
                          snap_precision: f64,
                          coordinates: Coordinates,
    ) -> GRID2D {
        let x_step_size = x_width / (x_num - 1) as f64;
        let y_step_size = y_width / (y_num - 1) as f64;
        let num_points = x_num * y_num;
        let (x_0, y_0) = center;
        let corner = (x_0 - x_width / 2.0, y_0 - y_width / 2.0);
        // println!("Huh?");
        //  println!("CORNER IS {:?} for {x_num}",Point::new(corner.0,corner.1,coordinates.clone()).to_absolute());
        //  println!("Center is {:?}",(x_0,y_0));
        assert!(snap_precision < 0.5);
        GRID2D {
            coordinates,
            x_num,
            x_step_size,
            x_size: x_width,
            y_num,
            y_step_size,
            y_size: y_width,
            num_points,
            center,
            corner,
            snap_precision: snap_precision * f64::min(y_step_size, x_step_size),
            label: "".to_string(),
        }
    }


    pub fn xy_indices(&self, grid_number: usize) -> (usize, usize) {
        assert!((grid_number <= self.num_points - 1) && (grid_number >= 0));
        let x_index = grid_number % self.x_num;
        let y_index = (grid_number - x_index) / self.y_num;
        (x_index, y_index)
    }

    pub fn grid_number(&self, x_index: usize, y_index: usize) -> usize {
        assert!(x_index <= self.x_num - 1);
        assert!(y_index <= self.y_num - 1);
        y_index * self.x_num + x_index
    }

    pub fn locate(&self, grid_number: usize) -> Point {
        assert!((grid_number <= self.x_num * self.y_num - 1) && (grid_number >= 0));
        let (x_corner, y_corner) = self.corner;
        let (x_index, y_index) = self.xy_indices(grid_number);
        let (x, y) = (x_corner + x_index as f64 * self.x_step_size, y_corner + y_index as f64 * self.y_step_size);
        //println!("relatice location is {:?}",(x,y));
        Point::new(x, y, self.coordinates.clone())
    }


    pub fn snap(&self, point: Point) -> usize {
        let (x_mod, y_mod, x_residual, y_residual) = self.fit_grid(&point);
        if (x_residual.abs() >= self.snap_precision) | (y_residual.abs() >= self.snap_precision) {
            panic!("Couldn't snap point")
        };
        self.grid_number(x_mod, y_mod)
    }

    pub fn random(&self) -> Point {
        let mut rng = rand::rng();
        let x_scale: f64 = rng.random();
        let y_scale: f64 = rng.random();
        let (x, y) = (self.corner.0 + self.x_size * x_scale, self.corner.1 + self.y_size * y_scale);
        //println!("Randomly generating point {:?} within the grid",(x,y));
        let point = Point::new(x, y, self.coordinates.clone());
        // println!("RANDOM:  {:?}",self.inside_or_outside(&point));
        point
    }

    pub fn fit_grid(&self, point: &Point) -> (usize, usize, f64, f64) {
        //ensure that the point is within the grid
        match self.inside_or_outside(&point) {
            Location::Outside => {
                println!("Tried to grid point {:?}", point);
                panic!("Tried to grid a point that was outside of the grid")
            }
            Location::Inside => {}
        }
        //find the nearest point and then return the residuals to it

        let (x, y) = point.convert(&self.coordinates).values();
        //  println!("{x} {y}");

        let (corner_x, corner_y) = self.corner;

        let delta_x = x - corner_x;
        let delta_y = y - corner_y;
        //  println!("delta x and delta y are {:?}, {:?}",delta_x,delta_y);
        let x_scaled_residual = delta_x / self.x_step_size - (delta_x / self.x_step_size).floor();
        let y_scaled_residual = delta_y / self.y_step_size - (delta_y / self.y_step_size).floor();

        // println!("residuals are {:?}, {:?}",x_scaled_residual,y_scaled_residual);

        let (x_mod, x_residual) = if x_scaled_residual <= 0.5 {
            let x_mod = (delta_x / self.x_step_size).floor() as usize;
            let x_residual = x_scaled_residual * self.x_step_size;

            (x_mod, x_residual)
        } else {
            let x_mod = (delta_x / self.x_step_size).floor() as usize + 1;
            let x_residual = (x_scaled_residual - 1.0) * self.x_step_size;
            (x_mod, x_residual)
        };

        let (y_mod, y_residual) = if y_scaled_residual <= 0.5 {
            let y_mod = (delta_y / self.y_step_size).floor() as usize;
            let y_residual = y_scaled_residual * self.y_step_size;
            (y_mod, y_residual)
        } else {
            let y_mod = (delta_y / self.y_step_size).floor() as usize + 1;
            let y_residual = (y_scaled_residual - 1.0) * self.y_step_size;
            (y_mod, y_residual)
        };

        //   println!("{:?}",(x_mod,y_mod,x_residual,y_residual));
        //residuals should be between -0.5 and 0.5 times the grid width

        (x_mod, y_mod, x_residual, y_residual)
    }
    //TODO remove redundancey of these two functions

    pub fn x_size(&self)->f64{
        self.x_size
    }

    pub fn y_size(&self)->f64{
        self.y_size
    }
    pub fn num_points(&self)-> usize{
        self.num_points
    }

    pub fn y_num(&self)-> usize{
        self.y_num
    }

    pub fn x_num(&self)-> usize{
        self.x_num
    }

    pub fn fit_grid_unscaled(&self, point: Point) -> (usize, usize, f64, f64) {
        //ensure that the point is within the grid
        match self.inside_or_outside(&point) {
            Location::Outside => {
                println!("Tried to grid point {:?}", point);
                panic!("Tried to grid a point that was outside of the grid")
            }
            Location::Inside => {}
        }
        //find the nearest point and then return the residuals to it

        let (x, y) = point.convert(&self.coordinates).values();
        //println!("{:?}corner {:?}",(x,y),(self.corner));
        let (corner_x, corner_y) = self.corner;

        let delta_x = x - corner_x;
        let delta_y = y - corner_y;
        //println!("DELTAS ARE {:?}",(delta_x,delta_y));
        //  println!("delta x and delta y are {:?}, {:?}",delta_x,delta_y);
        let x_scaled_residual = delta_x / self.x_step_size - (delta_x / self.x_step_size).floor();
        let y_scaled_residual = delta_y / self.y_step_size - (delta_y / self.y_step_size).floor();

        // println!("residuals are {:?}, {:?}",x_scaled_residual,y_scaled_residual);

        let (x_mod, x_residual) = if x_scaled_residual <= 0.5 {
            let x_mod = (delta_x / self.x_step_size).floor() as usize;
            let x_residual = x_scaled_residual;

            (x_mod, x_residual)
        } else {
            let x_mod = (delta_x / self.x_step_size).floor() as usize + 1;
            let x_residual = (x_scaled_residual - 1.0);
            (x_mod, x_residual)
        };

        let (y_mod, y_residual) = if y_scaled_residual <= 0.5 {
            let y_mod = (delta_y / self.y_step_size).floor() as usize;
            let y_residual = y_scaled_residual;
            (y_mod, y_residual)
        } else {
            let y_mod = (delta_y / self.y_step_size).floor() as usize + 1;
            let y_residual = (y_scaled_residual - 1.0);
            (y_mod, y_residual)
        };

        //   println!("{:?}",(x_mod,y_mod,x_residual,y_residual));
        //residuals should be between -0.5 and 0.5 times the grid width
        // println!("{:?} ",(x_mod,y_mod));
        (x_mod, y_mod, x_residual, y_residual)
    }

    pub fn bin_up_patch(&self, center_of_the_corner_pixel: Point, psf: &Vec<Vec<f64>>, scale: usize) -> ((usize, usize), Vec<Vec<f64>>) { //TODO make this grid dependent

        let (x_mod, y_mod, x_residual, y_residual) = self.fit_grid_unscaled(center_of_the_corner_pixel);
        //unscaled returns the raction of the pixel
        //TODO remove assumption that the psf is sven by even!!!
        //TODO remove the assumption that the scale is an intege
        //scale is the number of little pixels of the psf that fit into one big Detector pixel
        //x direction
        let x_pixels = 64; //TODO
        let y_pixels = 64;
        let scale = scale as f64;
        let x_tail_end_in_pixels = x_residual * scale + scale / 2.0 - 1.0;
        let y_tail_end_in_pixels = y_residual * scale + scale / 2.0 - 1.0;
        let x_offset = if x_tail_end_in_pixels < 0.0 { 0 } else {
            x_tail_end_in_pixels.ceil() as usize
        };
        let y_offset = if y_tail_end_in_pixels < 0.0 { 0 } else {
            y_tail_end_in_pixels.ceil() as usize
        };

        let binned_x_size = if x_offset > 0 {
            (x_pixels as f64 / scale + 1.0) as usize
        } else {
            (x_pixels as f64 / scale) as usize
        };

        let binned_y_size = if y_offset > 0 {
            (y_pixels as f64 / scale + 1.0) as usize
        } else {
            (y_pixels as f64 / scale) as usize
        };
        //  println!("Binned x size, y size is {:?}",((x_pixels as f64/scale).ceil() as usize + 1,(y_pixels as f64/scale).ceil() as usize + 1));

        let mut binned_psf = vec![vec![0.0; (x_pixels as f64 / scale).ceil() as usize + 1]; (y_pixels as f64 / scale).ceil() as usize + 1];
        for y in 0..y_pixels {
            for x in 0..x_pixels {
                let binned_x_index: usize = ((x + x_offset) as f64 / scale).floor() as usize;
                let binned_y_index: usize = ((y + y_offset) as f64 / scale).floor() as usize;
                binned_psf[binned_y_index][binned_x_index] += psf[y][x];
            }
        }
        //  println!(" {:?} MODULUSES ER {:?}",center_of_the_corner_pixel,(x_mod,y_mod));
        ((x_mod, y_mod), binned_psf)
    }


    pub fn inside_or_outside(&self, point: &Point) -> Location {
        let (x, y) = point.convert(&self.coordinates).values();
        // println!("POint in values: {:?}", (x,y));
        let epsilon = self.snap_precision;
        let (corner_x, corner_y) = self.corner;
        let (grid_x_min, grid_x_max) = (corner_x, corner_x + self.x_size);
        let (grid_y_min, grid_y_max) = (corner_y, corner_y + self.y_size);

        if (x < grid_x_min - epsilon) | (x > grid_x_max + epsilon) | (y < grid_y_min - epsilon) | (y > grid_y_max + epsilon) {
            //println!("Point was outside of the grid, min x is {grid_x_min}, max x is {grid_x_max}, min y is {grid_y_min}, max y is {grid_y_max}");
            return Location::Outside
        };
        Location::Inside
    }

    pub fn project(&self, point: &Point) -> Point {
        //The idea is that we may want to get a value from a data grid or something, but we try to acces
        //a poitn outside of the grid - we want to return a point which is the "projection" of that point on to the grid.
        //So if the x value is too low, we want to return the minimum x, if the x value is too high, we want to return the maximum x
        //same with y. In this way, the point location is "clipped" to fit the grid so that we can get the corresponding data.
        //in order to find interpolation coefficients, we can first project the point into the grid and then preform the interpolation at that point.
        //simple!
        //This requires less shuffling around of the architecture.

        let (x, y) = point.convert(&self.coordinates).values();
        let (corner_x, corner_y) = self.corner;
        let (grid_x_min, grid_x_max) = (corner_x, corner_x + self.x_size);
        let (grid_y_min, grid_y_max) = (corner_y, corner_y + self.y_size);

        let new_x = if x < grid_x_min {
            grid_x_min
        } else if x > grid_x_max {
            grid_x_max
        } else {
            x
        };
        let new_y = if y < grid_y_min {
            grid_y_min
        } else if y > grid_y_max {
            grid_y_max
        } else {
            y
        };
        Point::new(new_x, new_y, self.coordinates.clone())
    }


    pub fn find_corners(&self, point: Point) -> Corners {
        let epsilon = self.snap_precision;


        let (x_mod, y_mod, x_residual, y_residual) = self.fit_grid(&point);

        //first we check to see if it is most appropriate to snap this point to a grid point:
        if (x_residual.abs() <= epsilon) && (y_residual.abs() <= epsilon) {
            // println!("Snapped to grid point");
            return Corners::One(self.grid_number(x_mod, y_mod))
        };
        //Is the point between two vertical grid points?
        if (x_residual.abs() <= epsilon) {
            //println!("Snapped between vertical points");
            if y_residual < 0.0 {
                return Corners::Two(self.grid_number(x_mod, y_mod), self.grid_number(x_mod, y_mod - 1));
            } else {
                return Corners::Two(self.grid_number(x_mod, y_mod + 1), self.grid_number(x_mod, y_mod));
            }
        };
        //Is the point between two horizontal grid points?
        if (y_residual.abs() <= epsilon) {
            //  println!("Snapped between horizontal points");
            if x_residual < 0.0 {
                return Corners::Two(self.grid_number(x_mod, y_mod), self.grid_number(x_mod - 1, y_mod));
            } else {
                return Corners::Two(self.grid_number(x_mod + 1, y_mod), self.grid_number(x_mod, y_mod));
            }
        };
        //The point must be in the middle

        let (upper_x, lower_x) = if x_residual < 0.0 { (x_mod, x_mod - 1) } else { (x_mod + 1, x_mod) };
        let (upper_y, lower_y) = if y_residual < 0.0 { (y_mod, y_mod - 1) } else { (y_mod + 1, y_mod) };

        // println!("Finding four corners, enumerated clockwise starting top left");
        Corners::Four(self.grid_number(lower_x, upper_y),
                      self.grid_number(upper_x, upper_y),
                      self.grid_number(upper_x, lower_y),
                      self.grid_number(lower_x, lower_y))
    }

    pub fn plot_points(&self, plot: &mut Plot, add_point: PlotPoint) {
        //self.coordinates.plot(plot, )

        let mut grid_points = Curve::new();
        grid_points.set_line_style("none")
            .set_label(format!("Grid points: {:?}", self.label).as_str())
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

        let mut frame = Curve::new();
        frame.set_line_width(1.0)
            .set_label("Frame")
            .set_line_style("solid")
            .set_line_width(1.0)
            .set_marker_color("purple")
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
        for point in 0..self.num_points {
            let point_location = self.locate(point).as_absolute();
            grid_points.points_add(point_location.x, point_location.y);
            let label = format!("{}", point);
            grid_numbers.draw(point_location.x, point_location.y, label.as_str());
        }
        grid_points.points_end();


        corner.points_begin();
        let corner_location = self.locate(0).as_absolute().values();
        let corner_label = format!("Corner: ({:.3},{:.3})", corner_location.0, corner_location.1);
        corner.points_add(corner_location.0, corner_location.1).set_label(corner_label.as_str());
        corner.points_end();

        let mut example_point = Vec::new();

        match add_point {
            PlotPoint::No => {}
            PlotPoint::Given(point) => { example_point.push(point) }
            PlotPoint::Random => {
                let random = self.random();
                example_point.push(random);
            }
        };

        for point in example_point {
            let (_, _, x_res, y_res) = self.fit_grid(&point);
            extra_point.points_begin();
            extra_point.points_add(point.as_absolute().values().0, point.as_absolute().clone().values().1).set_label(format!("x, y residuals: {:.3}, {:.3}", x_res, y_res).as_str());
            extra_point.points_end();

            let corners = self.find_corners(point);
            let mut frame_points = Vec::new();
            match corners {
                Corners::Four(a, b, c, d) => {
                    frame_points.push(a);
                    frame_points.push(b);
                    frame_points.push(c);
                    frame_points.push(d);
                    frame_points.push(a);
                }
                Corners::Two(a, b) => {
                    frame_points.push(a);
                    frame_points.push(b);
                }
                Corners::One(a) => { frame_points.push(a); }
            }
            frame.points_begin();
            for point in frame_points {
                //println!("point {point}");
                let point = self.locate(point).as_absolute();
                frame.points_add(point.x, point.y);
            }

            frame.points_end();
        }

        plot.add(&frame);
        plot.add(&grid_numbers);
        plot.add(&grid_points);
        plot.add(&extra_point);
        plot.add(&corner)
            .set_figure_size_inches(10.0, 10.0)
            .set_num_ticks_y(self.y_num + 1)
            .set_num_ticks_x(self.x_num + 1)
            .grid_labels_legend("x", "y");
    }

    pub fn outer_boarder(&self) -> (Point, Point, Point, Point) {
        let (c1x, c1y) = (self.corner.0 - self.x_step_size / 2.0, self.corner.1 - self.y_step_size / 2.0);

        let (c2x, c2y) = (c1x + self.x_size + self.x_step_size, c1y);
        let (c3x, c3y) = (c1x + self.x_size + self.x_step_size, c1y + self.y_size + self.y_step_size);
        let (c4x, c4y) = (c1x, c1y + self.y_size + self.y_step_size);

        let point = Point::new(c1x, c1y, self.coordinates.clone());
        // println!("CORNER IS AT {:?}",(self.corner,point.to_absolute()));
        (Point::new(c1x, c1y, self.coordinates.clone()),
         Point::new(c2x, c2y, self.coordinates.clone()),
         Point::new(c3x, c3y, self.coordinates.clone()),
         Point::new(c4x, c4y, self.coordinates.clone()))
    }

    pub fn plot_outline(&self, plot: &mut Plot, color: String) {
        self.coordinates.plot(plot, color.clone());

        //let color = match &self.coordinates{ Coordinates::ABSOLUTE => "black", Coordinates::RELATIVE(c)=> c.color.as_str()};


        let mut outline = Curve::new();
        outline.set_line_width(1.0)
            .set_label(format!("Frame of {:?}", self.label).as_str())
            .set_line_style("solid")
            .set_line_width(1.0)
            .set_marker_color(&*color)
            .set_line_color(&*color)
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_marker_style(".");


        outline.points_begin();

        let (p0, p1, p2, p3) = self.outer_boarder();
        let p0 = p0.as_absolute();
        let p1 = p1.as_absolute();
        let p2 = p2.as_absolute();
        let p3 = p3.as_absolute();
        outline.points_add(p0.x, p0.y);
        outline.points_add(p1.x, p1.y);
        outline.points_add(p2.x, p2.y);
        outline.points_add(p3.x, p3.y);
        outline.points_add(p0.x, p0.y);

        outline.points_end();


        plot.add(&outline);
    }

    pub fn projected_interpolation_coefficients(&self, point: &Point) -> InterpolationData {
        self.interpolation_coefficients(&self.project(point))
    }


    pub fn interpolation_coefficients(&self, point: &Point) -> InterpolationData {
        match self.find_corners(point.clone()) {
            Corners::Four(Q12, Q22, Q21, Q11) => { // using the wikipedia convention https://en.wikipedia.org/wiki/Bilinear_interpolation

                let Q11point = self.locate(Q11);
                let Q22point = self.locate(Q22);

                let (x1, y1) = (Q11point.x, Q11point.y);
                let (x2, y2) = (Q22point.x, Q22point.y);
                let (x, y) = point.convert(&self.coordinates).values();
                let c11 = (x2 - x) * (y2 - y);
                let c12 = (x2 - x) * (y - y1);
                let c21 = (x - x1) * (y2 - y);
                let c22 = (x - x1) * (y - y1);
                //  println!("The coefficients are {:?}",(c11,c12,c21,c22));
                let normalization = (x2 - x1) * (y2 - y1);
                InterpolationData {
                    corners: Corners::Four(Q12, Q22, Q21, Q11),
                    coefficients: vec![c11, c12, c21, c22],
                    normalization
                }
            }
            Corners::Two(Q1, Q2) => {
                let Q1point = self.locate(Q1);
                let Q2point = self.locate(Q2);

                let (x1, y1) = (Q1point.x, Q1point.y);
                let (x2, y2) = (Q2point.x, Q2point.y);
                let (x, y) = point.convert(&self.coordinates).values();

                if ((x1 - x2) < 2.0 * self.snap_precision) && ((y1 - y2) > 2.0 * self.snap_precision) {
                    //the corners are above and below the point
                    let c1 = (y1 - y).abs();
                    let c2 = (y2 - y).abs();
                    let normalization = self.y_step_size;
                    return InterpolationData {
                        corners: Corners::Two(Q1, Q2),
                        coefficients: vec![c1, c2],
                        normalization
                    }
                } else if ((x1 - x2) > 2.0 * self.snap_precision) && ((y1 - y2) < 2.0 * self.snap_precision) {
                    //the corners are to the sides of the point
                    let c1 = (x1 - x).abs();
                    let c2 = (x2 - x).abs();
                    let normalization = self.x_step_size;
                    return InterpolationData {
                        corners: Corners::Two(Q1, Q2),
                        coefficients: vec![c1, c2],
                        normalization
                    }
                } else {
                    panic!("Unreachable")
                }
            }
            Corners::One(Q1) => {
                InterpolationData {
                    corners: Corners::One(Q1),
                    coefficients: vec![1.0],
                    normalization: 1.0,
                }
            }
        }
    }
}

pub struct InterpolationData {
    pub corners: Corners,
    pub coefficients: Vec<f64>,
    pub normalization: f64,
}




