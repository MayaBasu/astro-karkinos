use crate::telescope::PsfGrid;
use crate::uvex::config::Config;
use crate::uvex::uvex::UVEX;

pub fn run_oversample_demo(){

    let mut config = Config::default();
    //config.nuv_flatfield_illumination.0 = off;
    let mut uvex = UVEX::load(config.clone());

    let band = 2;
    let psf = 30;
    let patch_size = 15;
    println!("Using band {band}, {:?} - {:?}", config.nuv_bands[band], config.nuv_bands[band+1]);
    let psf_grid:PsfGrid = uvex.nuv_psfs[band].clone();
    for i in -5isize..5{
        for j in -5isize..5{
            println!("i,j = {i},{j}");

            let regridded_psf_grid = psf_grid.clone().regrid_psfs_grided_to_subpixel(10,patch_size,i,j);

            let psf = regridded_psf_grid.psf(psf);
            psf.write_file(&format!("/Users/mayabasu/Desktop/subpixel_outputs/psf_band_{band}_{i}_{j}.fits"), ("NA".to_string(),"NA".to_string()))

        }

    }




}