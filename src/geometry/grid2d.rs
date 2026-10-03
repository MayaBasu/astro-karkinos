use crate::geometry::geometry::*;
use plotpy::{Curve, Plot, Text};
use rand::RngExt;
use std::fmt;
use std::ops::Mul;
//gridding

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct GriddingError;
impl fmt::Display for GriddingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " issue with griding point")
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Corners {
    Four(usize, usize, usize, usize),
    Two(usize, usize),
    One(usize),
}


#[derive(Debug, Clone, Copy)]
pub struct GRID2D {
    coordinate_system: CoordinateSystem,
    xy_num: [usize;2],
    xy_step_size: [Regular;2],
    center: Point,
    snap_precision: Regular,
}

impl Mul<Regular> for f64 {
    type Output = Regular;

    fn mul(self, rhs: Regular) -> Self::Output {
        let value = Regular::try_from(self);
        value*rhs
    }
}

impl GRID2D {
    pub fn new([x_num, y_num]: [usize;2],
               [x_step_size, y_step_size]: [f64;2],
               center: Point,
               relative_snap_precision: f64,
               coordinate_system: CoordinateSystem,
    ) -> GRID2D {

        let xy_num = [x_num,y_num];
        assert_ne!(x_num,0, "Number of grid points in x dimension must be 1 or greater");
        assert_ne!(y_num,0, "Number of grid points in x dimension must be 1 or greater");

        let xy_step_size = [Regular::try_from(x_step_size),Regular::try_from(y_step_size)];
        assert!(x_step_size > 0.0, "x step size must be strictly positive");
        assert!(y_step_size > 0.0, "y step size must be strictly positive");

        center.transform_to(&coordinate_system);

        assert!(relative_snap_precision < 0.5, "Relative snap precision must be between 0 and 0.5");
        assert!(relative_snap_precision > 0.0, "Relative snap precision must be between 0 and 0.5");

        let snap_precision = Regular::try_from(
            x_step_size.min(y_step_size)*relative_snap_precision);

        GRID2D {
            xy_num,
            xy_step_size,
            center,
            snap_precision,
            coordinate_system
        }
    }
    pub fn coordinate_system(&self)-> &CoordinateSystem{
        &self.coordinate_system
    }
    pub fn xy_num(&self)-> [usize;2]{
        self.xy_num
    }
    pub fn y_num(&self)->usize {self.xy_num[1]}
    pub fn x_num(&self)->usize {self.xy_num[0]}
    pub fn center(&self)->Point{
        self.center
    }
    pub fn xy_step_size(&self) -> Point{
        Point::new_from_regular(
            self.xy_step_size[0],
            self.xy_step_size[1],
            self.coordinate_system()
        )
    }
    pub fn size(&self) -> Point{
        let [x_num,y_num] = self.xy_num;
        let [x_step,y_step] = self.xy_step_size;
        Point::new_from_regular(
            (x_num-1) as f64 *x_step,
            (y_num-1) as f64 *y_step,
            self.coordinate_system())
    }
    
    pub fn x_size(&self)->f64{self.size().x.value()}
    pub fn y_size(&self)->f64{self.size().y.value()}
    pub fn snap_precision(&self)-> f64{

        self.snap_precision.value()
    }
    pub fn num_points(&self)->usize{
        self.xy_num[0]*self.xy_num[1]
    }
    pub fn corner(&self)->Point{
        let half_size = self.size()/2.0;
        self.center-half_size

    }
    pub fn xy_indices(&self, grid_number: usize) -> [usize;2] {
        assert!((grid_number <= self.num_points() - 1));
        let x_index = grid_number % self.xy_num[0];
        let y_index = (grid_number - x_index) / self.xy_num[0];
        [x_index, y_index]
    }
    pub fn grid_number(&self, x_index: usize, y_index: usize) -> usize {
        assert!(x_index <= self.xy_num[0] - 1);
        assert!(y_index <= self.xy_num[1] - 1);
        y_index * self.xy_num[0] + x_index
    }
    pub fn locate(&self, grid_number: usize) -> Point {
        let [x_index, y_index] = self.xy_indices(grid_number);
       // println!("{:?}  {:?}",[x_index,y_index],self.xy_step_size);
        let delta_x = x_index*self.xy_step_size[0];
        let delta_y = y_index*self.xy_step_size[1];
       // println!("{delta_x} {delta_y}");
        let corner = self.corner().transform_to(&self.coordinate_system);
        //println!("{:?} {:?}  {:?}",delta_y,delta_x,corner);

        Point::new_from_regular(
            corner.x + delta_x,
            corner.y + delta_y,
            self.coordinate_system()
        )
    }
    
    pub fn locate_grid_points(&self)->Vec<Vec<Point>>{
        (0..self.y_num()).map(|y|{
            (0..self.x_num()).map(|x|{
                let grid_point = self.grid_number(x,y);
                self.locate(grid_point)
            }).collect()
        }).collect()
    }
    pub fn random(&self) -> Point {
        let mut rng = rand::rng();
        let x_scale: f64 = rng.random();
        let y_scale: f64 = rng.random();
        Point::new_from_regular(
            self.corner().x + self.size().x * x_scale,
            self.corner().y + self.size().y * y_scale,
            self.coordinate_system()
        )
    }
    pub fn is_point_inside(&self, point:&Point)-> bool{
        let half_grid_size = self.size()/2.0;
        let delta = point - &self.center();
        if (delta.x.value().abs() > (half_grid_size.x + self.snap_precision).value())|
            (delta.y.value().abs() > (half_grid_size.y + self.snap_precision).value()){
            false
        }else{
            true
        }

    }

    //TODO: Could maybe clean up this function
    pub fn project_inside(&self, point: &Point) -> Point {
        let corner = self.corner();

        let new_x = if point.x < corner.x - self.snap_precision {
            corner.x
        } else if point.x > corner.x + self.size().x + self.snap_precision {
            corner.x + self.size().x
        } else {
            point.x
        };
        let new_y = if point.y < corner.y - self.snap_precision {
            corner.y
        } else if point.y > corner.y + self.size().y + self.snap_precision{
            corner.y + self.size().y
        } else {
            point.y
        };
        Point::new_from_regular(
            new_x,
            new_y,
            self.coordinate_system())
    }

    pub fn project_fit(&self, point:&Point,fractional_residual:bool)->(usize,usize,f64,f64){
        let point = self.project_inside(point);
        self.fit_grid(&point, fractional_residual).unwrap()
    }
    pub fn fit_grid(&self, point:&Point, fractional_residual: bool) -> Result<(usize,usize,f64,f64), GriddingError>{
        if !self.is_point_inside(point){
            println!("Failed to grid point due to it being outside of the grid");
            return Err(GriddingError)
        };
        let delta = point - &self.corner();
        let step = Point::new_from_regular(
            self.xy_step_size[0],
            self.xy_step_size[1],
            self.coordinate_system());
        let modulus = (delta/step).round();
        let residual =
        if fractional_residual{
            delta/step-modulus
        }else{
            delta-modulus*step
        };

        Ok((modulus.x.value() as usize,
            modulus.y.value() as usize,
            residual.x.value(),
            residual.y.value()))

    }

    pub fn snap(&self, point: &Point) -> Result<usize, GriddingError> {
        match self.fit_grid(point,false){
            Ok((x_mod,y_mod,x_res,y_res)) => {
                if (x_res.abs() > self.snap_precision()) | (y_res.abs() > self.snap_precision()){
                    println!("Failed to snap point due to it being further than the grid's snap_precision from any grid point");
                    return Err(GriddingError)
                }
                Ok(self.grid_number(x_mod,y_mod))
            }
            Err(_) => {
                println!("Could not snap point to grid due to it being outside of the grid");
                Err(GriddingError) }
        }
    }





    pub fn bin_up_patch(&self, center_of_the_corner_pixel: Point, psf: &Vec<Vec<f64>>, scale: usize) -> ((usize, usize), Vec<Vec<f64>>) { //TODO make this grid dependent

        let (x_mod, y_mod, x_residual, y_residual) = match self.fit_grid(&center_of_the_corner_pixel, true) {
            Ok((x_mod,y_mod,x_res,y_res)) => {(x_mod,y_mod,x_res,y_res)}
            Err(_) => {panic!("Could not bin up patch due to griding error")}
        };

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





    pub fn find_corners(&self, point: &Point) -> Result<Corners, GriddingError> {
        let (x_mod,y_mod,x_res,y_res) = match self.fit_grid(point,false){
            Ok((x_mod,y_mod,x_res,y_res)) => {(x_mod,y_mod,x_res,y_res)}
            Err(err) => {return Err(err)}
        };
        println!("{:?}",(x_mod,y_mod,x_res,y_res));

        if (x_res.abs() < self.snap_precision()) & (y_res.abs() < self.snap_precision()){
            return Ok(Corners::One(self.grid_number(x_mod,y_mod)))
        }

        let delta = point - &self.corner();
        let step = Point::new_from_regular(
            self.xy_step_size[0],
            self.xy_step_size[1],
            self.coordinate_system());
        let lower_left_corner =
            [(delta/step).floor().x.value() as usize,
                (delta/step).floor().y.value() as usize];



        if (x_res.abs() < self.snap_precision()){
            return Ok(Corners::Two(
                self.grid_number(x_mod, lower_left_corner[1]),
                self.grid_number(x_mod, lower_left_corner[1]+1)
            ))
        }
        if (y_res.abs() < self.snap_precision()) {
            Ok(Corners::Two(
                self.grid_number(lower_left_corner[0], y_mod),
                self.grid_number(lower_left_corner[0] + 1, y_mod)
            ))
        } else {
            Ok(Corners::Four(
                self.grid_number(lower_left_corner[0], lower_left_corner[1]),
                self.grid_number(lower_left_corner[0] ,lower_left_corner[1]+1),
                self.grid_number(lower_left_corner[0] + 1, lower_left_corner[1]+1),
                self.grid_number(lower_left_corner[0] +1, lower_left_corner[1])

            ))
        }
    }

    pub fn plot_grid_points(&self, plot: &mut Plot, color:String, label:String) {
        let mut grid_points = Curve::new();
        grid_points.set_line_style("none")
            .set_label(format!("Grid points: {:?}", label).as_str())
            .set_marker_color(&*color)
            .set_marker_every(1)
            .set_marker_size(7.0)
            .set_marker_style(".");

        let mut corner = Curve::new();
        corner
            .set_label("Corner")
            .set_line_style("none")
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_marker_style(".");


        let mut grid_numbers = Text::new();
        grid_numbers.set_color(&*color)
            .set_fontsize(5.0);
        println!("ok {:?}",self.num_points());


        grid_points.points_begin();
        for point in 0..self.num_points() {
            println!("ok");
            let point_location = self.locate(point).to_absolute();
            println!(" {:?}",point_location);
            grid_points.points_add(point_location.x.value(), point_location.y.value());
            let label = format!("{}", point);
            grid_numbers.draw(point_location.x.value(), point_location.y.value(), label.as_str());
        }
        grid_points.points_end();
        println!("ok");


        corner.points_begin();
        let corner_location = self.locate(0).to_absolute().values();
        let corner_label = format!("Corner: ({:.3},{:.3})", corner_location[0], corner_location[1]);
        corner.points_add(corner_location[0], corner_location[1]).set_label(corner_label.as_str());
        corner.points_end();



        plot.add(&grid_numbers);
        plot.add(&grid_points);

        plot.add(&corner)
            .set_figure_size_inches(10.0, 10.0)
            .set_num_ticks_y(self.xy_num[1] + 1)
            .set_num_ticks_x(self.xy_num[0] + 1)
            .grid_labels_legend("x", "y");
    }

    pub fn plot_point(&self, plot: &mut Plot, point:&Point){

        let mut frame = Curve::new();
        frame.set_line_width(1.0)
            .set_label("Frame")
            .set_line_style("solid")
            .set_line_width(1.0)
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_marker_style(".");


        let corners = match self.find_corners(point) {
            Ok(corners) => {match corners{
                Corners::Four(c1, c2, c3, c4) => {vec![c1,c2,c3,c4]}
                Corners::Two(c1, c2) => {vec![c1,c2]}
                Corners::One(c) => {vec![c]}
            }}
            Err(_) => {vec![]}
        };

        frame.points_begin();
        for corner in corners {
            let point = self.locate(corner);
            point.to_absolute();
            frame.points_add(point.x.value(), point.y.value());
        }
        frame.points_end();


        let mut extra_point = Curve::new();
        extra_point
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_line_style("none")
            .set_marker_style("*");

        extra_point.points_begin();
        let point = point.to_absolute();
        extra_point.points_add(point.x.value(), point.y.value());
        extra_point.points_end();
        plot.add(&extra_point);
        plot.add(&frame);

    }
    pub fn plot(&self, plot:&mut Plot, color:&str,label:&str){
        self.plot_outline_2(plot,color.to_string(),label.to_string());
        self.plot_grid_points(plot,color.to_string(),label.to_string());
    }


    pub fn plot_outline_2(&self, plot: &mut Plot, color: String, label: String) {

        let mut outline = Curve::new();
        outline.set_line_width(1.0)
            .set_label(format!("Frame of {:?}", label).as_str())
            .set_line_style("solid")
            .set_line_width(1.0)
            .set_marker_color(&*color)
            .set_line_color(&*color)
            .set_marker_every(1)
            .set_marker_size(10.0)
            .set_marker_style(".");

        outline.points_begin();


        let p0 = self.locate(self.grid_number(0,0)).to_absolute();
        let p1 = self.locate(self.grid_number(0,self.y_num()-1)).to_absolute();
        let p2 = self.locate(self.grid_number(self.x_num()-1,self.y_num()-1)).to_absolute();
        let p3 = self.locate(self.grid_number(self.x_num()-1,0)).to_absolute();
        p0.add_to_curve(&mut outline);
        p1.add_to_curve(&mut outline);
        p2.add_to_curve(&mut outline);
        p3.add_to_curve(&mut outline);
        p0.add_to_curve(&mut outline);

        outline.points_end();


        plot.add(&outline);
    }

    pub fn projected_interpolation_coefficients(&self, point: &Point) -> InterpolationData {
        self.interpolation_coefficients(&self.project_inside(point))
    }


    pub fn interpolation_coefficients(&self, point: &Point) -> InterpolationData {
        match self.find_corners(point) {
            Ok(Corners::Four(Q12, Q11, Q21, Q22 )) => { // using the wikipedia convention https://en.wikipedia.org/wiki/Bilinear_interpolation

                let Q12point = self.locate(Q12);
                let Q21point = self.locate(Q21);


                let (x1, y1) = (Q12point.x, Q12point.y);
                let (x2, y2) = (Q21point.x, Q21point.y);
                point.transform_to(&self.coordinate_system);
                let x = point.x;
                let y = point.y;
                let c11 = -(x2 - x) * (y2 - y);
                let c12 = -(x2 - x) * (y - y1);
                let c21 = -(x - x1) * (y2 - y);
                let c22 = -(x - x1) * (y - y1);
                println!("The coefficients are {:?}",(c11.value(),c12.value(),c21.value(),c22.value()));
                //let normalization = (x2 - x1) * (y2 - y1);
                let normalization = self.xy_step_size[0]*self.xy_step_size[1];
                println!("CORNERS {:?} {:?} {:?} {:?}",Q12, Q22, Q21, Q11);
                println!("norm {:?}",normalization.value());
                if normalization.value() ==0.0{
                    println!("NORM IS 0");

                }
                InterpolationData {
                    corners: Corners::Four(Q12, Q22, Q21, Q11),
                    coefficients: vec![c11, c12, c21, c22],
                    normalization
                }
            }
            Ok(Corners::Two(Q1, Q2)) => {
                let Q1point = self.locate(Q1);
                let Q2point = self.locate(Q2);

                let (x1, y1) = (Q1point.x, Q1point.y);
                let (x2, y2) = (Q2point.x, Q2point.y);
                println!("pos {:?} {:?} {:?} {:?}",x1.value(),y1.value(),x2.value(),y2.value());

                point.transform_to(&self.coordinate_system);
                let x = point.x;
                let y = point.y;

                if ((x1 - x2).abs() <  2.0*self.snap_precision) && ((y1 - y2).abs() > 2.0*self.snap_precision) {
                    //the corners are above and below the point
                    println!("corners are above and below");
                    let c1 = (y1 - y).abs();
                    let c2 = (y2 - y).abs();
                    let normalization = self.xy_step_size[1];
                    return InterpolationData {
                        corners: Corners::Two(Q1, Q2),
                        coefficients: vec![c1, c2],
                        normalization
                    }
                } else if ((x1 - x2).abs() > 2.0*self.snap_precision) && ((y1 - y2).abs() < 2.0*self.snap_precision) {
                    //the corners are to the sides of the point
                    let c1 = (x1 - x).abs();
                    let c2 = (x2 - x).abs();
                    let normalization = self.xy_step_size[0];
                    return InterpolationData {
                        corners: Corners::Two(Q1, Q2),
                        coefficients: vec![c1, c2],
                        normalization
                    }
                } else {
                    println!("point: {:?}",point);
                    println!("point: {:?}",self.find_corners(&point));
                    panic!("Unreachable ")
                }
            }
            Ok(Corners::One(Q1)) => {
                InterpolationData {
                    corners: Corners::One(Q1),
                    coefficients: vec![Regular::try_from(1.0)],
                    normalization: Regular::try_from(1.0),
                }
            }
            _ => {panic!("Griding error")}
        }
    }
}



pub struct InterpolationData {
    pub corners: Corners,
    pub coefficients: Vec<Regular>,
    pub normalization: Regular,
}



#[cfg(test)]
pub mod tests{
    use crate::geometry::Corners::One;
    use crate::geometry::GRID1D;
    use super::*;


    #[test]
    pub fn test_grid_2d(){
        let coord = CoordinateSystem::new(
            [0.2,0.0],
            [0.0,0.5],
            [1.0,0.5]
        );
        let grid = GRID2D::new([10,9],
                               [1.,2.],
                               Point::new(0.,0.,&coord),
                               0.00001,
                               coord);

        assert_eq!(grid.xy_step_size(),Point::new(1.,2.,&coord));
        assert_eq!(grid.y_num(),9);
        assert_eq!(grid.x_num(),10);
        assert_eq!(grid.xy_num(), [10,9]);
        assert_eq!(grid.snap_precision(), 0.00001);
        assert_eq!(grid.center(),Point::new(0.,0.,&coord));
        assert_eq!(*grid.coordinate_system(),coord);
        assert_eq!(grid.size(),Point::new(9.,16.,&coord));
        assert_eq!(grid.y_size(), 16.);
        assert_eq!(grid.x_size(), 9.);
        assert_eq!(grid.num_points(), 90);
        assert_eq!(grid.corner(), Point::new(-4.5,-8., &coord));
        assert_eq!(grid.xy_indices(0), [0,0]);
        assert_eq!(grid.xy_indices(1),[1,0]);
        assert_eq!(grid.xy_indices(10),[0,1]);
        assert_eq!(grid.xy_indices(89),[9,8]);
        assert_eq!(grid.grid_number(4,3),3*10+4);
        assert_eq!(grid.locate(0),Point::new(-4.5,-8., &coord));
        assert_eq!(grid.locate(9),Point::new(4.5,-8., &coord));
        assert_eq!(grid.locate(89),Point::new(4.5,8., &coord));
        assert!(grid.random().x>grid.corner().x);
        assert!(grid.random().y>grid.corner().y);

        assert!(grid.random().x<grid.corner().x + Regular::try_from(grid.x_size()));
        assert!(grid.random().y<grid.corner().y + Regular::try_from(grid.y_size()));
        assert!(!grid.is_point_inside(&Point::new(-4.6,-8.,&coord)));
        assert!(grid.is_point_inside(&Point::new(-4.5,-8.,&coord)));

        assert!(!grid.is_point_inside(&Point::new(4.6,-8.,&coord)));
        assert!(!grid.is_point_inside(&Point::new(-4.5,-8.2,&coord)));

        assert!(grid.is_point_inside(&Point::new(-4.5,8.00001,&coord)));
        assert!(grid.is_point_inside(&Point::new(-4.5,-4.,&coord)));

        assert!(grid.is_point_inside(&Point::new(-4.3,-8.,&coord)));
        assert!(grid.is_point_inside(&Point::new(4.5,8.,&coord)));

        assert_eq!(grid.project_inside(&Point::new(-4.6,-8.,&coord)),Point::new(-4.5,-8.,&coord));
        assert_eq!(grid.project_inside(&Point::new(4.6,-8.,&coord)),Point::new(4.5,-8.,&coord));
        assert_eq!(grid.project_inside(&Point::new(4.6,-8.5,&coord)),Point::new(4.5,-8.,&coord));

        assert_eq!(grid.project_inside(&Point::new(4.4,-8.5,&coord)),Point::new(4.4,-8.,&coord));



        let grid = GRID2D::new([2,2],
                               [1.,2.],
                               Point::new(0.,0.,&coord),
                               0.00001,
                               coord);
        let points = vec![
            vec![Point::new(-0.5,-1.,&coord),Point::new(0.5,-1.,&coord)],
            vec![Point::new(-0.5,1.,&coord),Point::new(0.5,1.,&coord)]];
        assert_eq!(grid.locate_grid_points(), points);
        check_fit(grid.fit_grid(&Point::new(-0.25,-0.5,&coord),true).unwrap(),(0,0,0.25,0.25));
        check_fit(grid.fit_grid(&Point::new(0.25,-0.5,&coord),true).unwrap(),(1,0,-0.25,0.25));
        check_fit(grid.fit_grid(&Point::new(0.25,0.5,&coord),true).unwrap(),(1,1,-0.25,-0.25));
        check_fit(grid.fit_grid(&Point::new(-0.25,0.5,&coord),true).unwrap(),(0,1,0.25,-0.25));
        check_fit(grid.fit_grid(&Point::new(-0.16,0.5,&coord),true).unwrap(),(0,1,0.5-0.16,-0.25));

        assert_eq!(grid.fit_grid(&Point::new(4.4,-8.5,&coord),false), Err(GriddingError));


        pub fn check_fit(fit_1:(usize,usize,f64,f64),fit_2:(usize,usize,f64,f64)){
            assert_eq!(fit_1.0, fit_2.0);
            assert_eq!(fit_1.1,fit_2.1);
            assert!((fit_1.3-fit_2.3).abs()<NUMERICAL_PRECISION, " {:?} {:?}",fit_1.3,fit_2.3);
            assert!((fit_1.2-fit_2.2).abs()<NUMERICAL_PRECISION," {:?} {:?}",fit_1.2,fit_2.2);

        }

        let grid = GRID2D::new([3,3],
                               [1.,2.],
                               Point::new(0.,0.,&coord),
                               0.00001,
                               coord);

        check_fit(grid.fit_grid(&Point::new(-0.16,0.05,&coord),false).unwrap(),(1,1,-0.16,0.05));
        check_fit(grid.fit_grid(&Point::new(0.16,-0.05,&coord),false).unwrap(),(1,1,0.16,-0.05));
        check_fit(grid.fit_grid(&Point::new(0.16,-0.05,&coord),true).unwrap(),(1,1,0.16,-0.05/2.0));


        assert_eq!(grid.snap(&Point::new(0.000000016,-0.000000005,&coord)),Ok(4));
        assert_eq!(grid.snap(&Point::new(-1.000000016,-2.000000005,&coord)),Ok(0));
        assert_eq!(grid.snap(&Point::new(-0.000000016,-2.000000005,&coord)),Ok(1));
        assert_eq!(grid.snap(&Point::new(-1.000000016,-0.000000005,&coord)),Ok(3));


        assert_eq!(grid.find_corners(&Point::new(0.000000016,-0.000000005,&coord)),Ok(One(4)));
        assert_eq!(grid.find_corners(&Point::new(-1.000000016,-2.000000005,&coord)),Ok(One(0)));
        assert_eq!(grid.find_corners(&Point::new(-0.000000016,-2.000000005,&coord)),Ok(One(1)));
        assert_eq!(grid.find_corners(&Point::new(-1.000000016,-0.000000005,&coord)),Ok(One(3)));


        assert_eq!(grid.snap(&Point::new(-2.000000016,-0.000000005,&coord)),Err(GriddingError));
        assert_eq!(grid.snap(&Point::new(0.800000016,-0.200000005,&coord)),Err(GriddingError));
        assert_eq!(grid.snap(&Point::new(0.003000016,0.007000005,&coord)),Err(GriddingError));


        assert_eq!(grid.find_corners(&Point::new(-0.9,-1.9,&coord)),Ok(Corners::Four(0,3,4,1)));
        assert_eq!(grid.find_corners(&Point::new(0.0,-1.5,&coord)),Ok(Corners::Two(1,4)));
        assert_eq!(grid.find_corners(&Point::new(0.7,-1.5,&coord)),Ok(Corners::Four(1,4,5,2)));
        assert_eq!(grid.find_corners(&Point::new(-0.7,1.5,&coord)),Ok(Corners::Four(3,6,7,4)));
        assert_eq!(grid.find_corners(&Point::new(0.007,1.5,&coord)),Ok(Corners::Four(4,7,8,5)));











    }

}



