use rand::distr::Uniform;
use rand::prelude::IteratorRandom;
use rand_distr::{Binomial, Distribution, Poisson};


pub fn test_binom(){
    let num = 4000*4000*9;
    let mut vals = Vec::with_capacity(num);

    let max_flux = 2000*400*2000*60*60*24*7;
    let min_flux = max_flux/2;
    println!("Binomial Distribution");
    println!("Assuming photon count is random and between {min_flux} and {max_flux} photons...");
    let now = std::time::Instant::now();
    let photons = Uniform::new(min_flux,max_flux).unwrap();
    let mut rng = rand::rng();
    let mut rng2 = rand::rng();
    for i in 0..num{
        let flux =photons.sample(&mut rng);
        let bin = Binomial::new(flux, 0.3).unwrap();
        let v = bin.sample(&mut rng2);
        vals.push(v)
    }

    let sum:u64 = vals.iter().sum();
    let duration = now.elapsed().as_secs();
    println!("Calculated values for 4k*4k*9 = {:?} in {:?}s, sum is {:?}",num,duration, sum);



    let num = 4000*4000*9;
    let mut vals = Vec::with_capacity(num);

    let max_flux:u64 = 2000*400*2000*60*60*24*7;
    let min_flux = max_flux/2;

    println!("Poisson Distribution");
    println!("Assuming photon count is random and between {min_flux} and {max_flux} photons...");
    let now = std::time::Instant::now();
    let photons = Uniform::new(min_flux,max_flux).unwrap();
    let mut rng = rand::rng();
    let mut rng2 = rand::rng();
    for i in 0..num{
        let flux =photons.sample(&mut rng);
        let bin = Poisson::new(flux as f64).unwrap();
        let v = bin.sample(&mut rng2);
        vals.push(v)
    }

    let sum:f64 = vals.iter().sum();
    let duration = now.elapsed().as_secs();
    println!("Calculated values for 4k*4k*9 = {:?} in {:?}s, sum is {:?}",num,duration, sum);




}