use std::time::Instant;
use astroimsim_geometry::coordinate_system::Coordinates;
use astroimsim_geometry::grid2d::{Corners, Location, GRID2D};
use astroimsim_geometry::points::Point;
use astroimsim_spectra::spectral_response::SpectralResponseCurve;
use rand_distr;
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


    pub fn load_data(&mut self,trim:usize){
        println!("Loading {:?} into {:?}",self.fits_path, self.label);
        let fits = Fits::open(self.fits_path).expect("Failed to open FITS file");
        let raw_data_x = self.grid.x_num+2*trim;
        let raw_data_y = self.grid.y_num+2*trim;
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


    pub fn spawn_downsample(&self, grid2d: GRID2D)-> SpatialEffect{

        let down_sampled_data:Vec<f64> =
            (0..grid2d.num_points).map(|index|
            {//println!("value at index {:?} is  {:?}, location {:?}", index,self.get_data(&grid2d.locate(index)),grid2d.locate(index) );
                self.get_data(&grid2d.locate(index))}
            ).collect();
        let data:Vec<Vec<f64>> = down_sampled_data.chunks(grid2d.x_num)
            .map(|v|v.to_vec()).collect();
        println!("BEFORE {:?}", self.data[0][0]);
        println!("After{:?}", data[0][0]);
        SpatialEffect{
            label: "downsampled data ", //TODO
            grid: grid2d,
            data,
            fits_path: "N/A", //TODO: path enheritance
        }
    }


    pub fn get_data_at_grid_index(&self, grid_number:usize)->f64{
        let (x,y) = self.grid.xy_indices(grid_number);
        self.data[y][x]
    }


   pub fn get_data(&self,point:&Point)->f64{
       let interpolation_data = self.grid.projected_interpolation_coefficients(point);
       //println!("{:?}",interpolation_data.corners.len());

       let corners = match interpolation_data.corners{
           Corners::Four(one, two, three, four) => {
               vec![one,two,three,four]}
           Corners::Two(one,two) => {vec![one,two]}
           Corners::One(one) => {vec![one]}
       };
       let sum:f64 = corners
           .into_iter().
           zip(interpolation_data.coefficients)
           .map(|(point,coefficient)|{
               self.get_data_at_grid_index(point)*coefficient
           }).sum();
       sum/interpolation_data.normalization
   }
    
}





