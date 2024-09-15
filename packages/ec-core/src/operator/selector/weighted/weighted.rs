use super::WithWeight;
use crate::{operator::selector::Selector, population::Population};

#[derive(Debug)]
pub struct Weighted<T> {
    pub(crate) selector: T,
    pub(crate) weight: u32,
}

impl<T> Weighted<T> {
    pub const fn new(selector: T, weight: u32) -> Self {
        Self { selector, weight }
    }
}

impl<T> WithWeight for Weighted<T> {
    fn weight(&self) -> u32 {
        self.weight
    }
}

impl<P, T> Selector<P> for Weighted<T>
where
    P: Population,
    T: Selector<P>,
{
    type Error = T::Error;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Result<&'pop <P as Population>::Individual, Self::Error> {
        self.selector.select(population, rng)
    }
}