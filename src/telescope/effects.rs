use uvex_fitrs::{Fits, FitsData, FitsDataArray};
use crate::geometry::*;
use crate::gridded_data::{DATA1D, UnitlessUnit, DATA2D};


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
        if !Self::is_data_regular(&data){
            return Err(FloatError::ValueNotRegular)
        }
        let sum:f64 = data.iter().sum();
        if sum == 0.0{
            return Err(FloatError::ValueIsZero)
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
                    FloatError::ValueNotRegular => {panic!("Failed to initialize new SpectralResponse struct:\
                    Data must not contain inf or Nan values")}
                    FloatError::ValueIsZero => {panic!("Failed to initialize new SpectralResponse struct:\
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
impl DATA1D<UnitlessUnit> for SpectralResponse{
    fn grid(&self) -> &GRID1D {
        &self.grid1d
    }
    fn data(&self) -> &Vec<f64> {
        &self.data
    }
    fn unit(&self) -> &UnitlessUnit {
        &UnitlessUnit::NormalizedFraction
    }
}


pub struct SpatialEffect{
    grid: GRID2D,
    data:Vec<Vec<f64>>,
}

impl DATA2D<UnitlessUnit> for SpatialEffect{
    fn grid(&self) -> &GRID2D {
        &self.grid
    }
    fn data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }
    fn unit(&self) -> &UnitlessUnit {
        &UnitlessUnit::NormalizedFraction
    }
} 

impl SpatialEffect{

    pub fn normalize(data:Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>,FloatError>{
        if !Self::is_data_regular(&data){
            return Err(FloatError::ValueNotRegular)
        }
        let sum:f64 = data.clone().into_iter().flatten().sum();
        if sum == 0.0{
            return Err(FloatError::ValueIsZero)
        }
        Ok(data.into_iter().map(|v|v.into_iter().map(|e|e/sum).collect()).collect())
    }

    pub fn new(grid:GRID2D, data: Vec<Vec<f64>>) -> Result<SpatialEffect,FloatError>{
        let data = Self::normalize(data)?;
        Ok(SpatialEffect{
            grid,
            data,
        })
    }

    pub fn from_fits(grid:GRID2D,path:&str)-> Result<SpatialEffect,FloatError>{
        let fits = Fits::open(path).expect(&format!("Failed to open {path} as FITS file"));
        let primary_hdu= fits.iter().next().expect(&format!("Couldn't find primary HDU in {path}"));

        let (data,shape) = match primary_hdu.read_data() {
            FitsData::FloatingPoint64(FitsDataArray { shape, data }) => (data,shape),
            FitsData::FloatingPoint32(FitsDataArray { shape, data }) => {
                let data = data.into_iter().map(|x|x as f64).collect();
                (data,shape)},
            _ => {panic!("Failed to read data from FITS file: {path}")}
        };
        assert_eq!(shape[0],grid.x_num(),
                   "{}", format!("Expected {:?} points in x dimension, loaded {:?}",grid.x_num(),shape[0]));
        assert_eq!(shape[1],grid.y_num(),
                   "{}", format!("Expected {:?} points in y dimension, loaded {:?}",grid.y_num(),shape[1]));
        let data = data.chunks(grid.x_num()).map(|v|v.to_vec()).collect();
        Self::new(grid,data)

        }


}
