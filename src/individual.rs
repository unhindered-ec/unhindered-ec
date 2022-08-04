use rand::Rng;
use rand::rngs::ThreadRng;

pub fn count_ones(bits: &[bool]) -> f64 {
    bits.iter().filter(|&&bit| bit).count() as f64
}

#[derive(Debug)]
pub struct Individual {
    pub bits: Vec<bool>,
    pub fitness: f64,
}

impl Individual {
    pub fn new(bit_length: usize, rng: &mut ThreadRng) -> Individual {
        let mut bits = Vec::with_capacity(bit_length);
        for _ in 0..bit_length {
            bits.push(rng.gen_bool(0.5));
        }
        let fitness = count_ones(&bits);
        Individual {
            bits,
            fitness,
        }
    }
}
