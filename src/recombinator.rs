use rand::rngs::ThreadRng;

use crate::{population::Population, individual::Individual, selector::Selector};

pub mod uniform_xo;

pub trait Recombinator<P, S>
where
    P: Population,
    P::Individual: Individual,
    S: Selector<P>,
{
    fn recombine(
        &self,
        genome: &<P::Individual as Individual>::Genome,
        population: &P,
        selector: &S,
        rng: &mut ThreadRng,
    ) -> <P::Individual as Individual>::Genome;
}
