use rand::distr::Distribution;
use rand_distr::Poisson;
use crate::geometry::*;
use crate::gridded_data::{DetectorUnits, SpectralUnits, DATA2D};
use crate::telescope::effects::SpatialEffect;
use rand::{random, RngExt};
use rand_distr::{Normal};
#[derive(Clone,Debug)]
pub struct Detector {
    pub grid: GRID2D,
    pub data: Vec<Vec<f64>>,
    pub unit: DetectorUnits
}

impl DATA2D<DetectorUnits> for Detector{
    fn grid(&self) -> &GRID2D {
        &self.grid
    }
    fn data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }

    fn mutable_data(&mut self) -> &mut Vec<Vec<f64>> {
        &mut self.data
    }

    fn unit(&self) -> &DetectorUnits {
        &self.unit
    }
}



impl Detector {
    //TODO: combine initialize with first pass?
    pub fn initialize(grid: GRID2D, unit: DetectorUnits) -> Detector{
        let data:Vec<Vec<f64>> = (0..grid.y_num()).map(|_| {
            (0..grid.x_num()).map(|_|{
                0.0
            }).collect()
        }).collect();
        Detector{
            grid,
            data,
            unit,
        }
    }

    pub fn add(&mut self,effect:&SpatialEffect){
        for grid_point in 0..effect.grid().num_points(){
            let location = effect.grid().locate(grid_point).transform_to(&self.grid.coordinate_system());
                  if self.grid.is_point_inside(&location){

                let value = effect.value(&location);
                let (x_mod,y_mod,x_res,y_res) = self.grid.fit_grid(&location,true).unwrap();
                self.data[y_mod][x_mod] += value;
            }

        }
    }

    pub fn fast_add_multi(&mut self,center:&Point, psf_datas:&Vec<Vec<Vec<f64>>>,bands:&Vec<f64>,scale:f64){



        let binned_psfs:Vec<((usize,usize),Vec<Vec<f64>>)> = psf_datas.into_iter().map(|data|{
            //TODO scale is fixed, also center should depend on pixel to arc sec scale
            let x_num  = data[0].len();
            let y_num =  data.len();
            let psf_half = &Point::new(1./(60.*60.)*x_num as f64/(10 as f64 *2.),1./(60.*60.)*y_num as f64/(10 as f64 * 2.),&ABSOLUTE_COORDINATES);

            self.grid.bin_up_patch(&(center-psf_half), &data, 10)

        }).collect();

        let binned_matrix_x = binned_psfs[0].1[0].len();
        let binned_matrix_y = binned_psfs[0].1.len();

        let y_mod = binned_psfs[0].0.1;
        let x_mod = binned_psfs[0].0.0;

        for row in 0..binned_matrix_y {
            for column in 0..binned_matrix_x {

                if column + y_mod < self.grid.x_num() && row + x_mod < self.grid.y_num() {
                    let mut flux = 0.0;
                    for psf in 0..binned_psfs.len(){//TODO: binned_psfs.len()
                        flux += (binned_psfs[psf].1[column][row] as f64)*bands[psf]*scale
                    }
                    if (flux == 0.0) {
                        continue
                    } else {

                        self.data[column + y_mod][row + x_mod] += flux;

                    }
                } else {
                    // println!("dropping pixel");
                }

                // println!("modifying pixel {:?} to be {:?}",(row + x_mod,column + y_mod),binned_psf[column][row]);
            }
        }
    }

    pub fn fast_add(&mut self, effect:&SpatialEffect){

        let ((x_mod, y_mod), binned_psf) = self.grid.bin_up_patch(&effect.grid.center(), &effect.data, 10); //TODO scale is fixed
        let binned_matrix_x = binned_psf[0].len();
        let binned_matrix_y = binned_psf.len();

        for row in 0..binned_matrix_y {
            for column in 0..binned_matrix_x {


                //println!("{}{}",column + y, row + y);
                if column + y_mod < self.grid.x_num() && row + x_mod < self.grid.y_num() {
                    let fuv_flux = binned_psf[column][row] as f64; //* bands.fuv * point.scale;
                   // let nuv_flux = binned_psf[column][row] as f64; //* bands.nuv * point.scale;
                    if (fuv_flux == 0.0) {
                        continue
                    } else {
                        // println!("flux is {:?}", flux);
                        //  println!("fuv flux {:?}",fuv_flux);
                        //    let fuv_poisson = Poisson::new(fuv_flux as f64).unwrap();
                        //   let nuv_poisson = Poisson::new(nuv_flux as f64).unwrap();
                        //   let fuv = fuv_poisson.sample(&mut rng) as f64;
                        //  let nuv = nuv_poisson.sample(&mut rng) as f64;
                        self.data[column + y_mod][row + x_mod] += fuv_flux;
                        //self.data[column + y_mod][row + x_mod][1] += nuv_flux;
                        // self.detector_array.detectors[0].data[column + y_mod][row + x_mod][2] += fuv as f64 ;
                        // self.detector_array.detectors[0].data[column + y_mod][row + x_mod][3] += nuv as f64 ;
                        //bin.sample(&mut rng) as f32;
                    }
                } else {
                    // println!("dropping pixel");
                }

                // println!("modifying pixel {:?} to be {:?}",(row + x_mod,column + y_mod),binned_psf[column][row]);
            }
        }
    }



    pub fn to_electrons(&mut self, dark_current: f64, read_noise: f64, exposure_time:f64, saturation:f64,gain:f64){
        assert_eq!(self.unit, DetectorUnits::AverageElectrons);
        let mut rng = rand::rng();
        let read_noise = Poisson::new(read_noise).unwrap();
        for y in 0..self.grid.y_num(){
            for x in 0..self.grid.x_num(){
                //println!("Data is {:?}, {:?}",self.data[y][x] ,dark_current);

                let expected_source_electrons = (self.data[y][x] + dark_current)*exposure_time;
                //total_source = total 'expected' electrons from background and point sources
                let generator = Poisson::new(expected_source_electrons).unwrap();

                let poisson_source_electrons= generator.sample(&mut rng);

               // println!("sampled to {:?}",poisson_source_electrons);
                let saturated_source_electrons = if  poisson_source_electrons > saturation{
                    saturation
                }else{
                    poisson_source_electrons
                };

                //println!("saturated to {:?}",saturated_source_electrons);
                let final_electrons = saturated_source_electrons + read_noise.sample(&mut rng);
               // println!("final value is {:?}",final_electrons);
                self.data[y][x] = (final_electrons/gain).floor();

            }
        }
        self.unit = DetectorUnits::ADU;
    }
    pub fn plain_to_electrons(&mut self, dark_current: f64, read_noise: f64, exposure_time:f64, saturation:f64,gain:f64){
        assert_eq!(self.unit, DetectorUnits::AverageElectrons);

        for y in 0..self.grid.y_num(){
            for x in 0..self.grid.x_num(){
                //println!("Data is {:?}, {:?}",self.data[y][x] ,dark_current);

                let expected_source_electrons= (self.data[y][x])*exposure_time;//dark_current

                self.data[y][x] = (expected_source_electrons/gain);

            }
        }
        self.unit = DetectorUnits::ADU;
    }

    pub fn plain2_to_electrons(&mut self, dark_current: f64, read_noise: f64, exposure_time:f64, saturation:f64,gain:f64){
        assert_eq!(self.unit, DetectorUnits::AverageElectrons);
        let mut rng = rand::rng();
        let mut read_noise = Normal::new(0., read_noise).unwrap(); //Poisson::new(read_noise).unwrap();
        for y in 0..self.grid.y_num(){
            for x in 0..self.grid.x_num(){
                //println!("Data is {:?}, {:?}",self.data[y][x] ,dark_current);

                let expected_source_electrons= (self.data[y][x]+dark_current )*exposure_time;//dark_current
                //total_source = total 'expected' electrons from point sources + bakground
               // let generator = Poisson::new(expected_source_electrons).unwrap();

                let poisson_source_electrons= expected_source_electrons;//generator.sample(&mut rng);


                // println!("sampled to {:?}",poisson_source_electrons);
                let saturated_source_electrons = if  poisson_source_electrons > saturation{
                    saturation
                }else{
                    poisson_source_electrons
                };

                //println!("saturated to {:?}",saturated_source_electrons);
                let final_electrons = saturated_source_electrons + read_noise.sample(&mut rng);
                // println!("final value is {:?}",final_electrons);
                self.data[y][x] = (final_electrons/gain).floor();

            }
        }
        self.unit = DetectorUnits::ADU;
    }

    pub fn new_to_electrons(&mut self, dark_current: f64, read_noise: f64, exposure_time:f64, saturation:f64,gain:f64){
        assert_eq!(self.unit, DetectorUnits::AverageElectrons);
        let mut rng = rand::rng();
        let mut read_noise = Normal::new(0., read_noise).unwrap(); //Poisson::new(read_noise).unwrap();
        for y in 0..self.grid.y_num(){
            for x in 0..self.grid.x_num(){
                //println!("Data is {:?}, {:?}",self.data[y][x] ,dark_current);

                let expected_source_electrons= (self.data[y][x]+dark_current )*exposure_time;//dark_current
                //total_source = total 'expected' electrons from point sources + bakground
                let generator = Poisson::new(expected_source_electrons).unwrap();

                let poisson_source_electrons= generator.sample(&mut rng);


                // println!("sampled to {:?}",poisson_source_electrons);
                let saturated_source_electrons = if  poisson_source_electrons > saturation{
                    saturation
                }else{
                    poisson_source_electrons
                };

                //println!("saturated to {:?}",saturated_source_electrons);
                let final_electrons = saturated_source_electrons + read_noise.sample(&mut rng);
                // println!("final value is {:?}",final_electrons);
                self.data[y][x] = (final_electrons/gain).floor();

            }
        }
        self.unit = DetectorUnits::ADU;
    }

    pub fn multiply_effect2(&mut self,effect:&SpatialEffect){
        for grid_point in 0..effect.grid().num_points(){
            let location = effect.grid().locate(grid_point).transform_to(&self.grid.coordinate_system());
            if self.grid.is_point_inside(&location){

                let value = effect.value(&location);
                let (x_mod,y_mod,x_res,y_res) = self.grid.fit_grid(&location,true).unwrap();
                if self.data[y_mod][x_mod] > 0.{
                    println!("Applied vinietting as: {:?}", value);
                    println!("before: {:?}", self.data[y_mod][x_mod]);
                }
                self.data[y_mod][x_mod] *= value;
                if self.data[y_mod][x_mod] > 10. {
                    println!("after: {:?}", self.data[y_mod][x_mod]);
                }


            }

        }
    }
    
    


    pub fn add_background(&mut self,effect:&SpatialEffect){

        for y in 0..self.grid.y_num(){
            for x in 0..self.grid.x_num(){
                let grid_point = self.grid.grid_number(x,y);
                let location = self.grid.locate(grid_point);
                let value = effect.value(&location);
                //println!("{:?} at {:?}, {:?}  {:?}", value, grid_point, location.x.value(),location.y.value());
                self.data[y][x] += value;
                //std::thread::sleep(core::time::Duration::from_millis(100));
            }
        }
    }


    pub(crate) fn combine_2_add(&mut self, effect:&SpatialEffect) {
        let mut points_inside = 0;
        let mut nonzero = 0;
        for y in 0..self.grid().y_num(){
            for x in 0..self.grid().x_num(){
                let grid_number = self.grid().grid_number(x,y);
                let point = self.grid().locate(grid_number);
                let value = if effect.grid().is_point_inside_cells(&point){

                    points_inside += 1;
                    effect.value(&point)

                }else{
                    0.0
                };

                if value != 0.0{
                    nonzero += 1;
                }

                self.data[y][x] += value;



            }
        }
        println!("{:?} points inside {:?}", points_inside,nonzero);

    }

}









