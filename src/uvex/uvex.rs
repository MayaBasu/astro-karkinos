use crate::geometry::GRID1D;
use crate::geometry::Grid1DUnits::nm;
use crate::gridded_data::{SpectralUnits, DATA1D};
use crate::gridded_data::SpectralUnits::{f_lambda, AbMagnitude};
use crate::telescope::{SpectralResponse, Spectrum};

pub fn demo(path:&str, prefix:&str){

    let snap_precision = 0.0001;
    let mut input = Spectrum::load(path, 0.001).expect("failed to load input");

    let grid = GRID1D::new(1000,100.,1000.,0.0001,nm);




    let bins = vec![100.,300.,400.,500.,700.,1000.];
    let psf_directories = vec![
        "/Users/mayabasu/Desktop/NUV Out-of-Band-2/300 nm",
        "/Users/mayabasu/Desktop/NUV Out-of-Band-2/400 nm",
        "/Users/mayabasu/Desktop/NUV Out-of-Band-2/500 nm",
        "/Users/mayabasu/Desktop/NUV Out-of-Band-2/700 nm",
        "/Users/mayabasu/Desktop/NUV Out-of-Band-2/1000 nm"];
    let mut bins = (0..bins.len()-1).map(|i|{
        let lower = bins[i];
        let upper = bins[i+1];
        let directory = psf_directories[i];
        println!("Bin {i}: {lower} - {upper} nm -> {directory}");
        SpectralResponse::square_wave(grid,lower,upper)
    }).collect::<Vec<SpectralResponse>>();


    /*

    let bins = bins.iter_mut().enumerate().map(|(i,bin)|{
        let mut binned = input.clone().apply_spectral_response(bin);
        binned.write_to_dat("# bin", &format!("src/outputs/{}_bin_{}",prefix,i));
        input.clone()
    }).collect::<Vec<Spectrum>>();

     */

    test(input.apply_spectral_response(&mut bins[4]))

    /*


    let mut mirror_response = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/mirror_reflectivity.dat",
        snap_precision).unwrap();

    let mut dichroic_reflection = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/reflection_UVIM_dichroic_response.dat",
        snap_precision).unwrap();

    let mut dichroic_transmission = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/transmission_UVIM_dichroic_response.dat",
        snap_precision).unwrap();

    let mut fuv_contamination = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_FUV_contamination.dat",
        snap_precision).unwrap();

    let mut nuv_contamination = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_NUV_contamination.dat",
        snap_precision
    ).unwrap();

    let mut fuv_filter_response = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_FUV_filter_response.dat",
        snap_precision
    ).unwrap();

    let mut nuv_filter_response = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_NUV_filter_response.dat",
        snap_precision
    ).unwrap();

    let mut nuv_qe = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_NUV_QE.dat",
        snap_precision
    ).unwrap();


    let fuv_path = SpectralResponse::multiply_responses(vec![
        fuv_contamination, fuv_filter_response
    ]);


    dichroic_reflection.regrid(&grid).write_to_dat("selkfjslef", "lskef");
    nuv_qe.regrid(&grid).write_to_dat("#selkfjslef", "lskef");

     */


}


pub fn test(input:Spectrum){
    let snap_precision = 0.0001;

    let grid = GRID1D::new(1000,100.,1000.,0.0001,nm);

    //let mut input = Spectrum::new_flat(grid, 11.,AbMagnitude);
    //let mut input = Spectrum::load("/Users/mayabasu/PycharmProjects/make_inputs/inputs/input_spectra_1", 0.001).expect("failed to load input");



    let mut input = input.convert_to(&f_lambda);
    input.write_to_dat("#generated_flat","src/test_outputs/input");

    let mut mirror_response = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/mirror_reflectivity.dat",
        snap_precision).unwrap();

    mirror_response.write_to_dat("#1X mirror", "src/test_outputs/mirror");
    let mut triple_mirror = mirror_response.multiply(&mirror_response).multiply(&mirror_response);
    let mut mirror_plus_input = input.clone().apply_spectral_response(&mut triple_mirror);

    mirror_response.write_to_dat("#1X mirror", "src/test_outputs/mirror");
    triple_mirror.write_to_dat("#3X mirror", "src/test_outputs/triple_mirror");
    mirror_plus_input.write_to_dat("#3X mirror + input", "src/test_outputs/mirror_plus_input");

    let mut dichroic_reflection = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/reflection_UVIM_dichroic_response.dat",
        snap_precision).unwrap();

    let mut dichroic_transmission = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/transmission_UVIM_dichroic_response.dat",
        snap_precision).unwrap();


    dichroic_reflection.write_to_dat("#dichroic reflection","src/test_outputs/reflection");
    dichroic_transmission.write_to_dat("#dichroic transmission","src/test_outputs/transmission");


    let mut tot_ref = mirror_plus_input.clone().apply_spectral_response(&mut dichroic_reflection);
    let mut tot_trans = mirror_plus_input.apply_spectral_response(&mut dichroic_transmission);


     //   let mut mirror_transmission_input = mirror_plus_input.clone().apply_spectral_response(&mut dichroic_transmission);
     //   let mut mirror_reflection_input = input.apply_spectral_response(&mut triple_mirror).apply_spectral_response(&mut dichroic_reflection);

            //mirror_plus_input.apply_spectral_response(&mut dichroic_reflection);

    tot_ref.write_to_dat("#3X mirror + dichroic reflection + input","src/test_outputs/mirror_ref");

    tot_trans.write_to_dat("#3X mirror + dichroic transmission + input","src/test_outputs/mirror_trans");


    let mut fuv_filter_response = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_FUV_filter_response.dat",
        snap_precision
    ).unwrap();

    let mut nuv_filter_response = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_NUV_filter_response.dat",
        snap_precision
    ).unwrap();

    let mut nuv_qe = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_NUV_QE.dat",
        snap_precision
    ).unwrap();


    let mut nuv_filter_plus_qe = nuv_qe.multiply(&mut nuv_filter_response);
    nuv_filter_plus_qe.write_to_dat("#qe plus filter for nuv", "src/test_outputs/nuv_filter_plus_qe");

    fuv_filter_response.write_to_dat("# fuv filter response", "src/test_outputs/fuv_filter");
    nuv_filter_response.write_to_dat("# nuv filter response", "src/test_outputs/nuv_filter");
    nuv_qe.write_to_dat("# nuv QE", "src/test_outputs/nuv_qe");

    let mut tot_ref_plus_filter = tot_ref.apply_spectral_response(&mut fuv_filter_response);
    let mut tot_trans_plus_filter = tot_trans.apply_spectral_response(&mut nuv_filter_response)
        .apply_spectral_response(&mut nuv_qe);
    tot_ref_plus_filter.write_to_dat("#mirror ref fuv filter", "src/test_outputs/fuv");
    tot_trans_plus_filter.write_to_dat("#mirror ref nuv filter nuv qe", "src/test_outputs/nuv");



    let mut fuv_contamination = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_FUV_contamination.dat",
        snap_precision).unwrap();

    let mut nuv_contamination = SpectralResponse::load(
        "/Users/mayabasu/PycharmProjects/make_inputs/inputs/UVIM_NUV_contamination.dat",
        snap_precision
    ).unwrap();

    fuv_contamination.write_to_dat("#fuv contamination", "src/test_outputs/fuv_con");
    nuv_contamination.write_to_dat("#nuv contamination", "src/test_outputs/nuv_con");


        let mut full_fuv = tot_ref_plus_filter.apply_spectral_response(&mut fuv_contamination);
        let mut full_nuv = tot_trans_plus_filter.apply_spectral_response(&mut nuv_contamination);
    full_fuv.write_to_dat("#full fuv", "src/test_outputs/full_fuv");
    full_nuv.write_to_dat("#full nuv", "src/test_outputs/full_nuv");
}


pub struct UVEX{

    mirror_response: SpectralResponse,

    dichroic_transmission: SpectralResponse,
    dichroic_reflection: SpectralResponse,

    nuv_filter: SpectralResponse,
    fuv_filter: SpectralResponse,

    nuv_qe: SpectralResponse,

    nuv_contamination: SpectralResponse,
    fuv_contamination: SpectralResponse


}

impl UVEX{
    pub fn load() -> Self{

    }
}



