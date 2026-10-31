use serde::{Deserialize, Serialize};

#[derive(Clone,Copy,Serialize,Deserialize,Debug)]
pub enum Toggle{
    on,
    off,
}



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config{

    pub mirror_response: (Toggle, String),
    pub dichroic_transmission: (Toggle, String),
    pub dichroic_reflection: (Toggle, String),

    pub nuv_filter: (Toggle, String),
    pub fuv_filter: (Toggle, String),

    //pub nuv_qe: (Toggle, String),

    pub nuv_contamination: (Toggle, String),
    pub fuv_contamination: (Toggle, String),

    pub nuv_bands: Vec<f64>,
    pub nuv_psf_directories: Vec<String>,

    pub fuv_psf_directory: String,

    pub fuv_flatfield_illumination: (Toggle, String),
    pub nuv_flatfield_illumination: (Toggle, String),

    pub fuv_read_noise: (Toggle, Option<String>),
    pub nuv_read_noise: (Toggle, Option<String>),

    pub fuv_dark_current:  (Toggle, Option<String>),
    pub nuv_dark_current:  (Toggle, Option<String>),

    pub fuv_dead_pixels: (Toggle, Option<String>),
    pub nuv_dead_pixels:  (Toggle,Option< String>),

    pub psf_blur: (Toggle, String),

    pub x_gap_mm: f64,
    pub y_gap_mm: f64,

    pub entrance_pupil_diameter_cm: f64,

    pub x_pixels_per_detector: usize,
    pub y_pixels_per_detector: usize,

    pub x_detectors: usize,
    pub y_detectors: usize,

    pub pixel_size_um : f64,

    pub arcsec_per_pixel: f64,

    pub saturation_num_electrons: usize,

    // dark current (e/s per pixel), read noise (electrons rms)
    pub simple_mode: (f64,f64),

    pub gain: f64,

    pub epd_cm:f64,

    pub fuv_exposure_time: f64,
    pub nuv_exposure_time:f64,

}

impl Default for Config{
    fn default() -> Self {
        Self {
            mirror_response: (Toggle::on, "/Users/mayabasu/Desktop/CALDB_Rust/mirror_reflectivity.dat".to_string()),
            dichroic_transmission: (Toggle::on, "/Users/mayabasu/Desktop/CALDB_Rust/transmission_UVIM_dichroic_response.dat".to_string()),
            dichroic_reflection: (Toggle::on, "/Users/mayabasu/Desktop/CALDB_Rust/reflection_UVIM_dichroic_response.dat".to_string()),
            nuv_filter: (Toggle::on, "/Users/mayabasu/Desktop/CALDB_Rust/UVIM_NUV_filter_response.dat".to_string()),
            fuv_filter: (Toggle::on, "/Users/mayabasu/Desktop/CALDB_Rust/UVIM_FUV_filter_response.dat".to_string()),
            //nuv_qe: (Toggle::on, "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_NUV_QE.dat".to_string()),
            nuv_contamination: (Toggle::on, "/Users/mayabasu/Desktop/CALDB_Rust/UVIM_NUV_contamination.dat".to_string()),
            fuv_contamination: (Toggle::on, "/Users/mayabasu/Desktop/CALDB_Rust/UVIM_FUV_contamination.dat".to_string()),
            nuv_bands: vec![0., 270., 300.,400.,500.,700.,1000.],
            //nuv_bands: vec![0.,1000.],

            /*

            nuv_psf_directories: vec!["/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/300 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/400 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/500 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/700 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/1000 nm".to_string()
            ],

             */

            /*
            nuv_psf_directories: vec!["/Users/mayabasu/Desktop/NUV Out-of-Band-2/300 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/400 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/500 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/700 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/1000 nm".to_string()
            ],

             */





            nuv_psf_directories: vec!["/Users/mayabasu/Desktop/uvex/NUV_PSF_100mas".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/300nmblur_100mas".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/400nmblur_100mas".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/500nmblur_100mas".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/700nmblur_100mas".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/1000nmblur_100mas".to_string()
            ],


            fuv_psf_directory: "/Users/mayabasu/Desktop/uvex/FUV_PSF".to_string(),
            fuv_flatfield_illumination: (Toggle::on, "/Users/mayabasu/Desktop/uvex/vinietting/FUV_vignetting_model_4096.fits".to_string()),
            nuv_flatfield_illumination: (Toggle::on, "/Users/mayabasu/Desktop/uvex/vinietting/NUV_vignetting_model_4096.fits".to_string()),

            //Generated effects
            fuv_read_noise: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/fuv_read_noise".to_string())),
            nuv_read_noise: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/nuv_read_noise".to_string())),

            fuv_dark_current: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/fuv_dark_current".to_string())),
            nuv_dark_current: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/nuv_dark_current".to_string())),

            fuv_dead_pixels: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/fuv_dead_pixels".to_string())),
            nuv_dead_pixels: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/nuv_dead_pixels".to_string())),

            psf_blur: (Toggle::on, "".to_string()),


            x_gap_mm: 4.5,
            y_gap_mm: 2.55,
            entrance_pupil_diameter_cm: 75.0,
            x_pixels_per_detector: 4096,
            y_pixels_per_detector: 4096,
            x_detectors: 3,
            y_detectors: 3,
            pixel_size_um: 10.0,
            arcsec_per_pixel: 1.03,
            saturation_num_electrons: 15000,
            simple_mode: (0.001, 3.),
            //simple_mode: (0.000, 0.0),
            gain: 1.2, //Electrons per ADU, gets rounded down after applying this
            epd_cm: 75.,
            fuv_exposure_time: 900.,
            nuv_exposure_time: 300.,

        }

    }



}
impl Config{
    pub fn area_cm(&self)->f64{
        //(self.epd_cm/2.).powi(2)* std::f64::consts::PI
        //TODO
        4417.86466911
    }


    pub(crate) fn mini() -> Self {
        Self {
            mirror_response: (Toggle::on, "/Users/mayabasu/Desktop/uvex/rustsim-2/mirror_reflectivity.dat".to_string()),
            dichroic_transmission: (Toggle::on, "/Users/mayabasu/Desktop/uvex/rustsim-2/transmission_UVIM_dichroic_response.dat".to_string()),
            dichroic_reflection: (Toggle::on, "/Users/mayabasu/Desktop/uvex/rustsim-2/reflection_UVIM_dichroic_response.dat".to_string()),
            nuv_filter: (Toggle::on, "/Users/mayabasu/Desktop/uvex/rustsim-2/UVIM_NUV_filter_response.dat".to_string()),
            fuv_filter: (Toggle::on, "/Users/mayabasu/Desktop/uvex/rustsim-2/UVIM_FUV_filter_response.dat".to_string()),
            //nuv_qe: (Toggle::on, "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_NUV_QE.dat".to_string()),
            nuv_contamination: (Toggle::on, "/Users/mayabasu/Desktop/uvex/rustsim-2/UVIM_FUV_contamination.dat".to_string()),
            fuv_contamination: (Toggle::on, "/Users/mayabasu/Desktop/uvex/rustsim-2/UVIM_FUV_contamination.dat".to_string()),
            nuv_bands: vec![0.,500.,1000.],
            //270., 300.,400.,500.,700.

            /*
            nuv_psf_directories: vec!["/Users/mayabasu/Desktop/NUV Out-of-Band-2/300 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/400 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/500 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/700 nm".to_string(),
                                      "/Users/mayabasu/Desktop/NUV Out-of-Band-2/1000 nm".to_string()
            ],

             */



            nuv_psf_directories: vec!["/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string(),
                                       // "/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string(),
                                     // "/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string(),
                                      //"/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string(),
                                     // "/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string(),
                                      "/Users/mayabasu/Desktop/uvex/NUV_PSF".to_string()
            ],
            fuv_psf_directory: "/Users/mayabasu/Desktop/uvex/FUV_PSF".to_string(),
            fuv_flatfield_illumination: (Toggle::on, "/Users/mayabasu/Desktop/uvex/vinietting/FUV_vignetting_model_4096.fits".to_string()),
            nuv_flatfield_illumination: (Toggle::on, "/Users/mayabasu/Desktop/uvex/vinietting/NUV_vignetting_model_4096.fits".to_string()),

            //Generated effects
            fuv_read_noise: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/fuv_read_noise".to_string())),
            nuv_read_noise: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/nuv_read_noise".to_string())),

            fuv_dark_current: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/fuv_dark_current".to_string())),
            nuv_dark_current: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/nuv_dark_current".to_string())),

            fuv_dead_pixels: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/fuv_dead_pixels".to_string())),
            nuv_dead_pixels: (Toggle::on, Some("/Users/mayabasu/Desktop/uvex/detector_effects/nuv_dead_pixels".to_string())),

            psf_blur: (Toggle::on, "".to_string()),


            x_gap_mm: 4.5,
            y_gap_mm: 2.55,
            entrance_pupil_diameter_cm: 75.0,
            x_pixels_per_detector: 500,
            y_pixels_per_detector: 500,
            x_detectors: 1,
            y_detectors: 1,
            pixel_size_um: 10.0,
            arcsec_per_pixel: 1.03,
            saturation_num_electrons: 15000,
            simple_mode: (0., 0.),
            gain: 1.2, //Electrons per ADU, gets rounded down after applying this
            epd_cm: 75.,
            fuv_exposure_time: 900.,
            nuv_exposure_time: 300.,

        }

    }
}

