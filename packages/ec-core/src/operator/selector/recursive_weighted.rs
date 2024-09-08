use rand::distr::{Bernoulli, Distribution};

use super::Selector;
use crate::population::Population;

#[derive(Debug, thiserror::Error)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(u32, u32);

pub trait WithWeight {
    fn weight(&self) -> u32;
}

pub struct Weighted<T> {
    selector: T,
    weight: u32,
}

pub trait WithWeightedSelector
where
    Self: Sized,
{
    /// The type of the selector being extended
    /// through the addition of a new weighted selector.
    ///
    /// `InnerSelector` is often just `Self`, but in cases we
    /// implement this trait for a `Result` type, `InnerSelector`
    /// allows us to specify the type of the value for that `Result`.
    /// This allows us to chain the construction of
    /// complex weighted selectors ignoring the possibility of errors
    /// until the end of the chain.
    type InnerSelector;

    /// # Errors
    /// - [`WeightSumOverflow`] if trying to add this new selector with the
    ///   given weight to the existing chain would overflow the total weight
    ///   (i.e. weight sum `> u32::MAX`)
    fn with_selector_and_weight<S>(
        self,
        selector: S,
        weight: u32,
    ) -> Result<WeightedSelectorPair<Self::InnerSelector, Weighted<S>>, WeightSumOverflow> {
        self.with_weighted_selector(Weighted::new(selector, weight))
    }

    /// # Errors
    /// - [`WeightSumOverflow`] if trying to add this new selector to the
    ///   existing chain would overflow the total weight (i.e. weight sum `>
    ///   u32::MAX`)
    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::InnerSelector, WS>, WeightSumOverflow>
    where
        WS: WithWeight;
}

impl<T> WithWeightedSelector for Weighted<T> {
    type InnerSelector = Self;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::InnerSelector, WS>, WeightSumOverflow>
    where
        WS: WithWeight,
    {
        WeightedSelectorPair::new(self, weighted_selector)
    }
}

impl<A, B> WithWeightedSelector for WeightedSelectorPair<A, B> {
    type InnerSelector = Self;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::InnerSelector, WS>, WeightSumOverflow>
    where
        WS: WithWeight,
    {
        WeightedSelectorPair::new(self, weighted_selector)
    }
}

impl<T> WithWeightedSelector for Result<T, WeightSumOverflow>
where
    T: WithWeightedSelector,
{
    // Since we're implementing the trait for a `Result` type,
    // the type of the inner selector comes from the value type
    // `T`. Since `T` implements `WithWeightedSelect`, we need
    // to use `T::InnerSelector` to access `T`'s selector type.
    type InnerSelector = T::InnerSelector;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::InnerSelector, WS>, WeightSumOverflow>
    where
        WS: WithWeight,
    {
        self?.with_weighted_selector(weighted_selector)
    }
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

pub struct WeightedSelectorPair<A, B> {
    a: A,
    b: B,
    distr: Option<Bernoulli>,
    weight_sum: u32,
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
        let Some(weight_sum) = a_weight.checked_add(b_weight) else {
            return Err(WeightSumOverflow(a_weight, b_weight));
        };

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

    use super::WeightSumOverflow;
    use crate::operator::selector::{
        best::Best,
        random::Random,
        recursive_weighted::{Weighted, WithWeightedSelector},
        tournament::Tournament,
        worst::Worst,
        Selector,
    };

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
}
