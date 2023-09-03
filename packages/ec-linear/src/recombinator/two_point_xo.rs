use anyhow::{ensure, Result};
use rand::{rngs::ThreadRng, Rng};

use ec_core::operator::recombinator::Recombinator;

use super::crossover::Crossover;

pub struct TwoPointXo;

// TODO: Remove the `Vec<T>` versions when we're done migrating
//   to the struct-based version of `Bitstring`.
impl<T> Recombinator<[Vec<T>; 2]> for TwoPointXo {
    type Output = Vec<T>;

    fn recombine(
        &self,
        [mut first_genome, mut second_genome]: [Vec<T>; 2],
        rng: &mut ThreadRng,
    ) -> Result<Self::Output> {
        ensure!(
            first_genome.len() == second_genome.len(),
            "Attempted to perform TwoPointXo on genomes of different lengths {} and {}",
            first_genome.len(),
            second_genome.len()
        );
        let len = first_genome.len();

        let mut first = rng.gen_range(0..len);
        let mut second = rng.gen_range(0..len);
        if second < first {
            (first, second) = (second, first);
        }
        // We now know that first <= second
        first_genome[first..second].swap_with_slice(&mut second_genome[first..second]);
        Ok(first_genome)
    }
}

impl<T> Recombinator<(Vec<T>, Vec<T>)> for TwoPointXo {
    type Output = Vec<T>;

    fn recombine(&self, genomes: (Vec<T>, Vec<T>), rng: &mut ThreadRng) -> Result<Self::Output> {
        self.recombine(<[Vec<T>; 2]>::from(genomes), rng)
    }
}

// TODO: Note that `TwoPointXo` doesn't strictly need
//   the two vectors to have the same length, but the
//   swapped regions need to "make sense" for both parent
//   genomes. So we could just make sure that both
//   `first` and `second` are < the minimum length
//   and we'd be OK. I'm not sure that's really how we'd
//   want to implement `TwoPointXo` for differing lengths,
//   though. I suspect it would make more sense to ensure
//   that the length of the swapped region is less than the
//   the length of the shorter genome, but not require that
//   they line up. That's really sounding like a different
//   operator than this one, though.
impl<G> Recombinator<[G; 2]> for TwoPointXo
where
    G: Crossover,
{
    type Output = G;

    fn recombine(
        &self,
        [mut first_genome, mut second_genome]: [G; 2],
        rng: &mut ThreadRng,
    ) -> Result<Self::Output> {
        ensure!(
            first_genome.size() == second_genome.size(),
            "Attempted to perform TwoPointXo on genomes of different lengths {} and {}",
            first_genome.size(),
            second_genome.size()
        );
        let len = first_genome.size();
        let mut first = rng.gen_range(0..len);
        let mut second = rng.gen_range(0..len);
        if second < first {
            (first, second) = (second, first);
        }
        first_genome.crossover_segment(&mut second_genome, first..second)?;

        Ok(first_genome)
    }
}

impl<G> Recombinator<(G, G)> for TwoPointXo
where
    G: Crossover,
{
    type Output = G;

    fn recombine(&self, genomes: (G, G), rng: &mut ThreadRng) -> Result<Self::Output> {
        self.recombine(<[G; 2]>::from(genomes), rng)
    }
}
