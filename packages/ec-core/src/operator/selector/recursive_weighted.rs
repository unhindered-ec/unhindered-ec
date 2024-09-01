use rand::distr::{Bernoulli, Distribution};

use super::Selector;
use crate::population::Population;

#[derive(Debug, thiserror::Error)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(u32, u32);

pub trait WeightedSelector {
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
    type Inner;

    /// # Errors
    /// - [`WeightSumOverflow`] if trying to add this new selector with the
    ///   given weight to the existing chain would overflow the total weight
    ///   (i.e. weight sum `> u32::MAX`)
    fn with_selector_and_weight<S>(
        self,
        selector: S,
        weight: u32,
    ) -> Result<WeightedSelectorPair<Self::Inner, Weighted<S>>, WeightSumOverflow> {
        self.with_weighted_selector(Weighted::new(selector, weight))
    }

    /// # Errors
    /// - [`WeightSumOverflow`] if trying to add this new selector to the
    ///   existing chain would overflow the total weight (i.e. weight sum `>
    ///   u32::MAX`)
    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::Inner, WS>, WeightSumOverflow>
    where
        WS: WeightedSelector;
}

impl<T> WithWeightedSelector for Weighted<T> {
    type Inner = Self;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::Inner, WS>, WeightSumOverflow>
    where
        WS: WeightedSelector,
    {
        WeightedSelectorPair::new(self, weighted_selector)
    }
}

impl<A, B> WithWeightedSelector for WeightedSelectorPair<A, B> {
    type Inner = Self;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::Inner, WS>, WeightSumOverflow>
    where
        WS: WeightedSelector,
    {
        WeightedSelectorPair::new(self, weighted_selector)
    }
}

impl<T> WithWeightedSelector for Result<Weighted<T>, WeightSumOverflow> {
    type Inner = Weighted<T>;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::Inner, WS>, WeightSumOverflow>
    where
        WS: WeightedSelector,
    {
        WeightedSelectorPair::new(self?, weighted_selector)
    }
}

impl<A, B> WithWeightedSelector for Result<WeightedSelectorPair<A, B>, WeightSumOverflow> {
    type Inner = WeightedSelectorPair<A, B>;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<Self::Inner, WS>, WeightSumOverflow>
    where
        WS: WeightedSelector,
    {
        WeightedSelectorPair::new(self?, weighted_selector)
    }
}

impl<T> Weighted<T> {
    pub const fn new(selector: T, weight: u32) -> Self {
        Self { selector, weight }
    }
}

impl<T> WeightedSelector for Weighted<T> {
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
    A: WeightedSelector + Selector<P>,
    B: WeightedSelector + Selector<P>,
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

impl<A, B> WeightedSelector for WeightedSelectorPair<A, B> {
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
        A: WeightedSelector,
        B: WeightedSelector,
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
