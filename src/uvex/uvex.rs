use std::time::Instant;
use egui::Grid;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;
use crate::gridded_data::units::DetectorUnits;
use crate::geometry::{CoordinateSystem, Point, ABSOLUTE_COORDINATES, GRID1D, GRID2D, SPECTRAL_GRID};
use crate::geometry::Grid1DUnits::nm;
use crate::gridded_data::{DATA1D, DATA2D};
use crate::gridded_data::DetectorUnits::Electrons;
use crate::telescope::{Detector, Input, PointSource, PsfGrid, SpatialEffect, SpectralResponse, Spectrum};
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

   // pub nuv_qe: SpectralResponse,

    pub nuv_contamination: SpectralResponse,
    pub fuv_contamination: SpectralResponse,

    pub fuv: SpectralResponse,
    pub nuv: SpectralResponse,

    pub nuv_bands: Vec<f64>,
    pub nuv_psfs: Vec<PsfGrid>,

    pub fuv_psf: PsfGrid,
    pub sky_coord: CoordinateSystem,

    pub detectors: Vec<Detector>,
    pub detector_layout: GRID2D,




    pub(crate) config: Config,
    pub fuv_read_noise: Option<Vec<SpatialEffect>>,
    pub nuv_read_noise: Option<Vec<SpatialEffect>>,

    pub fuv_dark_current: Option<Vec<SpatialEffect>>,
    pub nuv_dark_current: Option<Vec<SpatialEffect>>,

    pub fuv_dead_pixels: Option<Vec<SpatialEffect>>,
    pub nuv_dead_pixels: Option<Vec<SpatialEffect>>,

    pub fuv_vinietting: SpatialEffect,
    pub nuv_vinietting: SpatialEffect,
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


        /*

        let nuv_qe = SpectralResponse::load(
            &config.nuv_qe.1,
            snap_precision
        ).unwrap();

         */

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
            .multiply(&fuv_contamination)
            .regrid(&SPECTRAL_GRID);

        let nuv = mirror_response.clone()
            .multiply(&mirror_response)
            .multiply(&mirror_response)
            .multiply(&dichroic_transmission)
            .multiply(&nuv_filter)
            //.multiply(&nuv_qe)
            .multiply(&nuv_contamination)
            .regrid(&SPECTRAL_GRID);


        let psf_grid = UVEX::psf_grid();


        let mut fuv_psf = PsfGrid::new(
            "FUV PSF grid".to_string(),
            psf_grid,
            config.fuv_psf_directory.clone(),
            ("XFLD".to_string(), "YFLD".to_string()));
        fuv_psf.load_data_frames();
        let mut nuv_psfs = Vec::new();

        for path in config.nuv_psf_directories.clone(){
            let mut psfs = PsfGrid::new(
                path.clone(),
                psf_grid,
                path.clone(),
                ("XFLD".to_string(), "YFLD".to_string()));
            psfs.load_data_frames();
            nuv_psfs.push(psfs)

        }

        let mut nuv_plain_psfs = PsfGrid::new(
            "in band 100 mas".to_string(),
            psf_grid,
            "/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string(),
            ("XFLD".to_string(), "YFLD".to_string()));
        nuv_plain_psfs.load_data_frames();
        nuv_plain_psfs.gaussian_blur("/Users/mayabasu/Desktop/uvex/NUV_PSF_100mas".to_string(),1.);



        //TODO vinietting size not exactly known


        let vinietting_grid = GRID2D::new([4096, 4096],
                                              [3.7 / 4096., 3.7 / 4096.],
                                              Point::new(-0.56, -0.06, &ABSOLUTE_COORDINATES),
                                              0.000001,
                                              ABSOLUTE_COORDINATES);



        //-65.5, 65.5, -64, 64 in mm on detector plane for vinietting

        let fuv_vinietting = SpatialEffect::from_fits(
            vinietting_grid,
            &config.clone().fuv_flatfield_illumination.1).unwrap();

        let nuv_vinietting = SpatialEffect::from_fits(
            vinietting_grid,
            &config.clone().nuv_flatfield_illumination.1).unwrap();

        let (detectors,detector_layout) = UVEX::detectors(config.clone());
        /*
        let (fuv_read_noise,
            nuv_read_noise,
            fuv_dark_current,
            nuv_dark_current,
            fuv_dead_pixels,
            nuv_dead_pixels) = UVEX::load_detector_effects(config.clone(),&detectors);

         */

        let (fuv_read_noise,
         nuv_read_noise,
         fuv_dark_current,
         nuv_dark_current,
         fuv_dead_pixels,
         nuv_dead_pixels) = (None,None,None,None,None,None);

        UVEX{
            mirror_response,
            dichroic_transmission,
            dichroic_reflection,
            nuv_filter,
            fuv_filter,
            //nuv_qe,
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

            fuv_read_noise,
            nuv_read_noise,
            fuv_dark_current,
            nuv_dark_current,
            fuv_dead_pixels,
            nuv_dead_pixels,
            fuv_vinietting,
            nuv_vinietting,
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
        //let deg_per_pixel = config.arcsec_per_pixel/(60.*60.);
        let x_gap_pixels = 0.;//T config.x_gap_mm*100.;
        let y_gap_pixels = 0.; //Tconfig.y_gap_mm*100.;

        //println!("Degrees per pixel: {:?}", deg_per_pixel);
        //TODO: maybe shift center?


        let detector_layout_grid = GRID2D::new(
            [config.x_detectors,config.y_detectors],
            [(config.x_pixels_per_detector as f64+ x_gap_pixels)/(60.*60.),
                (config.y_pixels_per_detector as f64+ y_gap_pixels)/(60.*60.)],
            Point::new(-0.56, -0.06,&ABSOLUTE_COORDINATES),
            0.0000000001,
            ABSOLUTE_COORDINATES);

        (detector_layout_grid.locate_grid_points().into_iter().flatten().map(|center|{
            let detector_grid = GRID2D::new(
                [config.x_pixels_per_detector,config.y_pixels_per_detector],
                [1./(60.*60.),1./(60.*60.)],
                center,
                0.00001,
                ABSOLUTE_COORDINATES);
            Detector::initialize(detector_grid,DetectorUnits::AverageElectrons)
        }).collect(),detector_layout_grid)


    }
    pub fn load_detector_effects(config:Config,detectors:&Vec<Detector>) -> (Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>,
                                                                             Vec<SpatialEffect>){
        let mut fuv_read_noise:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
            let grid = detector.grid;
            let path = format!("{}/fuv_read_noise_{:?}.fits",config.fuv_read_noise.1.clone().unwrap(),i);
            let mut effect = SpatialEffect::from_fits(grid,&path).unwrap();
                effect.cut_neg_values();
                effect
        }).collect();


        println!("Loaded FUV read noise");

        let nuv_read_noise:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/nuv_read_noise_{:?}.fits",config.nuv_read_noise.1.clone().unwrap(),i);
                let mut effect = SpatialEffect::from_fits(grid,&path).unwrap();
                effect.cut_neg_values();
                effect
            }).collect();
        println!("Loaded NUV read noise");

        let fuv_dark_current:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/fuv_dark_current_{:?}.fits",config.fuv_dark_current.1.clone().unwrap(),i);
                let mut effect = SpatialEffect::from_fits(grid,&path).unwrap();
                effect.cut_neg_values();
                effect
            }).collect();
        println!("Loaded FUV dark current");

        let nuv_dark_current:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/nuv_dark_current_{:?}.fits",config.nuv_dark_current.1.clone().unwrap(),i);
                let mut effect = SpatialEffect::from_fits(grid,&path).unwrap();
                effect.cut_neg_values();
                effect
            }).collect();
        println!("Loaded NUV dark current");


        let fuv_dead_pixels:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/fuv_dead_pixels_{:?}.fits",config.fuv_dead_pixels.1.clone().unwrap(),i);
                let mut effect = SpatialEffect::from_fits(grid,&path).unwrap();
                effect.cut_neg_values();
                effect
            }).collect();
        println!("Loaded FUV dead pixels");

        let nuv_dead_pixels:Vec<SpatialEffect> = detectors.iter()
            .enumerate()
            .map(|(i,detector)|{
                let grid = detector.grid;
                let path = format!("{}/nuv_dead_pixels_{:?}.fits",config.nuv_dead_pixels.1.clone().unwrap(),i);
                let mut effect = SpatialEffect::from_fits(grid,&path).unwrap();
                effect.cut_neg_values();
                effect
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


        GRID2D::new(
            [18, 18],
            [0.2, 0.2],
            Point::new(-0.56, -0.06, &ABSOLUTE_COORDINATES),
            0.00001,
            ABSOLUTE_COORDINATES)
    }

    pub fn make_nuv_background(&mut self, corners:[Vec<f64>;4], spectra:Vec<Spectrum>)-> SpatialEffect{
       // println!("Spectrum 1 is {:?}", spectra[0]);
        //println!("Spectrum 2 is {:?}", spectra[1]);
        //println!("NUV: {:?}",self.nuv);
        let nuv_bands:Vec<f64> = spectra.into_iter().map(|spectrum|{
            spectrum.apply_spectral_response(&mut self.nuv).integrate()
        }).collect();
        println!("NUV BANDS ARE {:?}", nuv_bands);
        let scaled_corners:Vec<f64> = corners.into_iter().map(|set|{
            set.into_iter().zip(nuv_bands.clone()).map(|(unscaled_corner,band)|{
                unscaled_corner*band
            }).sum::<f64>()
        }).collect();

        let scaled_corners:Vec<Vec<f64>> = scaled_corners.chunks(2).map(|c|c.to_vec()).collect();

        //TODO add this to config instead

        let background_grid = GRID2D::new(
            [2,2],
            [4.,4.],
            Point::new(-0.56, -0.06, &ABSOLUTE_COORDINATES),
            0.000000000001,
            ABSOLUTE_COORDINATES);

        SpatialEffect::new(background_grid,scaled_corners).unwrap()

    }


    pub fn run(&mut self, mut background:SpatialEffect, channel:Channel, input: Input) {
        let now = std::time::Instant::now();


        let mut polychromatic_bandpasses_nuv = (0..self.nuv_bands.len()-1).map(|i|{
            let lower = self.nuv_bands[i];
            let upper = self.nuv_bands[i+1];
            let directory = self.config.nuv_psf_directories[i].clone();
            println!("Bin {i}: {lower} - {upper} nm -> {directory}");
            SpectralResponse::square_wave(SPECTRAL_GRID,lower,upper)
        }).collect::<Vec<SpectralResponse>>();
        println!("Calculating total input counts:");
        let count = input.spectra[0].clone().apply_spectral_response(&self.nuv).integrate();
        println!("total COunts: {:?}", count*self.config.area_cm()*300.);
        println!("COunts w/ gain : {:?}", count*self.config.area_cm()*300./1.2);


        let banded_spectra:Vec<Vec<f64>> = input.spectra.clone().into_iter().map(|spectrum: Spectrum|{
            let mut spectra = spectrum.apply_spectral_response(&mut self.nuv.clone());
            let bands:Vec<f64> = polychromatic_bandpasses_nuv.iter_mut().enumerate().map(|(i,bin)|{
                let mut binned:Spectrum = spectra.clone().apply_spectral_response(bin); //photons/(s*cm^2*A)
                //after integrating, units are photons/(s*cm^2)
                let count_rate = binned.integrate()*self.config.area_cm();
                println!("COunts: {:?}", count_rate*300.);
                count_rate
            }).collect();
            //println!("BANDS ARE {:?}", bands);
            bands
        }).collect();




       // let background_spectrum:Spectrum = input.spectra[1].clone();
       // let background_bandpass = match channel{
       //     Channel::FUV => {background_spectrum.apply_spectral_response(&mut self.fuv).integrate()}
       //     Channel::NUV => {background_spectrum.apply_spectral_response(&mut self.nuv).integrate()}
       // };
        background.scale(self.config.area_cm());
        println!("area {:?}",self.config.area_cm());
        println!("BACKGROUND {:?}", background.value_at_index(0));
        println!("{:?} point sources with {:?} distinct spectra", input.points.len(), input.spectra.len());
        let start = Instant::now();
        println!("NUV spectral bins:");





        (0..self.detectors.len()).into_par_iter().for_each(|i|{
            let mut detector:Detector = self.detectors[i].clone();
            detector.add_background(&background);

            let mut dropped = 0;
            let detector_grid = detector.grid.clone();
            for source_number in 0..input.points.len() {
                if (source_number % 10000 == 0)&& (source_number>0) {
                    println!("Done {source_number} sources for detector number {i}");
                }
                let point:&Point = &input.points[source_number];
                //println!("PSF GRID FITING: {:?}", self.nuv_psfs[0].grid.fit_grid(point,true));
                //println!("{:?}", self.nuv_psfs[0].grid.grid_number(9,9));
               // let center_of_the_corner_pixel = point-&psf_half;
                let spectra_ref = input.spec_refs[source_number];
                let bands = &banded_spectra[spectra_ref];
                let scale = input.weights[source_number];




                match detector_grid.is_point_inside_with_margin(&point,13.) { //TODO was center
                    true => {
                       // println!("DIDNT DROP POINT");
                        match channel{
                            Channel::FUV => {panic!()}
                            Channel::NUV => {




                              //  println!("{:?}", bands);


                                //bands are in units of photons/s
                                let psf_datas:Vec<Vec<Vec<f64>>> = self.nuv_psfs
                                    .iter()
                                    .map(|(NUV_PSF_grid)|{
                                        let psf = NUV_PSF_grid.interpolated_psf(point);
                                        println!("PSF norm: {:?}",psf.iter().flatten().sum::<f64>());
                                        psf

                                    }).collect();

                                //TODO: ADD SCALE
                                detector.fast_add_multi(&point,&psf_datas, bands, scale)
                            }
                        }

                    }
                    false => {
                        //panic!();

                       // println!("point at {:?} was outside detecot {:?}", point.to_absolute(), i);


                        dropped += 1}
                }

            }
            println!("Done adding point sources in {:?}s, sum is {:?}, skipped {:?}", start.elapsed().as_secs(),self.detectors[i].data.iter().flatten().sum::<f64>(),dropped);

            match channel {
                Channel::FUV => {
                    panic!();

                    match self.config.fuv_flatfield_illumination.0{
                        Toggle::on => {
                            detector.multiply_effect2(&self.fuv_vinietting)
                        }
                        Toggle::off => {println!("skipped fuv vinietting")}
                    }

                    /*



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

                     */
                }
                Channel::NUV => {

                    match self.config.nuv_flatfield_illumination.0{
                        Toggle::on => {
                            let regridded_vinietting = SpatialEffect::new(detector.grid.clone(),self.nuv_vinietting.re_grid_values(detector.grid)).unwrap();
                            detector.multiply(&regridded_vinietting);
                        }
                        Toggle::off => {println!("skipped nuv vinietting")}
                    }
                    detector.plain2_to_electrons(self.config.simple_mode.0,
                                          self.config.simple_mode.1,
                                          self.config.nuv_exposure_time,
                                          self.config.saturation_num_electrons as f64,
                                          self.config.gain);


                    detector.write_to_fits(&format!("/Users/mayabasu/Desktop/Output/one_star_{i}.fits"))


                    /*

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

                     */


                }
            }



        });
        println!("Done run in {:?}s", now.elapsed().as_secs())
}

    }



