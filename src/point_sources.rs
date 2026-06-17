use std::fs::File;
use std::io::Write;
use std::time::Instant;
use astroimsim_geometry::grid1d::GRID1D;
use astroimsim_geometry::grid2d::{Location, GRID2D};
use astroimsim_geometry::points::Point;
use rand::distr::{Distribution, Uniform};
use astroimsim_spectra::power_spectrum::PowerSpectrum;
use crate::spatial_effect::SpatialEffect;

#[derive(Debug,Clone)]
pub struct FullSpectrumPointSource {
    pub point: Point,
    pub spectrum: PowerSpectrum,
    pub scale: f64,
}

impl FullSpectrumPointSource {
    pub fn flat_AB(point:Point,ab_mag:f64,grid1d: GRID1D)->FullSpectrumPointSource{
        let spectrum = PowerSpectrum::flat_AB(ab_mag,grid1d," ");
        FullSpectrumPointSource{point,spectrum,scale:1.0}
    }
    pub fn new_from_spectrum(point: Point,spectrum:PowerSpectrum)->FullSpectrumPointSource{
        FullSpectrumPointSource{point,spectrum,scale:1.0}
    }

    pub fn scale(&mut self,scale:f64){
        self.scale *= scale
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
            let scale = match effect.grid.inside_or_outside(&source.point){
                Location::Outside => continue,
                Location::Inside => effect.get_data(&source.point)
            };
            source.scale(scale)
        }
        println!("Applied spatial effect {:?} to {:?} sources in {:?} ms",effect.label,self.sources.len(),start.elapsed().as_millis())
    }


}