use crate::geometry::{FloatError, Regular, GRID1D};
use crate::geometry::FloatError::{ValueIsZero, ValueNotRegular};
use crate::gridded_data::{SpectralUnits, DATA1D, SpectralResponseUnit};



pub struct SpectralResponse{
    data: Vec<f64>,
    grid1d: GRID1D,
}

/*
Spectral Response assumptions:
1. One data point per grid point
2. Data is normalized
3. Data values are Regular
 */

impl SpectralResponse{
    pub fn normalize(data:Vec<f64>) -> Result<Vec<f64>,FloatError>{
        if !DATA1D::is_data_regular(&data){
            return Err(ValueNotRegular)
        }
        let sum = data.iter().sum();
        if sum == 0.0{
            return Err(ValueIsZero)
        }
        Ok(data.iter().map(|v|*v/sum).collect())
    }
    pub fn new(grid1d: GRID1D, data: Vec<f64>) -> Self {
        assert_eq!(grid1d.num_points(), data.len(),
                   "Failed to initialize new SpectralResponse struct:\
                   Must have exactly one data point per grid point");
        let data = match SpectralResponse::normalize(data){
            Ok(data) => {data}
            Err(float_error) => {
                match float_error{
                    ValueNotRegular => {panic!("Failed to initialize new SpectralResponse struct:\
                    Data must not contain inf or Nan values")}
                    ValueIsZero => {panic!("Failed to initialize new SpectralResponse struct:\
                    Data must not sum to 0")}
                }
            }
        };

        SpectralResponse{
            data,
            grid1d,
        }
    }
    pub fn regrid(&mut self, grid1d:GRID1D)->Self{
        let new_data = self.values(&grid1d);
        SpectralResponse::new(grid1d,new_data)
    }
}
impl DATA1D<SpectralResponseUnit> for SpectralResponse{
    fn grid(&self) -> GRID1D {
        self.grid1d
    }
    fn data(&self) -> Vec<f64> {
        self.data
    }
    fn unit(&self) -> SpectralResponseUnit {
        SpectralResponseUnit::NormalizedFraction
    }
}
