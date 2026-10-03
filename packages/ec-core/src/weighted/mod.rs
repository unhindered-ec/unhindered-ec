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
///
/// # Example
/// ```
/// # use ec_core::weighted::Weighted;
/// # use ec_core::operator::selector::best::Best;
/// #
/// let my_weighted = Weighted::new(Best, 10);
/// # let _ = my_weighted;
/// ```
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
    ///
    /// # Example
    /// ```
    /// # use ec_core::weighted::Weighted;
    /// # use ec_core::operator::selector::best::Best;
    /// #
    /// let my_weighted = Weighted::new(Best, 10);
    /// # let _ = my_weighted;
    /// ```
    pub const fn new(item: T, weight: u32) -> Self {
        Self { item, weight }
    }
}

impl<T> WithWeight for Weighted<T> {
    /// Weight that the given `T` is annotated with
    ///
    /// # Example
    /// ```
    /// # use ec_core::weighted::{Weighted, with_weight::WithWeight};
    /// # use ec_core::operator::selector::best::Best;
    /// #
    /// let my_weighted = Weighted::new(Best, 10);
    ///
    /// let weight = my_weighted.weight();
    /// assert_eq!(weight, 10);
    /// ```
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
    /// This basically directly forwards to the underlying types selector
    /// implementation and is meant to be used in compositions of
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     weighted::{Weighted, error::SelectionError},
    /// #     operator::selector::{
    /// #         best::Best,
    /// #         error::EmptyPopulation,
    /// #         Selector
    /// #     }
    /// # };
    /// # use rand::rng;
    /// #
    /// let my_weighted = Weighted::new(Best, 10);
    ///
    /// let selected = my_weighted.select(&[10, 1], &mut rng())?;
    /// assert_eq!(selected, &10);
    /// # Ok::<(), SelectionError<EmptyPopulation>>(())
    /// ```
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
