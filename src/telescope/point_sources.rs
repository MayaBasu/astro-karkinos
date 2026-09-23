use crate::telescope::power_spectrum::*;
use crate::telescope::spectral_response::*;
use std::time::Instant;

use rand::distr::{Distribution, Uniform};
use crate::geometry::*;


use rand_distr::Poisson;
use self::SpectrumUnits::*;
use crate::telescope::point_sources::BandUnits::{AverageElectronFlux, Electrons};
use crate::telescope::spatial_effect::SpatialEffect;

#[derive(Debug,Clone)]
pub struct FullSpectrumPointSource {
    pub point: Point,
    pub spectrum: PowerSpectrum,
    pub scale: f64,
}

#[derive(Debug)]
pub struct BandPasses {
    labels: Vec<String>,
    band_passes: Vec<>
    
}

#[derive(Debug)]
pub struct Bands {
    pub fuv: f64,
    pub nuv: f64,
    pub units: BandUnits,
}

#[derive(Debug)]
pub enum BandUnits{
    AverageElectronFlux,
    Electrons,
}

impl FullSpectrumPointSource {
    pub fn flat_AB(point:Point,ab_mag:f64,grid1d: GRID1D)->FullSpectrumPointSource{
        let mut spectrum = PowerSpectrum::flat_AB(ab_mag,grid1d,format!("flat ab {ab_mag}").to_string());
            spectrum.convert_to(&SpectrumUnits::f_lambda);
        //println!("spectrum is {:?}",spectrum);
        FullSpectrumPointSource{point,spectrum,scale:1.0}
    }
    pub fn new_from_spectrum(point: Point,spectrum:PowerSpectrum)->FullSpectrumPointSource{
        FullSpectrumPointSource{point,spectrum,scale:1.0}
    }

    pub fn scale(&mut self,scale:f64){
        self.scale *= scale
    }

    pub fn apply_spectral_response_curve(&mut self, curve:&SpectralResponseCurve){
        self.spectrum.convert_to(&f_lambda);
        self.spectrum.apply_spectral_response(curve);
    }
    pub fn to_bands(&self,fuv_path:&SpectralResponseCurve,nuv_path:&SpectralResponseCurve,area:f64)->Bands{
        let mut fuv_spectrum = self.clone();
        let mut nuv_spectrum = self.clone();
        fuv_spectrum.apply_spectral_response_curve(fuv_path);
        nuv_spectrum.apply_spectral_response_curve(nuv_path);
        fuv_spectrum.spectrum.write_to_dat("fuv_spectrum", "fuv spectrum ");
        nuv_spectrum.spectrum.write_to_dat("nuv_spectrum", "nuv spectrum");
        let fuv = fuv_spectrum.spectrum.total_average_photon_flux(area);
        let nuv = nuv_spectrum.spectrum.total_average_photon_flux(area);
        Bands{fuv,nuv,units:AverageElectronFlux}
    }
}
#[derive(Debug)]
pub struct FullSpectrumSourceList {
    pub sources: Vec<FullSpectrumPointSource>,
}
impl FullSpectrumSourceList {
    pub fn new_from(mut sources:Vec<FullSpectrumPointSource> ) -> FullSpectrumSourceList {
        //sources.sort_by(|a:&point_source, b:&point_source| b.bin.cmp(&a.bin));
        FullSpectrumSourceList {
            sources,
        }
    }
    pub fn new_empty(capacity:usize) -> FullSpectrumSourceList {
        FullSpectrumSourceList {
            sources: Vec::with_capacity(capacity)
        }
    }
    pub fn add_source(&mut self, source: FullSpectrumPointSource) -> &mut FullSpectrumSourceList {
        self.sources.push(source);
        self

    }
    pub fn full_spectrum_point_source_field(number_of_point_sources:usize,
                                           min_brightness: f64,
                                           max_brightness: f64,
                                            spectrum:PowerSpectrum,
                                            grid: &GRID2D,
    ) -> FullSpectrumSourceList {
        assert!(max_brightness>=min_brightness,"max brightness must be greater or equal to min brightness");
        let luminosities = Uniform::new(min_brightness,max_brightness).expect("Could not generate random luminosities in the given range");
        let mut rng = rand::rng();
        let sources: Vec<FullSpectrumPointSource> = (0..number_of_point_sources).map(|_x|{
            let scale = luminosities.sample(&mut rng);
            let point = grid.random();
            FullSpectrumPointSource { point,spectrum: spectrum.clone(), scale}
        }).collect();
        FullSpectrumSourceList::new_from(sources)
    }

    pub fn apply_spatial_effect(&mut self,effect:SpatialEffect){
        let start = Instant::now();
        for source in &mut self.sources{
            if effect.grid.is_point_inside(&source.point){
                let scale = effect.get_data(&source.point);
                source.scale(scale)
            }else{
                continue
            };
        }
        println!("Applied spatial effect {:?} to {:?} sources in {:?} ms",effect.label,self.sources.len(),start.elapsed().as_millis())
    }
}


impl Bands{
    pub fn poisson(&mut self){
        let mut rng = rand::rng();
        let fuv_poisson = Poisson::new(self.fuv as f64).unwrap();
        let nuv_poisson = Poisson::new(self.nuv as f64).unwrap();
        let fuv_electrons = fuv_poisson.sample(&mut rng);
        let nuv_electrons = nuv_poisson.sample(&mut rng);
        self.units = Electrons;
        self.nuv = nuv_electrons;
        self.fuv = fuv_electrons;
    }
}
