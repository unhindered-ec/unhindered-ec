use std::ops::Not;

use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

use crate::operator::{Composable, Operator};
use crate::population::Population;

pub struct Random;

impl<'pop, P> Operator<&'pop P> for Random
where
    P: Population + AsRef<[P::Individual]>,
{
    type Output = &'pop P::Individual;

    fn apply(&self, population: &'pop P, rng: &mut ThreadRng) -> Self::Output {
        // The population should never be empty here.
        assert!(
            population.is_empty().not(),
            "The population should not be empty"
        );
        #[allow(clippy::unwrap_used)]
        population.as_ref().choose(rng).unwrap()
    }
}
impl Composable for Random {}
