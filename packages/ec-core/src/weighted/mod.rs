use error::{SelectionError, ZeroWeight};
use rand::Rng;
use with_weight::WithWeight;

use crate::{operator::selector::Selector, population::Population};

pub mod error;
pub mod weighted_pair;
pub mod with_weight;
pub mod with_weighted_item;

#[derive(Debug)]
pub struct Weighted<T> {
    item: T,
    weight: u32,
}

impl<T> Weighted<T> {
    pub const fn new(item: T, weight: u32) -> Self {
        Self { item, weight }
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
    type Error = SelectionError<T::Error>;

    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
    ) -> Result<&'pop <P as Population>::Individual, Self::Error> {
        if self.weight == 0 {
            return Err(ZeroWeight.into());
        }
        self.item
            .select(population, rng)
            .map_err(SelectionError::Selector)
    }
}
