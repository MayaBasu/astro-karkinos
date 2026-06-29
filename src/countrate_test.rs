use std::fs::File;
use std::io::{BufRead, BufReader};
use astroimsim_geometry::coordinate_system::Coordinates::ABSOLUTE;
use astroimsim_geometry::points::Point;
use plotpy::*;
use crate::point_sources::FullSpectrumPointSource;
use crate::uvex_telescope::UVEX;

pub fn test_countrates() {
    let benchmark_path = "src/tests/countrate_mag_list.dat";
    let mut plot = plotpy::Plot::new();
    let mut nuv_benchmarks = Curve::new();
    nuv_benchmarks.set_label("nuv bench");
    let mut nuv_results = Curve::new();
    nuv_results.set_marker_color("#eeea83")
        .set_label("nuv results")
        .set_marker_every(1)
        .set_marker_size(5.0)
        .set_line_style("none").set_marker_style("*");



    let mut fuv_benchmarks = Curve::new();
    fuv_benchmarks.set_label("fuv benchmarks");
    let mut fuv_results = Curve::new();
    fuv_results
        .set_label("fuv results")
        .set_marker_color("#eeea83")
        .set_marker_every(1)
        .set_marker_size(5.0)
        .set_line_style("none")
        .set_marker_style("*");

    let mut residual_fuv = Curve::new();
    residual_fuv.set_label("FUV percent error");
    let mut residual_nuv = Curve::new();
    residual_nuv.set_label("NUV percent error");



    let file = File::open(benchmark_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut data: Vec<Vec<f64>> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            continue
        };
        let line = line.expect("Failed to read line");
        let line:Vec<f64> = line.trim()
            .split(" ")
            .map(|a|
                { //println!("{:?}",a.trim());
                    a.trim().parse::<f64>()
                })
            .map(|result|
                match result {
                    Ok(value) => { //println!("{:?}",value);
                        value
                    },
                    Err(v) => {
                        panic!("COULDN:T PARSE {:?}", v);
                    },
                }).collect();
        data.push(line)
    };
    println!("{:?}",data);


    let mut fuv_bench = Vec::new();
    let mut nuv_bench = Vec::new();
    let mut magnitudes = Vec::new();
    let mut fuv_vals = Vec::new();
    let mut nuv_vals = Vec::new();
    let mut fuv_res = Vec::new();
    let mut nuv_res = Vec::new();
    let (fuv,nuv) = UVEX::fuv_nuv();
    for test_case in data{
        let mag = test_case[0];
        let bench_nuv = test_case[1];
        let bench_fuv = test_case[2];


        let mut point_source = FullSpectrumPointSource::flat_AB(Point::new(0.0, 0.0, ABSOLUTE), mag, UVEX::spectral_grid());

        let bands = point_source.to_bands(&fuv,&nuv,4410.0);
        let nuv = bands.nuv*0.76153773;
        let fuv= bands.fuv*0.76153773;


        fuv_bench.push(bench_fuv);
        nuv_bench.push(bench_nuv);
        magnitudes.push(mag);
        fuv_vals.push(fuv);
        nuv_vals.push(nuv);

        fuv_res.push(100.0*(fuv-bench_fuv)/bench_fuv);
        nuv_res.push(100.0*(nuv-bench_nuv)/bench_nuv);



        println!(" MAGNTIUDE IS {:?}",mag);
        println!("fuv Bench {:?} fuv REsult {:?}, \n nuv bench {:?}  nuv result{:?}", bench_fuv,fuv,bench_nuv, nuv)
    }

    nuv_benchmarks.points_begin();
    for i in 0..magnitudes.len(){
        nuv_benchmarks.points_add(magnitudes[i], nuv_bench[i]);
    }
    nuv_benchmarks.points_end();


    fuv_benchmarks.points_begin();
    for i in 0..magnitudes.len(){
        fuv_benchmarks.points_add(magnitudes[i], fuv_bench[i]);
    }
    fuv_benchmarks.points_end();

    fuv_results.points_begin();
    for i in 0..magnitudes.len(){
        fuv_results.points_add(magnitudes[i], fuv_vals[i]);
    }
    fuv_results.points_end();


    nuv_results.points_begin();
    for i in 0..magnitudes.len(){
        nuv_results.points_add(magnitudes[i], nuv_vals[i]);
    }
    nuv_results.points_end();


    residual_fuv.points_begin();
    for i in 0..magnitudes.len(){
        residual_fuv.points_add(magnitudes[i], fuv_res[i]);
    }
    residual_fuv.points_end();
    residual_nuv.points_begin();
    for i in 0..magnitudes.len(){
        residual_nuv.points_add(magnitudes[i], nuv_res[i]);
    }
    residual_nuv.points_end();

    //plot.add(&nuv_benchmarks);
    //plot.add(&fuv_benchmarks);
    //plot.add(&nuv_results);
    //plot.add(&fuv_results);
    plot.add(&residual_fuv);
    plot.add(&residual_nuv);
    plot.legend();
    plot.show("slkef").unwrap();





}