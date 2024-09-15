use rand::distr::{Bernoulli, Distribution};

use super::{weighted::Weighted, WeightSumOverflow, WithWeight};
use crate::{operator::selector::Selector, population::Population};

#[derive(Debug)]
pub struct WeightedSelectorPair<A, B> {
    pub(crate) a: A,
    pub(crate) b: B,
    pub(crate) distr: Option<Bernoulli>,
    pub(crate) weight_sum: u32,
}

#[derive(Debug)]
pub enum WeightedSelectorsError<A, B> {
    A(A),
    B(B),

    ZeroWeightSum,

    // #[error("Adding the weights {0} and {1} overflows")]
    WeightSumOverflows(u32, u32),
}

impl<P, A, B> Selector<P> for WeightedSelectorPair<A, B>
where
    P: Population,
    A: WithWeight + Selector<P>,
    B: WithWeight + Selector<P>,
{
    type Error = WeightedSelectorsError<A::Error, B::Error>;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Result<&'pop <P as Population>::Individual, Self::Error> {
        let Some(distr) = self.distr else {
            return Err(WeightedSelectorsError::ZeroWeightSum);
        };
        if distr.sample(rng) {
            self.a
                .select(population, rng)
                .map_err(WeightedSelectorsError::A)
        } else {
            self.b
                .select(population, rng)
                .map_err(WeightedSelectorsError::B)
        }
    }
}

impl<A, B> WithWeight for WeightedSelectorPair<A, B> {
    fn weight(&self) -> u32 {
        self.weight_sum
    }
}

impl<A, B> WeightedSelectorPair<Weighted<A>, Weighted<B>> {
    /// # Errors
    /// - [`WeightSumOverflow`] if the total sum of `weight_a` and `weight_b`
    ///   would overflow `u32::MAX`
    pub fn new_with_weights(
        a: A,
        weight_a: u32,
        b: B,
        weight_b: u32,
    ) -> Result<Self, WeightSumOverflow> {
        Self::new(Weighted::new(a, weight_a), Weighted::new(b, weight_b))
    }
}

impl<A, B> WeightedSelectorPair<A, B> {
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
}

#[cfg(test)]
#[rustversion::attr(before(1.81), allow(clippy::unwrap_used))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::unwrap_used,
        reason = "Panicking is the best way to deal with errors in unit tests"
    )
)]
mod tests {
    use itertools::Itertools;
    use test_strategy::proptest;

    use crate::operator::selector::{
        best::Best,
        random::Random,
        tournament::Tournament,
        weighted::{
            weighted::Weighted,
            weighted_selector_pair::{WeightedSelectorPair, WeightedSelectorsError},
            with_weighted_selector::WithWeightedSelector,
            WeightSumOverflow, WithWeight,
        },
        worst::Worst,
        Selector,
    };

    #[test]
    fn can_construct_pair() {
        let weighted = Weighted::new(Best, 5).with_selector_and_weight(Worst, 8);
        assert!(matches!(
            weighted,
            Ok(WeightedSelectorPair {
                a: Weighted {
                    selector: Best,
                    weight: 5
                },
                b: Weighted {
                    selector: Worst,
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
        let mut rng = rand::thread_rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Best, 1)
            .with_selector_and_weight(Worst, 1)
            .unwrap();
        let selection = weighted.select(&pop, &mut rng).unwrap();
        let extremes: [&i32; 2] = pop.iter().minmax().into_option().unwrap().into();
        assert!(extremes.contains(&selection));
    }

    #[proptest]
    fn several_selectors(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::thread_rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Best, 1)
            .with_selector_and_weight(Worst, 1)
            .with_selector_and_weight(Random, 0)
            .with_selector_and_weight(Tournament::of_size::<3>(), 2)
            .unwrap();
        let selection = weighted.select(&pop, &mut rng).unwrap();
        assert!(pop.contains(selection));
    }

    #[test]
    fn zero_weight_sum_error() {
        let pop = vec![5, 8, 9, 6, 3, 2, 0];
        let mut rng = rand::thread_rng();
        let weighted = Weighted::new(Best, 0)
            .with_selector_and_weight(Worst, 0)
            .with_selector_and_weight(Random, 0)
            .unwrap();
        // If all the weights are zero, then selection should return an appropriate
        // error type.
        assert_eq!(
            weighted.select(&pop, &mut rng).unwrap_err(),
            WeightedSelectorsError::ZeroWeightSum
        );
    }

    #[test]
    fn weight_sum_overflow_error() {
        let weighted = Weighted::new(Best, u32::MAX).with_selector_and_weight(Worst, 0);
        assert!(weighted.is_ok());
        let weighted = weighted
            .with_selector_and_weight(Random, u32::MAX - 1)
            .unwrap_err();
        assert_eq!(weighted, WeightSumOverflow(u32::MAX, u32::MAX - 1));
    }

    #[test]
    fn weight_sum_overflow_error_chaining() {
        let weighted = Weighted::new(Best, u32::MAX).with_selector_and_weight(Random, u32::MAX - 1);
        assert!(weighted.is_err());
        let weighted = weighted.with_selector_and_weight(Worst, 0).unwrap_err();
        assert_eq!(weighted, WeightSumOverflow(u32::MAX, u32::MAX - 1));
    }
}
