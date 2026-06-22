use astroimsim_geometry::coordinate_system::{CoordinateSystem, Coordinates};
use astroimsim_geometry::coordinate_system::Coordinates::{ABSOLUTE, RELATIVE};
use astroimsim_geometry::grid1d::GRID1D;
use astroimsim_geometry::grid2d::GRID2D;
use astroimsim_geometry::points::Point;
use astroimsim_spectra::power_spectrum::{PowerSpectrum, SpectrumUnits};
use astroimsim_spectra::spectral_response::SpectralResponseCurve;
use crate::detector::{Detector, DetectorArray};
use crate::psf::SpatialEffect;
use crate::psf_grid::PsfGrid;

pub struct UVEX{
    fuv_psf_path: &'static str,
    fuv_psf_grid: GRID2D,
    x_gap: f64,
    y_gap:f64,
    detector_array: DetectorArray,
    flatfield_illumination: SpatialEffect,
    fuv_response: SpectralResponseCurve,
    nuv_response: SpectralResponseCurve,
    num_pixels: usize,
    detector_coordinates: Coordinates,

}

impl UVEX{
    pub fn initialize(
        fuv_psf_path: &'static str,
        flatfield_path: &'static str,
    ) {
        //load in flatfield illumination
        let flatfield_grid = UVEX::flatfield_illumination_grid();
        let mut flatfield = SpatialEffect::new_empty("Flatfield Illumination",flatfield_grid,flatfield_path);
        flatfield.load_data();
        //load in fuv psf files
        let center_keys = ("XFLD", "YFLD");
        let fuv_psf_grid = UVEX::empty_fuv();
        let mut fuv_psf = PsfGrid::new("FUV PSF",fuv_psf_grid,fuv_psf_path,center_keys);
        fuv_psf.load_data_frames(64,64);
        println!("{:?}",fuv_psf);
        let (fuv,nuv) = UVEX::fuv_nuv();




    }

    pub fn flatfield_illumination_grid() ->GRID2D{

        let num_pixels =512;
        let flatfield_width_degrees = 4.52;
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

    pub fn uvex_detector_array(num_pixels: usize, x_gap:f64, y_gap:f64) -> DetectorArray{
        let detector_width_degrees = 3.0;
        let center = Point::new(-0.5,0.0,Coordinates::ABSOLUTE);
        let num_detectors_y = 1;
        let num_detectors_x = 1;

        let pixel_to_deg_scale = detector_width_degrees/num_pixels as f64; //Degrees in FOV to pixels
        let detectors_x_axis = (pixel_to_deg_scale,0.0);
        let detectors_y_axis = (0.0,pixel_to_deg_scale);
        /*
        let coordinate_system = CoordinateSystem::new(
            detectors_x_axis,
            detectors_y_axis,
            center.values(),
            "Detector Coordinate System".to_string(),
            "magenta".to_string());

         */

        let coordinate_system = CoordinateSystem{
            x_axis: detectors_x_axis,
            y_axis: detectors_y_axis,
            center: (0.0, 0.0),
            color: "red".to_string(),
            label: "detector grid".to_string(),
        };


        let detector_grid = GRID2D::new_empty(
            (num_detectors_x,num_detectors_x),
            (1.0 + x_gap,1.0 + y_gap),
            (-0.56, -0.06),
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

        DetectorArray{
            label: "UVEX Detector Array".to_string(),
            detectors,
            coordinate_system,
        }


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
        const FUV_CONTAMINATION_PATH: &'static str = "src/spectral_response_files/UVIM_FUV_contamination.dat";
        const NUV_CONTAMINATION_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_contamination.dat";
        const FUV_RESPONSE_PATH: &'static str = "src/spectral_response_files/UVIM_FUV_filter_response.dat";
        const NUV_RESPONSE_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_filter_response.dat";
        const NUV_QE_CURVE_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_QE.dat";
        const DICHROIC_PATH: &'static str = "/Users/mayabasu/RustroverProjects/astroimsim-spectra/src/spectral_response_files/UVIM_dichroic_response.dat";
        const MIRROR_PATH: &'static str = "src/spectral_response_files/mirror_reflectivity.dat";

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

}