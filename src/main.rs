
use clap::Parser;
use crate::geometry::{generate_notebook};

pub mod geometry;
pub mod tests;

pub mod gridded_data;



pub fn main() {
   // test_coordinate_system();
    generate_notebook()

}






