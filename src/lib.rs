pub mod telescope;
pub mod geometry;
pub mod tests;

pub mod gridded_data;

pub mod inputs;

pub mod prelude {
    pub use crate::{
        geometry,
        inputs,
        telescope,
    };
}