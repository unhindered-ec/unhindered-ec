use rand::{rngs::ThreadRng, Rng};

use crate::{individual::Individual, population::Population, selector::Selector};

use super::Recombinator;

pub struct TwoPointXo;

impl<P, S, T> Recombinator<P, S> for TwoPointXo
where
    P: Population,
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
