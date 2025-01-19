use error::{SelectionError, ZeroWeight};
use rand::Rng;
use with_weight::WithWeight;

use crate::{operator::selector::Selector, population::Population};

pub mod error;
pub mod weighted_pair;
pub mod with_weight;
pub mod with_weighted_item;

/// Wrapper type to annotate a `T` with a weight.
///
/// The important [implementation](Weighted::weight) on this type is probably
/// the [`WithWeight`] trait.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Weighted<T> {
    item: T,
    weight: u32,
}

/// Create a new weighted `T` using its Default implementation and with a
/// default Weight of 1.
impl<T> Default for Weighted<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default(), 1)
    }
}

impl<T> Weighted<T> {
    /// Create a new weighted `T` with the given weight
    pub const fn new(item: T, weight: u32) -> Self {
        Self { item, weight }
    }
}

impl<T> WithWeight for Weighted<T> {
    /// Weight that the given `T` is annotated with
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

    /// Select from this weighted type, if the weight is not 0
    ///
    /// # Errors
    /// - [`SelectionError::Selector`] if the underlying selector errors
    /// - [`SelectionError::ZeroWeight`] if the weight is 0 and as such this
    ///   selector should not be selected from
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
