use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rand::distr::{Distribution, Uniform};
use serde::Serialize;
use astroimsim_geometry::grid2d::{Location, GRID2D};
use astroimsim_geometry::points::Point;
use crate::units::Units;
pub const spectral_resolution:usize = 1000;
#[derive( Clone, Debug)]
pub enum Spectrum{
    Full(f64,[f64;spectral_resolution], Units),
    Bands(Vec<Bands>, Units),

}
#[derive(Clone,Debug)]
pub enum Bands{
    FUV(f64),
    NUV(f64),
}

#[derive(Debug,Clone)]
pub struct PointSource {
    pub point: Point,
    pub spectrum: Spectrum,
}

impl PointSource {
    pub fn new_full(point:Point, spectrum: [f64;spectral_resolution],luminosity:f64, units: Units) -> PointSource {
        let spectrum = Spectrum::Full(luminosity,spectrum, units);
        PointSource {
            point,
            spectrum
        }
    }
    pub fn new_fuv_nuv(point:Point,fuv:f64,nuv:f64,units: Units ) -> PointSource{
        let spectrum = Spectrum::Bands(vec![Bands::NUV(nuv),Bands::FUV(fuv)],units);
        PointSource{
            point,
            spectrum
        }
    }
    pub fn new(point:Point, spectrum: Spectrum) -> PointSource{

        PointSource{
            point,
            spectrum,
        }
    }

    pub fn fake_spectrum()-> [f64;spectral_resolution]{
        let mut spectrum = [0.0;spectral_resolution];
        let luminosities = Uniform::new(0.0,1.0).expect("Could not generate random luminosities in the given range");
        let mut rng = rand::rng();
        for element in 0..spectrum.len(){
            spectrum[element] = 1.0  + luminosities.sample(&mut rng);
        }
        /*
        Spectrum::Full(1.0,spectrum)

         */
        spectrum
    }

    pub fn fake_bands(units: Units) -> Spectrum{
        let luminosities = Uniform::new(0.0,1.0).expect("Could not generate random luminosities in the given range");
        let mut rng = rand::rng();
        Spectrum::Bands(vec![Bands::FUV(luminosities.sample(&mut rng)),Bands::NUV(luminosities.sample(&mut rng))],units)

    }

    pub fn scale(&mut self,scale:f64){
        match &self.spectrum{
            Spectrum::Full(l, c,u) => {self.spectrum = Spectrum::Full(l*scale,*c,(*u).clone())}
            Spectrum::Bands(bands,u) => {
                self.spectrum = Spectrum::Bands(bands.iter().map(|band|
                    match band {
                        Bands::FUV(l) => { Bands::FUV(l * scale) }
                        Bands::NUV(l) => { Bands::FUV(l * scale) }
                    }).collect(),(*u).clone() )
            }
        };
    }



}

#[derive(Debug)]
pub struct SourceList {
    pub sources: Vec<PointSource>,
}
impl SourceList {
    pub fn new_from(mut sources:Vec<PointSource> ) -> SourceList {
        //sources.sort_by(|a:&point_source, b:&point_source| b.bin.cmp(&a.bin));
        SourceList {
            sources,
        }
    }
    pub fn new_empty(capacity:usize) -> SourceList {
        SourceList {
            sources: Vec::with_capacity(capacity)
        }
    }
    pub fn add_source(&mut self, source: PointSource) -> &mut SourceList {
        self.sources.push(source);
        self

    }
    pub fn random_full_spectrum_point_source_field(number_of_point_sources:usize,
                                                   min_brightness: f64,
                                                   max_brightness: f64,
                                                   grid: &GRID2D,
                                                   units: Units
    ) -> SourceList {
        //Some checks to make sure that the incoming values are as expected
        //TODO Fix these checks to work with f64 values
        /*
        for end_point in [min_brightness,max_brightness,min_x,max_x,min_y,max_y]{
            assert!((0.0 <= end_point) || (end_point <= 1.0),"{}: {} must be a float between 0 and 1",stringify!(end_point),end_point );
        }
        assert!(min_brightness <= max_brightness,"min_brightness must be less than or equal to max_brightness");
        assert!(min_x <= max_x,"min_x must be less than or equal to max_x");
        assert!(min_y <= max_y,"min_y must be less than or equal to max_y");

         */

        let luminosities = Uniform::new(min_brightness,max_brightness).expect("Could not generate random luminosities in the given range");
        let mut rng = rand::rng();
        let sources: Vec<PointSource> = (0..number_of_point_sources).map(|_x|{
            let luminosity = luminosities.sample(&mut rng);
            let point = grid.random();
            // println!("Point is {:?}",point);
            let spectrum = PointSource::fake_spectrum();
            PointSource::new_full(point,spectrum,luminosity,units.clone())
        }).collect();
        SourceList::new_from(sources)
    }
    /*
    pub fn write_to_yaml(&self, file_name:&str,) {
        println!("Serializing point sources");
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the sources");
        let mut file = File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }

     */

    pub fn random_bands_point_source_field(number_of_point_sources:usize,
                                           min_brightness: f64,
                                           max_brightness: f64,
                                           units: Units,
                                           grid: &GRID2D,
    ) -> SourceList {
        //Some checks to make sure that the incoming values are as expected
        //TODO Fix these checks to work with f64 values
        /*
        for end_point in [min_brightness,max_brightness,min_x,max_x,min_y,max_y]{
            assert!((0.0 <= end_point) || (end_point <= 1.0),"{}: {} must be a float between 0 and 1",stringify!(end_point),end_point );
        }
        assert!(min_brightness <= max_brightness,"min_brightness must be less than or equal to max_brightness");
        assert!(min_x <= max_x,"min_x must be less than or equal to max_x");
        assert!(min_y <= max_y,"min_y must be less than or equal to max_y");

         */

        let luminosities = Uniform::new(min_brightness,max_brightness).expect("Could not generate random luminosities in the given range");
        let mut rng = rand::rng();
        let sources: Vec<PointSource> = (0..number_of_point_sources).map(|_x|{
            let point = grid.random();
            let spectrum = PointSource::fake_bands(units.clone());
            PointSource { point, spectrum }
        }).collect();
        SourceList::new_from(sources)
    }
    /*
    pub fn write_to_yaml(&self, file_name:&str,) {
        println!("Serializing point sources");
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the sources");
        let mut file = File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }

     */

    /*

    pub fn apply_flatfield_illumination(&mut self,illumination:flatfield){
        let start = Instant::now();
        for source in &mut self.sources{
            let scale = match illumination.grid.inside_or_outside(&source.point){
                Location::Outside => {continue}
                Location::Inside => {
                    let (x_mod,y_mod,_x_residual,_y_residual) = illumination.grid.fit_grid(&source.point);
                    illumination.data[y_mod][x_mod]
                }
            };
            // println!("Scle is {scale}");

            source.scale(scale)
        }
        println!("Applied flatfield illumination to {:?} sources in {:?}ms",self.sources.len(),start.elapsed().as_millis())
    }

     */


}

