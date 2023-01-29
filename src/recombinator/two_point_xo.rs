use rand::{rngs::ThreadRng, Rng};

use super::Recombinator;

pub struct TwoPointXo;

impl<T> Recombinator<Vec<T>> for TwoPointXo
where
    T: Clone,
{
    fn recombine(&self, genomes: &[&Vec<T>], rng: &mut ThreadRng) -> Vec<T> {
        let genome = genomes[0];
        let second_genome = genomes[1];
        assert_eq!(genome.len(), second_genome.len());
        let len = genome.len();

        let mut first = rng.gen_range(0..len);
        let mut second = rng.gen_range(0..len);
        if second < first {
            (first, second) = (second, first);
        }
        // We now know that first <= second
        let mut genome = genome.clone();
        genome[first..second].clone_from_slice(&second_genome[first..second]);
        genome
    }
}
