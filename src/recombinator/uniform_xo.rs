use rand::{rngs::ThreadRng, Rng};

use super::Recombinator;

pub struct UniformXo;

impl<T> Recombinator<2, Vec<T>> for UniformXo
where
    T: Clone,
{
    fn recombine(&self, genomes: [&Vec<T>; 2], rng: &mut ThreadRng) -> Vec<T> {
        let genome = genomes[0];
        let second_genome = genomes[1];
        assert_eq!(genome.len(), second_genome.len());
        let len = genome.len();
        (0..len)
            .map(|pos| {
                if rng.gen_bool(0.5) {
                    genome[pos].clone()
                } else {
                    second_genome[pos].clone()
                }
            })
            .collect()
    }
}
