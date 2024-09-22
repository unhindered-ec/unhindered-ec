use crate::{
    operator::selector::{weighted::WithWeight, Selector},
    population::Population,
};

#[derive(Debug)]
pub struct Weighted<T> {
    pub(crate) item: T,
    pub(crate) weight: u32,
}

impl<T> Weighted<T> {
    pub const fn new(selector: T, weight: u32) -> Self {
        Self {
            item: selector,
            weight,
        }
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
        self.item.select(population, rng)
    }
}
