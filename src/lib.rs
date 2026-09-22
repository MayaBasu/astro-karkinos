pub mod telescope_components;
pub mod geometry;
pub mod tests;

pub mod inputs;

pub mod prelude {
    pub use crate::{
        geometry,
        inputs,
        telescope_components,
    };
}