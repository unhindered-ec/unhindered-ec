use rand::{rngs::ThreadRng, Rng};

use crate::{population::Population, individual::Individual, selector::Selector};

use super::Recombinator;

pub struct UniformXO;

impl<P, S, T> Recombinator<P, S> for UniformXO
where
    P: Population,
    // TODO: Should `Vec<T>` be replaced with a trait?
    P::Individual: Individual<Genome = Vec<T>>,
    S: Selector<P>,
    T: Clone,
{
    fn recombine(
        &self,
        genome: &Vec<T>,
        population: &P,
        selector: &S,
        rng: &mut ThreadRng,
    ) -> Vec<T> {
        let second_parent = selector.select(rng, population);
        let second_genome = second_parent.genome();
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