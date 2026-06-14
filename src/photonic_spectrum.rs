use rand_distr::Binomial;

pub fn electronify(&self, qe:f64) -> Electrons{
    let mut electron_numbers = Vec::with_capacity(self.values.len());
    for photon_number in &self.values{
        let binomial = Binomial::new(*photon_number as u64, qe).unwrap();
        electron_numbers.push(binomial.sample(&mut rand::rng()) as usize);
    }
    Electrons{
        values:electron_numbers
    }
}