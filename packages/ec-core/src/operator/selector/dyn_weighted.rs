use std::error::Error;

use miette::Diagnostic;
use rand::{
    Rng,
    seq::{IndexedRandom, WeightError},
};

use super::{DynSelector, Selector};
use crate::population::Population;

/// Weighted selector, based on type-erased selectors and dynamic dispatch.
///
/// Also see the [`WeightedPair`](crate::weighted::weighted_pair::WeightedPair)
/// selector for a compile time (typed) version of this which is not using
/// dynamic dispatch, which is the recommended way to do this when possible.
///
/// Each selector has an associated weight passed in at construction time and
/// one selector will be selected based on that on each selection call, and
/// actual selection will then be forwarded to that selector.
///
/// # Example
/// ```
/// # use ec_core::operator::selector::{best::Best, worst::Worst, dyn_weighted::{DynWeighted, DynWeightedError}, Selector};
/// # use rand::rng;
/// #
/// type Population = Vec<i32>;
///
/// let dyn_selector = DynWeighted::<Population>::new(Best, 1).with_selector(Worst, 1);
///
/// let population = vec![1,2,3];
/// let selected = dyn_selector.select(&population, &mut rng())?;
/// assert!(selected == &3 || selected == &1);
/// # Ok::<(), DynWeightedError>(())
/// ```
#[derive(Default)]
pub struct DynWeighted<P: Population> {
    selectors: Vec<(Box<dyn DynSelector<P> + Send + Sync>, usize)>,
}

/// Debugs the `DynWeighted` selector.
///
/// Note that this has no way to actually debug the underlying selector, since
/// we don't require a debug bound in the dyn selector trait. As such this only
/// debugs the count of selectors as well as their weights.
impl<P: Population> std::fmt::Debug for DynWeighted<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynWeighted")
            .field("selector_count", &self.selectors.len())
            .field(
                "selector_weights",
                &self.selectors.iter().map(|(_, x)| x).collect::<Vec<_>>(),
            )
            .finish_non_exhaustive()
    }
}

impl<P, S> FromIterator<(S, usize)> for DynWeighted<P>
where
    P: Population,
    S: Selector<P, Error: Error + Send + Sync + 'static> + Send + Sync + 'static,
{
    /// Construct a new [`DynWeighted`] selector from an iterator over tuples of
    /// selectors and associated weights
    ///
    /// Usually you probably want to use the chaining construction method
    /// `DynWeighted::new(...).with_selector(...)..` instead since the purpose
    /// of [`DynWeighted`] is usually to hold a heterogeneous collection of
    /// selectors, rather than a homogeneous like iterators support.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{dyn_weighted::DynWeighted, best::Best};
    /// # type Population = Vec<i32>;
    /// let dyn_selector = [(Best, 1)].into_iter().collect::<DynWeighted<Population>>();
    /// # let _ = dyn_selector;
    /// ```
    fn from_iter<T: IntoIterator<Item = (S, usize)>>(iter: T) -> Self {
        Self {
            selectors: iter
                .into_iter()
                .map(
                    |(selector, weight)| -> (Box<dyn DynSelector<P> + Send + Sync>, _) {
                        (Box::new(selector), weight)
                    },
                )
                .collect(),
        }
    }
}

/// Error that can occur when trying to select from a [`DynWeighted`]:
///
/// - [`WeightError`] if the weight's of all the selectors add up to less than
///   1, or
/// - `Box<dyn Error + Send + Sync>` (boxed) error of an underlying selector if
///   the selection fails there.
#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum DynWeightedError {
    #[error(transparent)]
    #[diagnostic(help = "Ensure that the weights are all non-negative and add to more than zero")]
    ZeroWeightSum(#[from] WeightError),

    #[error(transparent)]
    Other(Box<dyn Error + Send + Sync>),
}

impl<P: Population> DynWeighted<P> {
    // Since we should never have an empty collection of weighted selectors,
    // the `new` implementation takes an initial selector so `selectors` is
    // guaranteed to never be empty.
    /// Construct a new `DynWeighted` selector, by using an intial selector and
    /// a weight
    ///
    /// Use [`DynWeighted::with_selector`] as a chain on this to add more
    /// selectors afterwards
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{best::Best, dyn_weighted::DynWeighted};
    /// # type Population = Vec<i32>;
    /// let dyn_selector = DynWeighted::<Population>::new(Best, 1);
    /// # let _ = dyn_selector;
    /// ```
    #[must_use]
    pub fn new<S>(selector: S, weight: usize) -> Self
    where
        S: Selector<P, Error: Error + Send + Sync + 'static> + Send + Sync + 'static,
    {
        Self {
            selectors: vec![(Box::new(selector), weight)],
        }
    }

    /// Add the given selector to the [`DynWeighted`] selector with the given
    /// weight.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{best::Best, worst::Worst, dyn_weighted::DynWeighted};
    /// # type Population = Vec<i32>;
    /// let dyn_selector = DynWeighted::<Population>::new(Best, 1).with_selector(Worst, 1);
    /// # let _ = dyn_selector;
    /// ```
    #[must_use]
    pub fn with_selector<S>(mut self, selector: S, weight: usize) -> Self
    where
        S: Selector<P, Error: Error + Send + Sync + 'static> + Send + Sync + 'static,
    {
        self.selectors.push((Box::new(selector), weight));
        self
    }
}

impl<P> Selector<P> for DynWeighted<P>
where
    P: Population,
{
    type Error = DynWeightedError;

    /// Select an individual from the given `Population` using this selector.
    ///
    /// This will randomly choose one selector from the options according to the
    /// weights, and then run that selector on the [`Population`]
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{best::Best, worst::Worst, dyn_weighted::{DynWeighted, DynWeightedError}, Selector};
    /// # use rand::rng;
    /// #
    /// type Population = Vec<i32>;
    ///
    /// let dyn_selector = DynWeighted::<Population>::new(Best, 1).with_selector(Worst, 1);
    ///
    /// let population = vec![1,2,3];
    /// let selected = dyn_selector.select(&population, &mut rng())?;
    /// assert!(selected == &3 || selected == &1);
    /// # Ok::<(), DynWeightedError>(())
    /// ```
    ///
    /// # Errors
    /// - [`DynWeightedError::ZeroWeightSum`] if the weight sum of all selectors
    ///   is less than 1
    /// - [`DynWeightedError::Other`] if the selected selector errors
    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
    ) -> Result<&'pop P::Individual, Self::Error> {
        let (selector, _) = self.selectors.choose_weighted(rng, |(_, w)| *w)?;
        selector
            .select(population, rng)
            .map_err(DynWeightedError::Other)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use test_strategy::proptest;

    use super::DynWeighted;
    use crate::operator::selector::{Selector, best::Best, worst::Worst};

    #[proptest]
    fn best_or_worst(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = DynWeighted::new(Best, 1).with_selector(Worst, 1);
        let selection = weighted.select(&pop, &mut rng).unwrap();
        let extremes: [&i32; 2] = pop.iter().minmax().into_option().unwrap().into();
        assert!(extremes.contains(&selection));
    }
}
