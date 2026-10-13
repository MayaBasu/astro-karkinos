use std::time::Instant;
use egui::Grid;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use crate::gridded_data::units::DetectorUnits;
use crate::geometry::{CoordinateSystem, Point, ABSOLUTE_COORDINATES, GRID2D};
use crate::gridded_data::{DATA1D, DATA2D};
use crate::gridded_data::DetectorUnits::Electrons;
use crate::telescope::{Detector, PointSource, PsfGrid, SpatialEffect, SpectralResponse, Spectrum};
use crate::uvex::config::{Config, Toggle};


pub enum Channel{
    FUV,
    NUV,
}

pub struct UVEX{

    pub mirror_response: SpectralResponse,

    pub dichroic_transmission: SpectralResponse,
    pub dichroic_reflection: SpectralResponse,

    pub nuv_filter: SpectralResponse,
    pub fuv_filter: SpectralResponse,

    pub nuv_qe: SpectralResponse,

    pub nuv_contamination: SpectralResponse,
    pub fuv_contamination: SpectralResponse,


    pub fuv: SpectralResponse,
    pub nuv: SpectralResponse,

    pub nuv_bands: Vec<f64>,
    pub nuv_psfs: Vec<PsfGrid>,

    pub fuv_psf: PsfGrid,
    pub sky_coord: CoordinateSystem,

    pub psf_coords: CoordinateSystem,

    pub detectors: Vec<Detector>,
    pub detector_layout: GRID2D,




    config: Config,
    pub fuv_read_noise: Vec<SpatialEffect>,
    pub nuv_read_noise: Vec<SpatialEffect>,

    pub fuv_dark_current: Vec<SpatialEffect>,
    pub nuv_dark_current: Vec<SpatialEffect>,

    pub fuv_dead_pixels: Vec<SpatialEffect>,
    pub nuv_dead_pixels: Vec<SpatialEffect>,
}

impl UVEX{
    pub fn load(config:Config) -> Self{
        let snap_precision = 0.00001;

        let mirror_response = SpectralResponse::load(
            &config.mirror_response.1,
            snap_precision).unwrap();

        let dichroic_reflection = SpectralResponse::load(
            &config.dichroic_reflection.1,
            snap_precision).unwrap();

        let dichroic_transmission = SpectralResponse::load(
            &config.dichroic_transmission.1,
            snap_precision).unwrap();

        let fuv_filter = SpectralResponse::load(
            &config.fuv_filter.1,
            snap_precision
        ).unwrap();

        let nuv_filter = SpectralResponse::load(
            &config.nuv_filter.1,
            snap_precision
        ).unwrap();

        let nuv_qe = SpectralResponse::load(
            &config.nuv_qe.1,
            snap_precision
        ).unwrap();

        let fuv_contamination = SpectralResponse::load(
            &config.fuv_contamination.1,
            snap_precision).unwrap();

        let nuv_contamination = SpectralResponse::load(
            &config.nuv_contamination.1,
            snap_precision
        ).unwrap();

        let fuv = mirror_response.clone()
            .multiply(&mirror_response)
            .multiply(&mirror_response)
            .multiply(&dichroic_reflection)
            .multiply(&fuv_filter)
            .multiply(&fuv_contamination);

        let nuv = mirror_response.clone()
            .multiply(&mirror_response)
            .multiply(&mirror_response)
            .multiply(&dichroic_transmission)
            .multiply(&nuv_filter)
            .multiply(&nuv_qe)
            .multiply(&nuv_contamination);


        let psf_grid = UVEX::psf_grid();


        let mut fuv_psf = PsfGrid::new(
            "FUV PSF grid".to_string(),
            psf_grid,
            config.fuv_psf_directory.clone(),
            ("XFLD".to_string(), "YFLD".to_string()));
        fuv_psf.load_data_frames(64,64);
        let mut nuv_psfs = Vec::new();

        for path in config.nuv_psf_directories.clone(){
            let mut psfs = PsfGrid::new(
                path.clone(),
                psf_grid,
                path.clone(),
                ("XFLD".to_string(), "YFLD".to_string()));
            psfs.load_data_frames(128,128);
            nuv_psfs.push(psfs)

        }

        let PSF_coords = CoordinateSystem::new(
            [0.1,0.],
        [0.,0.1],
        [0.,0.]);









        let (detectors,detector_layout) = UVEX::detectors(config.clone());
        let (fuv_read_noise,
            nuv_read_noise,
            fuv_dark_current,
            nuv_dark_current,
            fuv_dead_pixels,
            nuv_dead_pixels) = UVEX::load_detector_effects(config.clone(),&detectors);

        UVEX{
            mirror_response,
            dichroic_transmission,
            dichroic_reflection,
            nuv_filter,
            fuv_filter,
            nuv_qe,
            nuv_contamination,
            fuv_contamination,
            fuv,
            nuv,
            nuv_bands: config.clone().nuv_bands,
            nuv_psfs,
            fuv_psf,
            sky_coord: ABSOLUTE_COORDINATES,
            detectors,
            detector_layout,
            psf_coords: PSF_coords,
            fuv_read_noise,
            nuv_read_noise,
            fuv_dark_current,
            nuv_dark_current,
            fuv_dead_pixels,
            nuv_dead_pixels,
            config
        }
    }

    pub fn get_psfs(&self, point: &Point) -> (Vec<Vec<f64>>,Vec<Vec<Vec<f64>>>){
        let fuv_psf = self.fuv_psf.interpolated_psf(point);
        let nuv_psfs:Vec<Vec<Vec<f64>>> = self.nuv_psfs.iter().map(|grid|{
            grid.interpolated_psf(point)
        }).collect();
        (fuv_psf, nuv_psfs)
    }
    pub fn apply_fuv_path(&self, input:Spectrum) -> Spectrum{
        let mut fuv = self.fuv.clone();
        input.apply_spectral_response(&mut fuv)

    }
    pub fn apply_nuv_path(&self, input:Spectrum) -> Spectrum{
        let mut nuv = self.nuv.clone();
        input.apply_spectral_response(&mut nuv)

    }

    pub fn detectors(config:Config) -> (Vec<Detector>,GRID2D) {
        let deg_per_pixel = config.arcsec_per_pixel/(60.*60.);
        let x_gap_pixels = config.x_gap_mm*1000.;
        let y_gap_pixels = config.y_gap_mm*1000.;

        println!("Degrees per pixel: {:?}", deg_per_pixel);
        //TODO: maybe shift center?
        let detector_coordinates = CoordinateSystem::new(
            [deg_per_pixel,0.],
            [0.,deg_per_pixel],
            [-0.56, -0.06]);

        let detector_layout_grid = GRID2D::new(
            [config.x_detectors,config.y_detectors],
            [config.x_pixels_per_detector as f64+ x_gap_pixels,
                config.y_pixels_per_detector as f64+ y_gap_pixels],
            Point::zero(&detector_coordinates),
            0.0000000001,
            detector_coordinates);

        (detector_layout_grid.locate_grid_points().into_iter().flatten().map(|center|{
            let detector_grid = GRID2D::new(
                [config.x_pixels_per_detector,config.y_pixels_per_detector],
                [1.,1.],
                center,
                0.00001,
                detector_coordinates);
            Detector::initialize(detector_grid,DetectorUnits::AverageElectrons)
        }).collect(),detector_layout_grid)


    }
    pub fn load_detector_effects(config:Config,detectors:&Vec<Detector>) -> (Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>){
        let fuv_read_noise:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
            let grid = detector.grid;
            let path = format!("{}/fuv_read_noise_{:?}.fits",config.fuv_read_noise.1,i);
            SpatialEffect::from_fits(grid,&path).unwrap()
        }).collect();
        println!("Loaded FUV read noise");

        let nuv_read_noise:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/nuv_read_noise_{:?}.fits",config.nuv_read_noise.1,i);
                SpatialEffect::from_fits(grid,&path).unwrap()
            }).collect();
        println!("Loaded NUV read noise");

        let fuv_dark_current:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/fuv_dark_current_{:?}.fits",config.fuv_dark_current.1,i);
                SpatialEffect::from_fits(grid,&path).unwrap()
            }).collect();
        println!("Loaded FUV dark current");

        let nuv_dark_current:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/nuv_dark_current_{:?}.fits",config.nuv_dark_current.1,i);
                SpatialEffect::from_fits(grid,&path).unwrap()
            }).collect();
        println!("Loaded NUV dark current");


        let fuv_dead_pixels:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/fuv_dead_pixels_{:?}.fits",config.fuv_dead_pixels.1,i);
                SpatialEffect::from_fits(grid,&path).unwrap()
            }).collect();
        println!("Loaded FUV dead pixels");

        let nuv_dead_pixels:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/nuv_dead_pixels_{:?}.fits",config.nuv_dead_pixels.1,i);
                SpatialEffect::from_fits(grid,&path).unwrap()
            }).collect();
        println!("Loaded NUV dead pixels");

        (fuv_read_noise,
         nuv_read_noise,
         fuv_dark_current,
         nuv_dark_current,
        fuv_dead_pixels,
        nuv_dead_pixels)

    }

    pub fn psf_grid()-> GRID2D{
        let sky_coordinates = CoordinateSystem::new(
            [1.,0.],
            [0.,1.],
            [0.,0.]
        );

        GRID2D::new(
            [18, 18],
            [0.2, 0.2],
            Point::new(-0.56, -0.06, &sky_coordinates),
            0.00001,
            sky_coordinates.clone())
    }


    pub fn run(&mut self, background:SpatialEffect,channel:Channel, point_sources: Vec<PointSource>, exposure_time: f64) {

        let start = Instant::now();
        (0..self.detectors.len()).into_par_iter().for_each(|i|{


            let mut detector = self.detectors[i].clone();
            detector.add_background(&background);

            let mut dropped = 0;
            let detector_grid = detector.grid.clone();
            for (num, point) in point_sources.clone().iter().enumerate() {
                if (num % 100 == 0)&& (num>0) {
                    println!("Done {num} sources for detector number {i}");
                }
                match detector_grid.is_point_inside(&point.location) {
                    true => {

                    }
                    false => {dropped += 1}
                }

            }
            println!("Done adding point sources in {:?}s, sum is {:?}, skipped {:?}", start.elapsed().as_secs(),self.detectors[i].data.iter().flatten().sum::<f64>(),dropped);

            match channel {
                Channel::FUV => {

                    match self.config.fuv_read_noise.0 {
                        Toggle::on => {
                            detector.add(&self.fuv_read_noise[i])
                        }
                        Toggle::off => {println!("skipped nuv read noise")}
                    }
                    match self.config.fuv_dark_current.0 {
                        Toggle::on => {
                            let mut read_noise = self.nuv_read_noise[i].clone();
                            read_noise.scale(exposure_time);
                            detector.add(&read_noise)
                        }
                        Toggle::off => {println!("skipped nuv dark current")}
                    }
                    match self.config.fuv_dead_pixels.0 {
                        Toggle::on => {
                            detector.multiply(&self.fuv_dead_pixels[i])
                        }
                        Toggle::off => {println!("skipped nuv dead pixels")}
                    }
                }
                Channel::NUV => {

                    match self.config.nuv_read_noise.0 {
                        Toggle::on => {
                            detector.add(&self.nuv_read_noise[i])
                        }
                        Toggle::off => {println!("skipped nuv read noise")}
                    }

                    match self.config.nuv_dark_current.0 {
                        Toggle::on => {
                            let mut read_noise = self.nuv_read_noise[i].clone();
                            read_noise.scale(exposure_time);
                            detector.add(&read_noise)
                        }
                        Toggle::off => {println!("skipped nuv dark current")}
                    }

                    match self.config.nuv_dead_pixels.0 {
                        Toggle::on => {
                            detector.multiply(&self.nuv_dead_pixels[i])
                        }
                        Toggle::off => {println!("skipped nuv dead pixels")}
                    }


                }
            }
            detector.write_to_fits(&format!("/Users/mayabasu/Desktop/Output/detector_{i}.fits"))

        });
}














    }



