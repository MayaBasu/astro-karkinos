use std::time::Instant;
use astroimsim_geometry::coordinate_system::Coordinates;
use astroimsim_geometry::grid2d::{Location, GRID2D};
use astroimsim_geometry::points::Point;
use astroimsim_spectra::spectral_response::SpectralResponseCurve;

use uvex_fitrs::{Fits, FitsData, FitsDataArray};
use uvex_fitrs::{ Hdu};
use crate::psf::{DataFile, Load, PSF};

#[derive(Clone,Debug)]
pub struct SpatialEffect {
    pub label: &'static str,
    pub grid: GRID2D,
    pub data: Vec<Vec<f64>>, //data must be fractional - implement percent later?
    pub fits_path: &'static str,
}

impl SpatialEffect{
    pub fn new_empty(label:&'static str, grid:GRID2D,fits_path:&'static str)-> SpatialEffect{
        SpatialEffect{label,grid,data:vec![],fits_path}
    }
    
    pub fn from_matrix(label:&'static str, grid:GRID2D,fits_path:&'static str,data:Vec<Vec<f64>>)-> SpatialEffect{
        assert_eq!(grid.y_num,data.len());
        for row in &data{ assert_eq!(grid.x_num, row.len()); }
        SpatialEffect{label,grid,fits_path,data}
    }

    pub fn load_data(&mut self){
        println!("Loading {:?} into {:?}",self.fits_path, self.label);
        let fits = Fits::open(self.fits_path).expect("Failed to open FITS file");
        let primary_hdu= fits.iter().next().expect("Couldn't find primary HDU");
        let (mut data,shape) = match primary_hdu.read_data() {
            FitsData::FloatingPoint64(FitsDataArray { shape, data }) => (data,shape),
            _ => {panic!("huh? Couldn't load FITS file")}
        };
        assert_eq!(shape[0],self.grid.x_num,"FITS data had the wrong width");
        assert_eq!(shape[1],self.grid.y_num,"FITS data had the wrong height");
        let data:Vec<Vec<f64>> = data.chunks(self.grid.x_num).map(|v|v.to_vec()).collect();
        self.data = data;

    }
    pub fn get_data_at_grid_index(&self, grid_number:usize)->f64{
        let (x,y) = self.grid.xy_indices(grid_number);
        self.data[y][x]
    }

   pub fn get_data(&self,point:&Point)->f64{
       let ((i12, i22, i21, i11),(c11,c12,c21,c22),normalization) = self.grid.interpolation_coefficients(point);
       let q11 = self.get_data_at_grid_index(i11);
       let q12 = self.get_data_at_grid_index(i12);
       let q21 = self.get_data_at_grid_index(i21);
       let q22 = self.get_data_at_grid_index(i22);
       (q11*c11 + q12*c12 + q21*c21 + q22*c22)/normalization
   }
    
}





