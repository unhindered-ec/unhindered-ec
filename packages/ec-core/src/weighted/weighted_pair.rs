use rand::{
    Rng,
    distr::{Bernoulli, Distribution},
};

use super::{
    error::{SelectionError, WeightSumOverflow, WeightedPairError, ZeroWeight},
    with_weight::WithWeight,
};
use crate::{operator::selector::Selector, population::Population};

/// A weighted pair of two types `A` and `B`
///
/// This is a very general type that can be used in many different ways. If you
/// are looking for a specific usage example take a look at the (selector
/// impl)[`WeightedPair::select`] on this type.
#[derive(Debug, Copy, Clone)]
pub struct WeightedPair<A, B> {
    a: A,
    b: B,
    distr: Option<Bernoulli>,
    weight_sum: u32,
}

impl<A, B> WeightedPair<A, B> {
    /// Creates a new [`WeightedPair`] from two types (`A` and `B`) implementing
    /// [`WithWeight`]
    ///
    /// Note that this calls the [`WithWeight::weight`] function at construction
    /// time and not at usage time, so dynamic weights are not directly
    /// supported (using interiour mutability)
    ///
    /// # Errors
    /// - [`WeightSumOverflow`] if the total sum of the weights of a and b would
    ///   overflow `u32::MAX`
    pub fn new(a: A, b: B) -> Result<Self, WeightSumOverflow>
    where
        A: WithWeight,
        B: WithWeight,
    {
        let a_weight = a.weight();
        let b_weight = b.weight();

        let weight_sum = a_weight
            .checked_add(b_weight)
            .ok_or(WeightSumOverflow(a_weight, b_weight))?;

        let distr = Bernoulli::from_ratio(a_weight, weight_sum).ok();

        Ok(Self {
            a,
            b,
            distr,
            weight_sum,
        })
    }

    /// Create a new [`WeightedPair`] by using the [`Default::default()`]
    /// implementation of `A` and `B`
    ///
    /// This directly forwards to [`WeightedPair::new`]. See there for more
    /// details.
    ///
    /// # Errors
    /// - [`WeightSumOverflow`] if the total sum of the weights of a and b would
    ///   overflow `u32::MAX`
    pub fn new_default() -> Result<Self, WeightSumOverflow>
    where
        A: WithWeight + Default,
        B: WithWeight + Default,
    {
        Self::new(Default::default(), Default::default())
    }
}

// Note: This implementation is important since it allows recursive chaining of
// this type
impl<A, B> WithWeight for WeightedPair<A, B> {
    /// Returns the total weight of both elements of the pair
    fn weight(&self) -> u32 {
        self.weight_sum
    }
}

impl<P, A, B> Selector<P> for WeightedPair<A, B>
where
    P: Population,
    A: WithWeight + Selector<P>,
    B: WithWeight + Selector<P>,
{
    type Error = SelectionError<WeightedPairError<A::Error, B::Error>>;

    /// Select one of two [`Selector`]s based on their weights.
    ///
    /// This implementation can be used to randomly choose between multiple
    /// selectors, by chaining this type recursively.
    ///
    /// See the underlying Selector types for more information on how they work.
    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
    ) -> Result<&'pop <P as Population>::Individual, Self::Error> {
        let Some(distr) = self.distr else {
            return Err(ZeroWeight.into());
        };
        if distr.sample(rng) {
            self.a.select(population, rng).map_err(WeightedPairError::A)
        } else {
            self.b.select(population, rng).map_err(WeightedPairError::B)
        }
        .map_err(SelectionError::Selector)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use test_strategy::proptest;

    use crate::{
        operator::selector::{
            Selector, best::Best, random::Random, tournament::Tournament, worst::Worst,
        },
        weighted::{
            Weighted,
            error::{SelectionError, WeightSumOverflow, ZeroWeight},
            weighted_pair::WeightedPair,
            with_weight::WithWeight,
            with_weighted_item::WithWeightedItem,
        },
    };

    #[test]
    fn can_construct_pair() {
        let weighted = Weighted::new(Best, 5).with_item_and_weight(Worst, 8);
        assert!(matches!(
            weighted,
            Ok(WeightedPair {
                a: Weighted {
                    item: Best,
                    weight: 5
                },
                b: Weighted {
                    item: Worst,
                    weight: 8
                },
                distr: Some(_),
                weight_sum: 13,
            })
        ));
        assert_eq!(13, weighted.unwrap().weight());
    }

    #[proptest]
    fn best_or_worst(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Best, 1)
            .with_item_and_weight(Worst, 1)
            .unwrap();
        let selection = weighted.select(&pop, &mut rng).unwrap();
        let extremes: [&i32; 2] = pop.iter().minmax().into_option().unwrap().into();
        assert!(extremes.contains(&selection));
    }

    #[proptest]
    fn several_selectors(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Best, 1)
            .with_item_and_weight(Worst, 1)
            .with_item_and_weight(Random, 0)
            .with_item_and_weight(Tournament::of_size::<3>(), 2)
            .unwrap();
        let selection = weighted.select(&pop, &mut rng).unwrap();
        assert!(pop.contains(selection));
    }

    #[test]
    fn zero_weight_sum_error() {
        let pop = [5, 8, 9, 6, 3, 2, 0];
        let mut rng = rand::rng();
        let weighted = Weighted::new(Best, 0)
            .with_item_and_weight(Worst, 0)
            .with_item_and_weight(Random, 0)
            .unwrap();
        // If all the weights are zero, then selection should return an appropriate
        // error type.
        assert_eq!(
            weighted.select(&pop, &mut rng).unwrap_err(),
            SelectionError::from(ZeroWeight)
        );
    }

    #[test]
    fn weight_sum_overflow_error() {
        let weighted = Weighted::new(Best, u32::MAX).with_item_and_weight(Worst, 0);
        assert!(weighted.is_ok());
        let weighted = weighted
            .with_item_and_weight(Random, u32::MAX - 1)
            .unwrap_err();
        assert_eq!(weighted, WeightSumOverflow(u32::MAX, u32::MAX - 1));
    }

    #[test]
    fn weight_sum_overflow_error_chaining() {
        let weighted = Weighted::new(Best, u32::MAX).with_item_and_weight(Random, u32::MAX - 1);
        assert!(weighted.is_err());
        let weighted = weighted.with_item_and_weight(Worst, 0).unwrap_err();
        assert_eq!(weighted, WeightSumOverflow(u32::MAX, u32::MAX - 1));
    }
}
