use crate::geometry::GRID2D;
use crate::geometry::{Point, GRID1D};
use crate::geometry::Grid1DUnits::nm;
use crate::gridded_data::{SpectralUnits, DATA1D, DATA2D};
use crate::gridded_data::SpectralUnits::{f_lambda, AbMagnitude};
use crate::telescope::{SpatialEffect, SpectralResponse, Spectrum};
use crate::uvex::uvex::UVEX;

pub fn demo(path:&str,temp:usize)->(Vec<f64>,Vec<f64>){

    let mut input = Spectrum::load(path, 0.001).expect("failed to load input");


    let grid = GRID1D::new(9900,100.,1000.,0.0001,nm);




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

    let mut uvex = UVEX::load();

    let unfilterd_sums:Vec<f64> = bins.iter_mut().enumerate().map(|(i,bin)|{
        let mut binned = input.clone().apply_spectral_response(bin);
        binned.write_to_dat("#", &format!("src/outputs/{temp}_bin_{i}"));
        binned.sum()
    }).collect();
    let total_unflitered_sum:f64 = unfilterd_sums.iter().sum();

    let unfiltered = unfilterd_sums.iter().map(|s|*s).collect::<Vec<f64>>();



    let sums:Vec<f64> = bins.iter_mut().enumerate().map(|(i,bin)|{
        let mut binned = input.clone().apply_spectral_response(bin);
        let mut binned = binned.apply_spectral_response(&mut uvex.full_nuv);
        binned.write_to_dat("#", &format!("src/outputs/{temp}_bin_{i}_filtered"));
        binned.sum()
    }).collect();
    let total_sum:f64 = sums.iter().sum();

    let filtered = sums.iter().map(|s|*s).collect::<Vec<f64>>();

    //let point = Point::new(0.,0.,&uvex.fuv_psf.grid.coordinate_system());
    let point = uvex.fuv_psf.grid.locate(0);
    let nuv_psf = uvex.get_psfs(&point).1;

    let centers = [Point::new(-12.8*2.,0.,&uvex.psf_coords),
        Point::new(-12.8,0.,&uvex.psf_coords),
        Point::new(0.,0.,&uvex.psf_coords),
        Point::new(12.8,0.,&uvex.psf_coords),
        Point::new(12.8*2.,0.,&uvex.psf_coords)];
    //let weight = [1.,1.,1.,1.,1.];
    let weight = filtered.clone();



    let mut nuv_psfs:Vec<SpatialEffect> = nuv_psf
        .iter()
        .enumerate()
        .map(|(i,data)|{
            let PsfGrid = GRID2D::new(
                [128,128],
                [0.1,0.1],
                centers[i],
                0.0001,
                uvex.psf_coords,
            );
            let mut effect = SpatialEffect::new(PsfGrid, data.clone()).unwrap();
            effect.scale(weight[i]);
            effect }).collect();

   // println!("nuv psf {:?}",nuv_psfs);
    let mut detector = uvex.detector;
    for effect in nuv_psfs{
        //println!("adding effect");
        detector.add(&effect);

    }
    detector.write_to_fits(&format!("psfs_{temp}_filtered_binned.fits"));


    (unfiltered,filtered)



    //compare(input.apply_spectral_response(&mut bins[4]))

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

pub fn path(path:&str)->String{
    format!("src/test_outputs/{path}")
}



pub fn compare(input:Spectrum){


    let mut uvex = UVEX::load();
    let mut input = input.convert_to(&f_lambda);
    let mut triple_mirror = uvex.mirror_response.clone()
        .multiply(&self.mirror_response)
        .multiply(&self.mirror_response)
    let mut mirror_plus_input = input.clone().apply_spectral_response(&mut triple_mirror);



    input.write_to_dat("# input spectrum",&path("input"));
    uvex.mirror_response.write_to_dat("#1X mirror", &path("mirror"));
    triple_mirror.write_to_dat("#3X mirror", &path("triple_mirror"));
    mirror_plus_input.write_to_dat("#3X mirror + input", &path("mirror_plus_input"));


    uvex.dichroic_reflection.write_to_dat("#dichroic reflection",&path("reflection"));
    uvex.dichroic_transmission.write_to_dat("#dichroic transmission",&path("transmission"));


    let mut tot_ref = mirror_plus_input.clone().apply_spectral_response(&mut uvex.dichroic_reflection);
    let mut tot_trans = mirror_plus_input.apply_spectral_response(&mut uvex.dichroic_transmission);


    tot_ref.write_to_dat("#3X mirror + dichroic reflection + input",&path("mirror_ref"));
    tot_trans.write_to_dat("#3X mirror + dichroic transmission + input",&path("mirror_trans"));


    let mut nuv_filter_plus_qe = uvex.nuv_qe.multiply(&mut uvex.nuv_filter);
    nuv_filter_plus_qe.write_to_dat("#qe plus filter for nuv", &path("nuv_filter_plus_qe"));

    uvex.fuv_filter.write_to_dat("# fuv filter response", &path("fuv_filter"));
    uvex.nuv_filter.write_to_dat("# nuv filter response", &path("nuv_filter"));
    uvex.nuv_qe.write_to_dat("# nuv QE", &path("nuv_qe"));

    let mut tot_ref_plus_filter = tot_ref.apply_spectral_response(&mut uvex.fuv_filter);
    let mut tot_trans_plus_filter = tot_trans.apply_spectral_response(&mut uvex.nuv_filter)
        .apply_spectral_response(&mut uvex.nuv_qe);
    tot_ref_plus_filter.write_to_dat("#mirror ref fuv filter", &path("fuv"));
    tot_trans_plus_filter.write_to_dat("#mirror ref nuv filter nuv qe", &path("nuv"));


    uvex.fuv_contamination.write_to_dat("#fuv contamination", &path("fuv_con"));
    uvex.nuv_contamination.write_to_dat("#nuv contamination", &path("nuv_con"));


    let mut full_fuv = tot_ref_plus_filter.apply_spectral_response(&mut uvex.fuv_contamination);
    let mut full_nuv = tot_trans_plus_filter.apply_spectral_response(&mut uvex.nuv_contamination);
    full_fuv.write_to_dat("#full fuv", &path("full_fuv"));
    full_nuv.write_to_dat("#full nuv", &path("full_nuv"));
}


