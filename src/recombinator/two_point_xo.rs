use rand::{rngs::ThreadRng, Rng};

use super::Recombinator;
use crate::operator::{Composable, Operator};

pub struct TwoPointXo;

impl<T> Recombinator<Vec<T>> for TwoPointXo
where
    T: Clone,
{
    fn recombine(&self, genomes: &[&Vec<T>], rng: &mut ThreadRng) -> Vec<T> {
        let genome = genomes[0].clone();
        let second_genome = genomes[1].clone();

        self.apply([genome, second_genome], rng)
    }
}

impl<T> Operator<[Vec<T>; 2]> for TwoPointXo
where
    T: Clone,
{
    type Output = Vec<T>;

    fn apply(
        &self,
        [mut first_genome, mut second_genome]: [Vec<T>; 2],
        rng: &mut ThreadRng,
    ) -> Self::Output {
        assert_eq!(first_genome.len(), second_genome.len());
        let len = first_genome.len();

        let mut first = rng.gen_range(0..len);
        let mut second = rng.gen_range(0..len);
        if second < first {
            (first, second) = (second, first);
        }
        // We now know that first <= second
        first_genome[first..second].swap_with_slice(&mut second_genome[first..second]);
        first_genome
    }
}
impl Composable for TwoPointXo {}
