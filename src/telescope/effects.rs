use std::fs::File;
use uvex_fitrs::{Fits, FitsData, FitsDataArray};

use crate::geometry::{FloatError, Regular, GRID1D, GRID2D};
use crate::geometry::FloatError::{ValueIsZero, ValueNotRegular};
use crate::gridded_data::{SpectralUnits, DATA1D, UnitlessUnit, DATA2D, Unit};

/*

pub fn load_data(&mut self, trim:usize){
    println!("Loading {:?} into {:?}",self.fits_path, self.label);
    let fits = Fits::open(self.fits_path.clone()).expect("Failed to open FITS file");
    let raw_data_x = self.grid.x_num()+2*trim;
    let raw_data_y = self.grid.y_num()+2*trim;
    let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");
    let (mut data,shape) = match primary_hdu.read_data() {
        FitsData::FloatingPoint64(FitsDataArray { shape, data }) => (data,shape),
        FitsData::FloatingPoint32(FitsDataArray { shape, data }) => {
            let data = data.iter().map(|x|*x as f64).collect();
            (data,shape)},
        _ => {panic!("huh? Couldn't load FITS file")}
    };
    assert_eq!(shape[0],raw_data_x,"FITS data had the wrong width");
    assert_eq!(shape[1],raw_data_y,"FITS data had the wrong height");
    let mut data:Vec<Vec<f64>> = data.chunks(raw_data_x).map(|v|{
        v[trim..raw_data_x-trim].to_vec()
    }).collect();
    for delete_row in 0..trim{
        data.remove(0);
        data.pop();
    };
    self.data = data;

}

fn write_to_dat(&mut self, header:&str, path:&str){
    let mut file = File::create(path).expect("Could not create file");
    file.write_all(header.as_bytes()).expect("Failed to write header");
    file.write_all(format!("| \n{:?} | {:?} |",self.grid().unit(), self.unit()).as_bytes())
        .expect("Failed to write units to file");
    for (point,value) in &(0..self.grid().num_points()).zip(self.data()){
        let location = self.grid().locate_grid_point(*point);
        file.write_all(format!("\n{:?} | {:?}", location, value).as_bytes())
            .expect("Failed to write data");
    }
}

 */






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
impl DATA1D<UnitlessUnit> for SpectralResponse{
    fn grid(&self) -> GRID1D {
        self.grid1d
    }
    fn data(&self) -> Vec<f64> {
        self.data
    }
    fn unit(&self) -> UnitlessUnit {
        UnitlessUnit::NormalizedFraction
    }
}


pub struct SpatialEffect{
    grid: GRID2D,
    data:Vec<Vec<f64>>,
}

impl DATA2D<UnitlessUnit> for SpatialEffect{
    fn grid(&self) -> GRID2D {
        self.grid
    }
    fn data(&self) -> Vec<Vec<f64>> {
        self.data
    }
    fn unit(&self) -> UnitlessUnit {
        UnitlessUnit::NormalizedFraction
    }
}
