use std::time::Instant;
use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::coordinate_system::Coordinates::{ABSOLUTE, RELATIVE};
use astroimsim_geometry::grid1d::GRID1D;
use astroimsim_geometry::grid2d::{Location, GRID2D};
use astroimsim_geometry::points::Point;
use astroimsim_spectra::power_spectrum::{PowerSpectrum, SpectrumUnits};
use astroimsim_spectra::spectral_response::SpectralResponseCurve;
use rand::distr::Distribution;
use rand_distr::Poisson;
use crate::detector::{Detector, DetectorArray};
use crate::point_sources::FullSpectrumSourceList;
use crate::psf::SpatialEffect;
use crate::psf_grid::PsfGrid;
use rayon::prelude::*;

pub struct UVEX{
    pub fuv_psf_path: &'static str,
    pub fuv_psf_grid: GRID2D,
    pub fuv_psf: PsfGrid,
    pub x_gap: f64,
    pub y_gap:f64,
    pub detector_array: DetectorArray,
    pub detector_grid: GRID2D,
    pub flatfield_illumination: SpatialEffect,
    pub flatfield_4k_illumination:SpatialEffect,
    pub fuv_response: SpectralResponseCurve,
    pub nuv_response: SpectralResponseCurve,
    pub num_pixels: usize,

}

impl UVEX{
    pub fn initialize(
        fuv_psf_path: &'static str,
        flatfield_path: &'static str,
    )->UVEX {
        let num_pixels = 4096;
        let x_detectors = 3;
        let y_detectors = 3;
        let x_gap = 0.02;
        let y_gap = 0.02;

        //load in flatfield illumination
        let flatfield_grid = UVEX::flatfield_illumination_grid();
        let mut flatfield = SpatialEffect::new_empty("Flatfield Illumination",flatfield_grid,flatfield_path);
        flatfield.load_data();

        let flat4_path = "/Users/mayabasu/Desktop/uvex_psf_files/NUV_vignetting_model_4096.fits";
        let flatfield_grid_4k = UVEX::flatfield_4k_illumination_grid();
        let mut flatfield_4k = SpatialEffect::new_empty("Flatfield 4k Illumination",flatfield_grid_4k,flat4_path);
        flatfield_4k.load_data();



        //load in fuv psf files
        let center_keys = ("XFLD", "YFLD");
        let fuv_psf_grid = UVEX::empty_fuv();
        let mut fuv_psf = PsfGrid::new("FUV PSF",fuv_psf_grid.clone(),fuv_psf_path,center_keys);
        fuv_psf.load_data_frames(64,64);

        //Load spectral response data
        let (fuv,nuv) = UVEX::fuv_nuv();

        //make detector grid
        let (detectors,detector_grid) = UVEX::uvex_detector_array(
            4096,
            3 ,
            3,
            0.02,
            0.02,
            1.0,
            Point::new(-0.56, -0.06,ABSOLUTE));


        UVEX{
            fuv_psf_path,
            fuv_psf_grid,
            fuv_psf,
            x_gap: 0.02,
            y_gap: 0.02,
            detector_array: detectors,
            detector_grid: detector_grid,
            flatfield_illumination: flatfield,
            flatfield_4k_illumination:flatfield_4k,
            fuv_response: fuv,
            nuv_response: nuv,
            num_pixels,
        }




    }

    pub fn area()->f64{
        4410.0
    }

    pub fn spectral_grid()->GRID1D{
        GRID1D::new_empty(1.0,120.0,1000.0,0.01,1.0)
    }


    pub fn flatfield_illumination_grid() ->GRID2D{

        let num_pixels =512;
        let flatfield_width_degrees = 4.52*512.0/511.0;
        let center_absolute = Point::new(-0.5,0.0,Coordinates::ABSOLUTE);

        let pixel_to_deg_scale = flatfield_width_degrees/num_pixels as f64; //Degrees in FOV to pixels
        let flatfield_x_axis = (pixel_to_deg_scale,0.0);
        let flatfield_y_axis = (0.0,pixel_to_deg_scale);

        let coordinate_system = CoordinateSystem::new(
            flatfield_x_axis,
            flatfield_y_axis,
            center_absolute.values(),
            "Detector Coordinate System".to_string(),
            "magenta".to_string());

        GRID2D::new_empty(
            (num_pixels,num_pixels),(1.0,1.0), (0.0,0.0),0.01,Coordinates::RELATIVE(coordinate_system))

    }

    pub fn flatfield_4k_illumination_grid() -> GRID2D{

        let num_pixels =4096;
        let flatfield_width_degrees = (4.52)*4096.0/4095.0;
        let center_absolute = Point::new(-0.5,0.0,Coordinates::ABSOLUTE);

        let pixel_to_deg_scale = flatfield_width_degrees/num_pixels as f64; //Degrees in FOV to pixels
        let flatfield_x_axis = (pixel_to_deg_scale,0.0);
        let flatfield_y_axis = (0.0,pixel_to_deg_scale);

        let coordinate_system = CoordinateSystem::new(
            flatfield_x_axis,
            flatfield_y_axis,
            center_absolute.values(),
            "Detector Coordinate System".to_string(),
            "magenta".to_string());

        GRID2D::new_empty(
            (num_pixels,num_pixels),(1.0,1.0), (0.0,0.0),0.01,Coordinates::RELATIVE(coordinate_system))


    }


    pub fn compare_flatfields(self,num_points:usize)-> Vec<f64>{



        println!("{:?}",self.flatfield_4k_illumination.grid.x_size);
        println!("{:?}",self.flatfield_illumination.grid.x_size);

        for i in 0..512{
            /*
            println!("\n \n \n {:?} \n   {:?}", self.flatfield_4k_illumination.grid.locate(i),
                     self.flatfield_illumination.grid.locate(i)
            );

             */


        }
        let errors = (0..num_points).map(|_i|{
            let random_point = self.flatfield_4k_illumination.grid.random();
            let fine_grain_result = self.flatfield_4k_illumination.get_data(&random_point);
            let corse_grain_result = self.flatfield_4k_illumination.get_data(&random_point);
            let error = (fine_grain_result - corse_grain_result)/ fine_grain_result;

            println!("{:?}    {:?}   {:?}",
                     self.flatfield_4k_illumination.grid.fit_grid(&random_point),
                     self.flatfield_illumination.grid.fit_grid(&random_point),
                     error);


            error*100.0
        }).collect();
        errors
    }

    pub fn empty_fuv() -> GRID2D {

        let coord = CoordinateSystem{
            x_axis: (1.0,0.0),
            y_axis: (0.0,1.0),
            center: (0.0, 0.0),
            color: "red".to_string(),
            label: "fuv".to_string(),
        };
        let mut grid = GRID2D::new_empty(
            (18,18), //x_num
            (0.2,0.2), //x_step_size
            (-0.56, -0.06), //y_num
            0.1, //y_step_size
            Coordinates::RELATIVE(coord)
        );
        grid.label = "fuv".to_string();
        grid
    }

    pub fn uvex_detector_array(
        num_pixels: usize,
        x_num:usize,
        y_num:usize,
        x_gap_deg:f64,
        y_gap_deg:f64,
        detector_width_deg:f64,
        detector_grid_center:Point) -> (DetectorArray,GRID2D){

        let pixel_to_deg_scale = detector_width_deg/num_pixels as f64; //Degrees in FOV to pixels
        let detectors_x_axis = (pixel_to_deg_scale,0.0);
        let detectors_y_axis = (0.0,pixel_to_deg_scale);

        let coordinate_system = CoordinateSystem{
            x_axis: detectors_x_axis,
            y_axis: detectors_y_axis,
            center: (0.0, 0.0),
            color: "red".to_string(),
            label: "detector grid".to_string(),
        };
        let detector_grid = GRID2D::new_empty(
            (x_num,y_num),
            (1.0 + x_gap_deg,1.0 + y_gap_deg),
            detector_grid_center.to_absolute().values(),//(-0.56, -0.06),
            0.001,
            ABSOLUTE);

        let mut detectors = Vec::new();
        for point in 0..detector_grid.num_points{
            let point_location = detector_grid.locate(point);
            // println!("Point location of point {point} is {:?}",point_location);
            let center = Point::new(point_location.x, point_location.y, Coordinates::ABSOLUTE);
            //println!("Center is at {:?}",center);
            detectors.push(UVEX::new_uvex_detector(
                stringify!(point),
                center,
                num_pixels,
                RELATIVE(coordinate_system.clone())))

        }

        (DetectorArray{
            label: "UVEX Detector Array".to_string(),
            detectors,
            coordinate_system,
        }, detector_grid)
    }

    pub fn new_uvex_detector(label: &'static str, center:Point,num_pixels:usize,coordinates: Coordinates) -> Detector {

        let grid = GRID2D::new_empty((num_pixels, num_pixels), (1.0, 1.0), center.convert(&coordinates).values(), 0.001, coordinates);
        let mut data = Vec::with_capacity(num_pixels*num_pixels);
        for _row in 0..num_pixels{
            let mut row_vec = Vec::with_capacity(num_pixels);
            for _column in 0..num_pixels{
                row_vec.push([0.0;4])
            }
            data.push(row_vec);
        }
        Detector {label, grid, data}
    }



    pub fn fuv_nuv()->(SpectralResponseCurve,SpectralResponseCurve){

        let directory = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files";
        const FUV_CONTAMINATION_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_FUV_contamination.dat";
        const NUV_CONTAMINATION_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_contamination.dat";
        const FUV_RESPONSE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_FUV_filter_response.dat";
        const NUV_RESPONSE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_filter_response.dat";
        const NUV_QE_CURVE_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_NUV_QE.dat";
        const DICHROIC_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/UVIM_dichroic_response.dat";
        const MIRROR_PATH: &'static str = "/Users/mayabasu/Desktop/uvex_psf_files/spectral_reponse_files/mirror_reflectivity.dat";

        pub const FUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
        pub const NUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
        pub const FUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);
        pub const NUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1050.0,0.01,1.0);
        pub const NUV_QE_CURVE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1000.0);
        pub const DICHROIC_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1000.0,0.01,1000.0);
        pub const MIRROR_GRID:GRID1D = GRID1D::new_empty(1.0,110.0,1100.0,0.01,1.0);

        //sd   let FUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("FUV_CONTAMINATION",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH);
        // let NUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("NUV_CONTAMINATION",NUV_CONTAMINATION_GRID,NUV_CONTAMINATION_PATH);
        let mut FUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("FUV_DICHROIC",DICHROIC_GRID,DICHROIC_PATH,1,"      ");
        let mut NUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("NUV_DICHROIC",DICHROIC_GRID,DICHROIC_PATH,2,"      ");
        let mut  NUV_QE: SpectralResponseCurve = SpectralResponseCurve::new("NUV_QE",NUV_QE_CURVE_GRID,NUV_QE_CURVE_PATH,1,"   ");
        let mut FUV_CONTAMINATION:SpectralResponseCurve = SpectralResponseCurve::new("FUV Contamination",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH,1,"   ");
        //FUV_CONTAMINATION.write_to_dat("contamination","FUV contamination");
        let mut NUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("NUV Filter response",NUV_RESPONSE_GRID,NUV_RESPONSE_PATH,1,"   ");
        let mut FUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("FUV Filter response",FUV_RESPONSE_GRID,FUV_RESPONSE_PATH,1,"   ");

        let mut MIRROR_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("Mirror", MIRROR_GRID, MIRROR_PATH,1,"    ");
        let mut MIRROR_CURVE_3 = MIRROR_CURVE.clone();
        MIRROR_CURVE_3.self_compose(3);


        let fuv = SpectralResponseCurve::compose(vec![
            MIRROR_CURVE_3.clone(),
            FUV_CONTAMINATION.clone(),
            FUV_DICHROIC,
            FUV_FILTER_CURVE,

        ]);
        let nuv = SpectralResponseCurve::compose(vec![
            MIRROR_CURVE_3,
            FUV_CONTAMINATION,
            NUV_DICHROIC,
            NUV_FILTER_CURVE,
            NUV_QE
        ]);
        (fuv,nuv)
    }


    pub fn run(self, source_list: FullSpectrumSourceList){

        let start = Instant::now();
      //  let detector_grid =self.detector_array.detectors[0].grid.clone();

        /*
        //add background
        for row in 0..4000{
            let mut row_vec = Vec::new();
            for column in 0..4000{
                //if row < matrix_x && column < matrix_y{
                   // row_vec.push(matrix[column][row]);

                //}else{
                //    row_vec.push(0.0);
               // }
                //row_vec.push((((row + column) as f32)/100.0))
                row_vec.push(0.0)
            }
            data.push(row_vec);
        }

         */
        // println!("{}, {}",data.len(),data[0].len());


        self.detector_array.detectors.into_par_iter().enumerate().for_each(|(i,mut detector)|
            { let mut dropped = 0;
                let detector_grid = detector.grid.clone();
                for (num, point) in source_list.sources.iter().enumerate(){

                    if num % 10000 ==0{
                        println!("Done {:?} sources for detector number {:?}", num,i)
                    }
                    let bands = point.to_bands(&self.fuv_response,&self.nuv_response, UVEX::area());

                     let mut rng = rand::rng();

                    match detector_grid.inside_or_outside(&point.point){ //TODO remove many unneeded clone() calls by borrowing Points
                        Location::Outside => {dropped +=1}
                        Location::Inside => {let psf = self.fuv_psf.interpolated_psf(&point.point);
                            let ((x_mod,y_mod),binned_psf) = detector_grid.bin_up_patch(point.point.clone(),&psf,10); //TODO scale is fixed
                            //println!("{:?}",(x_mod,y_mod));
                            let binned_matrix_x = binned_psf[0].len();
                            let binned_matrix_y = binned_psf.len();

                            for row in 0..binned_matrix_y{
                                for column in 0..binned_matrix_x{


                                    //println!("{}{}",column + y, row + y);
                                    if column + y_mod < detector_grid.x_num && row + x_mod < detector_grid.y_num{

                                        let fuv_flux =binned_psf[column][row] as f64*bands.fuv;
                                        let nuv_flux =binned_psf[column][row] as f64*bands.nuv;
                                        if (fuv_flux == 0.0) &&(nuv_flux ==0.0){
                                            continue
                                        }else{
                                            // println!("flux is {:?}", flux);
                                           // println!("{:?}",fuv_flux);
                                             let fuv_poisson = Poisson::new(fuv_flux as f64).unwrap();
                                            let nuv_poisson = Poisson::new(nuv_flux as f64).unwrap();
                                            let fuv = fuv_poisson.sample(&mut rng) as f64;
                                            let nuv = nuv_poisson.sample(&mut rng) as f64;
                                            detector.data[column + y_mod][row + x_mod][0] += fuv_flux;
                                            detector.data[column + y_mod][row + x_mod][1] += nuv_flux;
                                            detector.data[column + y_mod][row + x_mod][2] += fuv as f64 ;
                                            detector.data[column + y_mod][row + x_mod][3] += nuv as f64 ;
                                            //bin.sample(&mut rng) as f32;
                                        }



                                    }else{
                                        // println!("dropping pixel");
                                    }

                                    // println!("modifying pixel {:?} to be {:?}",(row + x_mod,column + y_mod),binned_psf[column][row]);
                                }
                            }}
                    }






                }
                // data[0][0]  += 100.0;

                let size = detector.data.len();
                let size2 = detector.data[0].len();

                let sum:f64  = detector.data.iter().flatten().flatten().sum();
                println!("Done computation: {:?} for detecgtor {:?}",sum,i);
                detector.write(i);
                let duration = start.elapsed();
                println!("Time elapsed in expensive_function() is: {:?}, dropped {:?}", duration,dropped);

                println!("made array, sum is  :{}, size is {:?}, {:?}",sum, size,size2);


            });










    }


}