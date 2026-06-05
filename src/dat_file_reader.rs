use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::time::Instant;
use plotpy::{Curve, Plot};
use astroimsim_geometry::grid1d::{Location1D, Neighbors, GRID1D};

#[derive(Clone, Debug)]
pub struct FrequencyFile {
    pub path: &'static str,
    pub grid1d: GRID1D,
    pub data: Vec<(usize,Vec<f32>)>
}

impl FrequencyFile{








    pub fn plot(&self) -> Vec<Vec<Vec<f64>>>{
        let mut curves = Vec::new();
        for i in 1..self.data[0].1.len() {
            let mut curve = Vec::new();

            /*
            curve.set_line_style(line)

            .set_marker_line_width(2.5)
            .set_marker_size(4.0)
                .set_marker_color(color)
                .set_line_color(color)
            .set_marker_style(".");

             */

            // curve.points_begin();
            for point in self.data.clone() {
                // println!("{:?}", point);
                println!("adding {:?}",(point.1[0],point.1[i]));
                curve.push(vec![point.1[0] as f64, point.1[i] as f64]);
            }
            //curve.points_end();
            curves.push(curve);
        }
        println!("{:?}",self.data);
        curves
    }

}



