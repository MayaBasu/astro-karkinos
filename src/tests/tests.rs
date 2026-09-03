use plotpy::Plot;
use crate::geometry::*;
use rand::distr::{Distribution, Uniform};
use crate::geometry::Coordinates::RELATIVE;
use crate::power_spectrum::{PowerSpectrum, SpectrumUnits};
use crate::spectral_response::SpectralResponseCurve;



#[test]
pub fn test_coordinate_systems(){

    let abs = Coordinates::ABSOLUTE;

    let between = Uniform::try_from(-5.0..5.0).unwrap();
    let mut rng = rand::rng();

    let rel1 = CoordinateSystem::new(
        (between.sample(&mut rng), between.sample(&mut rng)),
        (between.sample(&mut rng), between.sample(&mut rng)),
        (between.sample(&mut rng), between.sample(&mut rng)),
        "relative 1".to_string());

    let rel2 = CoordinateSystem::new(
        (between.sample(&mut rng), between.sample(&mut rng)),
        (between.sample(&mut rng), between.sample(&mut rng)),
        (between.sample(&mut rng), between.sample(&mut rng)),
        "relative 2".to_string());

    let point_1 = Point::new(
        between.sample(&mut rng),
        between.sample(&mut rng),
        abs.clone());

    let point_2 = Point::new(
        between.sample(&mut rng),
        between.sample(&mut rng),
        Coordinates::RELATIVE(rel1.clone()));

    let point_3 = Point::new(
        between.sample(&mut rng),
        between.sample(&mut rng),
        Coordinates::RELATIVE(rel2.clone()));


    let mut plot = Plot::new();

  


    println!("{:?}", rel1);
    println!("{:?}", rel2);

    println!("{:?}", point_1);
    println!("{:?}", point_2);
    println!("{:?}", point_3);
    println!("----------------------- \n \n");


    println!("{:?}", point_1.to_absolute());
    println!("{:?}", point_2.to_absolute());
    println!("{:?}", point_3.to_absolute());

    abs.plot(&mut plot, "black".to_string());
    rel1.plot(&mut plot, "red".to_string());
    rel2.plot(&mut plot, "blue".to_string());

    point_1.plot(&mut plot, "yellow".to_string());
    point_2.plot(&mut plot, "orange".to_string());
    point_3.plot(&mut plot, "pink".to_string());
    
    assert!((point_1.to_absolute().convert(&abs).values().0- point_1.values().0) < 0.0000001,"ahh");



    plot.show("coord_sys_test").expect("Could not plot for test");

}


pub fn visulaize(){

    let STANDARD_SPECTRAL_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);




    const FUV_CONTAMINATION_PATH: &'static str = "src/spectral_response_files/UVIM_FUV_contamination.dat";
    const NUV_CONTAMINATION_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_contamination.dat";
    const FUV_RESPONSE_PATH: &'static str = "src/spectral_response_files/UVIM_FUV_filter_response.dat";
    const NUV_RESPONSE_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_filter_response.dat";
    const NUV_QE_CURVE_PATH: &'static str = "src/spectral_response_files/UVIM_NUV_QE.dat";
    const DICHROIC_PATH: &'static str = "/Users/mayabasu/RustroverProjects/astroimsim-spectra/src/spectral_response_files/UVIM_dichroic_response.dat";
    const MIRROR_PATH: &'static str = "src/spectral_response_files/mirror_reflectivity.dat";

    let  FUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
    let NUV_CONTAMINATION_GRID: GRID1D = GRID1D::new_empty(1.0,110.0,999.0,0.01,1.0);
    let FUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1.0);
    let NUV_RESPONSE_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1050.0,0.01,1.0);
    let NUV_QE_CURVE_GRID: GRID1D = GRID1D::new_empty(1.0,100.0,1100.0,0.01,1000.0);
    let DICHROIC_GRID: GRID1D = GRID1D::new_empty(1.0,120.0,1000.0,0.01,1000.0);
    let MIRROR_GRID:GRID1D = GRID1D::new_empty(1.0,110.0,1100.0,0.01,1.0);

    //  let FUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("FUV_CONTAMINATION",FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH);
    // let NUV_CONTAMINATION: SpectralResponseCurve = SpectralResponseCurve::new("NUV_CONTAMINATION",NUV_CONTAMINATION_GRID,NUV_CONTAMINATION_PATH);
    let mut FUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("FUV_DICHROIC".to_string(),DICHROIC_GRID.clone(),DICHROIC_PATH.to_string(),1,"      ");
    let mut NUV_DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("NUV_DICHROIC".to_string(),DICHROIC_GRID,DICHROIC_PATH.to_string(),2,"      ");
    let mut  NUV_QE: SpectralResponseCurve = SpectralResponseCurve::new("NUV_QE".to_string(),NUV_QE_CURVE_GRID,NUV_QE_CURVE_PATH.to_string(),1,"   ");
    let mut FUV_CONTAMINATION:SpectralResponseCurve = SpectralResponseCurve::new("FUV Contamination".to_string(),FUV_CONTAMINATION_GRID,FUV_CONTAMINATION_PATH.to_string(),1,"   ");
    FUV_CONTAMINATION.write_to_dat("contamination","FUV contamination",&STANDARD_SPECTRAL_GRID);
    let mut NUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("NUV Filter response".to_string(),NUV_RESPONSE_GRID,NUV_RESPONSE_PATH.to_string(),1,"   ");
    let mut FUV_FILTER_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("FUV Filter response".to_string(),FUV_RESPONSE_GRID,FUV_RESPONSE_PATH.to_string(),1,"   ");

    NUV_FILTER_CURVE.write_to_dat("nuv_filter_curve","NUV Filter curve",&STANDARD_SPECTRAL_GRID);
    FUV_FILTER_CURVE.write_to_dat("fuv_filter_curve","FUV Filter curve",&STANDARD_SPECTRAL_GRID);
    NUV_QE.write_to_dat("nuv_qe_curve","NUV QE curve",&STANDARD_SPECTRAL_GRID);

    let mut NUV_FILTER_QE = SpectralResponseCurve::compose(vec![NUV_FILTER_CURVE,NUV_QE]);
    NUV_FILTER_QE.write_to_dat("nuv_filter_qe","NUV filter cuve + NUV QE",&STANDARD_SPECTRAL_GRID);


    //let DICHROIC: SpectralResponseCurve = SpectralResponseCurve::new("DICHROIC",DICHROIC_GRID,DICHROIC_PATH);
    let mut MIRROR_CURVE:SpectralResponseCurve = SpectralResponseCurve::new("Mirror".to_string(), MIRROR_GRID, MIRROR_PATH.to_string(),1,"    ");
    MIRROR_CURVE.re_grid(&STANDARD_SPECTRAL_GRID);
    let mut MIRROR_CURVE_3 = MIRROR_CURVE.clone();
    MIRROR_CURVE_3.self_compose(3);
    MIRROR_CURVE_3.re_grid(&STANDARD_SPECTRAL_GRID);


    FUV_DICHROIC.write_to_dat("fuvdichroic","fuv path of dichroic (transmission)",&STANDARD_SPECTRAL_GRID);
    NUV_DICHROIC.write_to_dat("nuvdichroic","nuv path (reflection)",&STANDARD_SPECTRAL_GRID);

    let mut input_spectrum = PowerSpectrum::flat_AB(20.0,STANDARD_SPECTRAL_GRID.clone(),"Input Spectrum".to_string());

    input_spectrum.convert_to(&SpectrumUnits::f_lambda);
    input_spectrum.write_to_dat("input","initial spectrum");

    MIRROR_CURVE.write_to_dat("mirror","mirror response curve",&STANDARD_SPECTRAL_GRID);
    MIRROR_CURVE_3.write_to_dat("mirror3","3 mirror response curves composed together",&STANDARD_SPECTRAL_GRID);

    input_spectrum.apply_spectral_response(&MIRROR_CURVE_3);
    input_spectrum.write_to_dat("input_plus_mirrors","Input spectrum with the mirror curve applied three times");
    let mut input_spectrum = PowerSpectrum::flat_AB(20.0,STANDARD_SPECTRAL_GRID,"Input Spectrum".to_string());
    input_spectrum.convert_to(&SpectrumUnits::f_lambda);
    let mut fuv_path = input_spectrum.clone();
    let mut nuv_path = input_spectrum;
    fuv_path.apply_spectral_response(&MIRROR_CURVE_3);
    fuv_path.apply_spectral_response(&FUV_DICHROIC);
    fuv_path.write_to_dat("fuv_dic_mir","Input spectrum with FUV and 3 mirror curves ");
    fuv_path.apply_spectral_response(&FUV_FILTER_CURVE);
    fuv_path.write_to_dat("fuv_dic_mir_qe","Input spectrum with FUV and 3 mirror curves  abd QE");


    nuv_path.apply_spectral_response(&MIRROR_CURVE_3);
    nuv_path.apply_spectral_response(&NUV_DICHROIC);
    nuv_path.write_to_dat("nuv_dic_mir","Input spectrum with NUV and 3 mirror curves ");
    nuv_path.apply_spectral_response(&NUV_FILTER_QE);
    nuv_path.write_to_dat("nuv_dic_mir_qe","Input spectrum with NUV and 3 mirror curves  abd QE");

    // println!("{:?}",input_spectrum)

    nuv_path.apply_spectral_response(&FUV_CONTAMINATION);

    nuv_path.write_to_dat("full_nuv","All effects NUV");

    fuv_path.apply_spectral_response(&FUV_CONTAMINATION);
    fuv_path.write_to_dat("full_fuv","All effects FUV");



    println!("fuv: {:?}",fuv_path.total_average_photon_flux(4417.864669110647)*0.76153773);
    println!("nuv: {:?}",nuv_path.total_average_photon_flux(4417.864669110647)*0.76153773);



}



use crate::spatial_effect::SpatialEffect;

pub fn test_2d_interpolation() {
    let coordinates = CoordinateSystem::new(
        (0.0, 1.0),
        (1.0, 0.0),
        (0.0, 0.0),
        "test coords".to_string(),
    );

    let test_grid = GRID2D::new_empty((100, 100), (0.010, 0.010), (10.0, 10.0), 0.45, RELATIVE(coordinates));
    let mut matrix = Vec::new();
    for row in 0..test_grid.y_num{
        let mut row_vec = Vec::new();
        for column in 0..test_grid.x_num{
            let point = test_grid.locate(test_grid.grid_number(column,row));
            row_vec.push(test_function(&point))

        }
        matrix.push(row_vec)
    }

    let data_grid = SpatialEffect::from_matrix("data grid".to_string(), test_grid.clone(), "NA".to_string(), matrix);

    println!("data grid is :{:?}",data_grid);


    for i in 0..100{
        let point = test_grid.random();
        let datum1 = data_grid.get_data(&point);
        let datum2 = test_function(&point);
        println!(" {:?} {:?}   {:?}  {:?}",test_grid.fit_grid(&point), datum1,datum2, 100.0*(datum2-datum1)/datum2)
    }
}

pub fn test_function(point: &Point)-> f64{
    let (x,y) = point.to_absolute().values();
    let result = x.powi(3) + y.powi(2);
    if result.is_nan(){
        println!("{:?} gave NaN",(x,y))
    }
    result
}

pub fn test_project(){
    let mut plot = Plot::new();
    let coords = CoordinateSystem::new((1.0,0.0),(0.0,1.0),(0.0,0.0), "test".to_string());
    let grid = GRID2D::new_empty((3,3),(1.0,1.0), (0.0,0.0), 0.01,RELATIVE(coords.clone()));

    let point = Point::new(-0.8,0.7, RELATIVE(coords));
    point.plot(&mut plot, "orange".parse().unwrap());
    let projected_point = grid.project(&point);
    grid.plot_points(&mut plot, PlotPoint::Given(projected_point));
    plot.show("test").unwrap();

}
