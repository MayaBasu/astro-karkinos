use crate::geometry::{CoordinateSystem, Point, GRID2D};
use crate::gridded_data::{DATA1D, DATA2D};
use crate::gridded_data::DetectorUnits::Electrons;
use crate::telescope::{Detector, PsfGrid, SpatialEffect, SpectralResponse, Spectrum};
use crate::uvex::config::Config;

pub struct UVEX{

    pub fuv: SpectralResponse,
    pub nuv: SpectralResponse,

    pub nuv_bands: Vec<f64>,
    pub nuv_psfs: Vec<PsfGrid>,

    pub fuv_psf: PsfGrid,
    pub sky_coord: CoordinateSystem,
    pub detector: Detector,
    pub psf_coords: CoordinateSystem,



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

        let full_fuv = mirror_response.clone()
            .multiply(&mirror_response)
            .multiply(&mirror_response)
            .multiply(&dichroic_reflection)
            .multiply(&fuv_filter)
            .multiply(&fuv_contamination);

        let full_nuv = mirror_response.clone()
            .multiply(&mirror_response)
            .multiply(&mirror_response)
            .multiply(&dichroic_transmission)
            .multiply(&nuv_filter)
            .multiply(&nuv_qe)
            .multiply(&nuv_contamination);

        let sky_coordinates = CoordinateSystem::new(
            [1.,0.],
            [0.,1.],
            [0.,0.]
        );

        let psf_grid = GRID2D::new(
            [18, 18],
            [0.2, 0.2],
            Point::new(-0.56, -0.06, &sky_coordinates),
            0.00001,
            sky_coordinates.clone());


        let mut fuv_psf = PsfGrid::new(
            "FUV PSF grid".to_string(),
            psf_grid,
            config.fuv_psf_directory,
            ("XFLD".to_string(), "YFLD".to_string()));
        fuv_psf.load_data_frames(64,64);
        let mut nuv_psfs = Vec::new();

        for path in config.nuv_psf_directories{
            let mut psfs = PsfGrid::new(
                path.clone(),
                psf_grid,
                path.clone(),
                ("XFLD".to_string(), "YFLD".to_string()));
            psfs.load_data_frames(128,128);
            nuv_psfs.append(*psfs)

        }

        let PSF_coords = CoordinateSystem::new(
            [0.1,0.],
        [0.,0.1],
        [0.,0.]);


        let PsfGrid = GRID2D::new(
        [128,128],
        [1.,1.],
        Point::new(0.,0.,&PSF_coords),
        0.0001,
        PSF_coords,
        );

        /*


        let test_point = Point::new(0.,0.,&sky_coordinates);
        let psf_1 = nuv_1000_psf.interpolated_psf(&test_point);//nuv_300_psf.interpolated_psf(&test_point);
        let patch1 = SpatialEffect::new(PsfGrid.clone(), psf_1).unwrap();

        let PsfGrid = GRID2D::new(
            [128,128],
            [1.,1.],
            Point::new(50.,50.,&PSF_coords),
            0.0001,
            PSF_coords,
        );

        let psf_2 = nuv_300_psf.interpolated_psf(&test_point);
        let patch2 = SpatialEffect::new(PsfGrid.clone(), psf_2).unwrap();
        let detector_grid = GRID2D::new(
            [228,228],
            [1.,1.],
            Point::new(0.,0.,&PSF_coords),
            0.0001,
            PSF_coords,
        );
        let mut detector = Detector::initialize(detector_grid,Electrons);
        detector.add(&patch2.clone());
        detector.add(&patch1.clone());
        detector.write_to_fits("detector.fits");

        patch1.write_to_fits("patch1.fits");
        patch2.write_to_fits("patch2.fits");
        println!("{:?}",patch2.data());

         */


        let detector_coords = CoordinateSystem::new(
            [0.1,0.0],
            [0.,0.1],
            [0.,0.]);

        let detector_grid = GRID2D::new(
            [13*5,13],
            [1.,1.],
            Point::new(0.,0.,&detector_coords),
            0.0001,
            detector_coords,
        );
        let mut detector = Detector::initialize(detector_grid,Electrons);





        UVEX{
            fuv,
            nuv,
            nuv_bands: config.nuv_bands,
            nuv_psfs,
            fuv_psf,
            sky_coord: sky_coordinates,
            detector,
            psf_coords: PSF_coords,
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
    pub fn triple_mirror(&self) -> SpectralResponse{

    }







}



