use ec_core::operator::recombinator::Recombinator;
use rand::{Rng, seq::IteratorRandom};

use super::{
    crossover::Crossover,
    errors::{CrossoverGeneError, DifferentGenomeLength},
};
use crate::genome::Linear;

pub struct TwoPointXo;

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
    G: Crossover + Linear,
{
    type Output = G;
    type Error = CrossoverGeneError<G::SegmentCrossoverError>;

    fn recombine<R: Rng + ?Sized>(
        &self,
        [mut first_genome, mut second_genome]: [G; 2],
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        let len = first_genome.size();
        if len != second_genome.size() {
            return Err(DifferentGenomeLength(len, second_genome.size()).into());
        }

        let mut first = rng.random_range(0..len);
        let mut second = rng.random_range(0..len);
        if second < first {
            (first, second) = (second, first);
        }

        // let mut crossover_points = [0; 2];
        // (1..len).choose_multiple_fill(rng, &mut crossover_points);
        // // crossover_points.sort_unstable();
        // let [mut first, mut second] = crossover_points;
        // if second < first {
        //     (first, second) = (second, first);
        // }

        first_genome
            .crossover_segment(&mut second_genome, first..second)
            .map_err(CrossoverGeneError::Crossover)?;

        Ok(first_genome)
    }
}

impl<G> Recombinator<(G, G)> for TwoPointXo
where
    G: Crossover + Linear,
{
    type Output = G;
    type Error = <Self as Recombinator<[G; 2]>>::Error;

    fn recombine<R: Rng + ?Sized>(
        &self,
        genomes: (G, G),
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        self.recombine(<[G; 2]>::from(genomes), rng)
    }
}
