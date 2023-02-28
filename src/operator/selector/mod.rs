use rand::rngs::ThreadRng;

use crate::population::Population;

use super::{Composable, Operator};

pub mod best;
pub mod lexicase;
pub mod random;
pub mod tournament;
pub mod weighted;

pub trait Selector<P>
where
    P: Population,
{
    fn select<'pop>(&self, population: &'pop P, rng: &mut ThreadRng) -> &'pop P::Individual;
}

pub struct Select<S> {
    selector: S,
}

impl<S> Select<S> {
    pub const fn new(selector: S) -> Self {
        Self { selector }
    }
}

impl<'pop, P, S> Operator<&'pop P> for Select<S>
where
    P: Population,
    S: Selector<P>,
{
    type Output = &'pop P::Individual;

    fn apply(&self, population: &'pop P, rng: &mut ThreadRng) -> Self::Output {
        self.selector.select(population, rng)
    }
}
impl<S> Composable for Select<S> {}
