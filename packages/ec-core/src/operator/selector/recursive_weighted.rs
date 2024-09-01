use std::marker::PhantomData;

use rand::Rng;

use super::{weighted, Selector};
use crate::population::Population;

#[derive(Debug)]
pub enum WeightedSelectorCreationError {
    WeightSumOverflows(usize, usize),
}

pub trait WeightedSelector<P>: Selector<P>
where
    P: Population,
{
    fn weight(&self) -> usize;
}

pub struct Weighted<T> {
    selector: T,
    weight: usize,
}

impl<T> Weighted<T> {
    pub const fn new(selector: T, weight: usize) -> Self {
        Self { selector, weight }
    }

    pub const fn with_selector_and_weight<P, S>(
        self,
        selector: S,
        weight: usize,
    ) -> WeightedSelectorPair<P, Self, Weighted<S>>
    where
        P: Population,
        S: Selector<P>,
        T: Selector<P>,
    {
        WeightedSelectorPair {
            a: self,
            b: Weighted { selector, weight },
            _p: PhantomData,
        }
    }

    pub const fn with_weighted_selector<P, WS>(
        self,
        weighted_selector: WS,
    ) -> WeightedSelectorPair<P, Self, WS>
    where
        P: Population,
        WS: WeightedSelector<P>,
        T: Selector<P>,
    {
        WeightedSelectorPair {
            a: self,
            b: weighted_selector,
            _p: PhantomData,
        }
    }
}

impl<P, T> WeightedSelector<P> for Weighted<T>
where
    P: Population,
    T: Selector<P>,
{
    fn weight(&self) -> usize {
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

pub struct WeightedSelectorPair<P, A, B>
where
    P: Population,
    A: WeightedSelector<P>,
    B: WeightedSelector<P>,
{
    a: A,
    b: B,
    // weight: usize,
    _p: PhantomData<P>,
}

pub enum WeightedSelectorsError<P, A, B>
where
    P: Population,
    A: Selector<P>,
    B: Selector<P>,
{
    A(A::Error),
    B(B::Error),

    ZeroWeightSum,

    // #[error("Adding the weights {0} and {1} overflows")]
    WeightSumOverflows(usize, usize),
}

impl<P, A, B> std::fmt::Debug for WeightedSelectorsError<P, A, B>
where
    P: Population,
    A: Selector<P>,
    A::Error: std::fmt::Debug,
    B: Selector<P>,
    B::Error: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(arg0) => f.debug_tuple("A").field(arg0).finish(),
            Self::B(arg0) => f.debug_tuple("B").field(arg0).finish(),
            Self::ZeroWeightSum => write!(f, "ZeroWeightSum"),
            Self::WeightSumOverflows(arg0, arg1) => f
                .debug_tuple("WeightSumOverflows")
                .field(arg0)
                .field(arg1)
                .finish(),
        }
    }
}

impl<P, A, B> Selector<P> for WeightedSelectorPair<P, A, B>
where
    P: Population,
    A: WeightedSelector<P>,
    B: WeightedSelector<P>,
{
    type Error = WeightedSelectorsError<P, A, B>;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Result<&'pop <P as Population>::Individual, Self::Error> {
        let a_weight = self.a.weight();
        let b_weight = self.b.weight();
        // For performance reasons, it would be nice to check that the sum works
        // and is > 0 at construction time so we don't have to do it here.
        let weight_sum = a_weight
            .checked_add(b_weight)
            .ok_or_else(|| WeightedSelectorsError::WeightSumOverflows(a_weight, b_weight))?;
        if weight_sum == 0 {
            return Err(WeightedSelectorsError::ZeroWeightSum);
        }
        //------------------
        let choose_a = rng.gen_range(0..weight_sum) < a_weight;
        if choose_a {
            self.a
                .select(population, rng)
                .map_err(|e| WeightedSelectorsError::A(e))
        } else {
            self.b
                .select(population, rng)
                .map_err(|e| WeightedSelectorsError::B(e))
        }
    }
}

impl<P, A, B> WeightedSelector<P> for WeightedSelectorPair<P, A, B>
where
    P: Population,
    A: WeightedSelector<P>,
    B: WeightedSelector<P>,
{
    fn weight(&self) -> usize {
        self.a.weight() + self.b.weight()
    }
}

impl<P, A, B> WeightedSelectorPair<P, Weighted<A>, Weighted<B>>
where
    P: Population,
    A: Selector<P>,
    B: Selector<P>,
{
    pub fn new_with_weights(
        a: A,
        weight_a: usize,
        b: B,
        weight_b: usize,
    ) -> Result<Self, WeightedSelectorCreationError> {
        Self::new(Weighted::new(a, weight_a), Weighted::new(b, weight_b))
    }
}

impl<P, A, B> WeightedSelectorPair<P, A, B>
where
    P: Population,
    A: WeightedSelector<P>,
    B: WeightedSelector<P>,
{
    pub fn new(a: A, b: B) -> Result<Self, WeightedSelectorCreationError> {
        let a_weight = a.weight();
        let b_weight = b.weight();
        let Some(weight) = a_weight.checked_add(b_weight) else {
            return Err(WeightedSelectorCreationError::WeightSumOverflows(
                a_weight, b_weight,
            ));
        };
        Ok(Self {
            a,
            b,
            _p: PhantomData,
        })
    }

    pub fn with_selector_and_weight<S: Selector<P>>(
        self,
        selector: S,
        weight: usize,
    ) -> Result<WeightedSelectorPair<P, Self, Weighted<S>>, WeightedSelectorCreationError> {
        Self::with_weighted_selector(self, Weighted::new(selector, weight))
    }

    pub fn with_weighted_selector<WS: WeightedSelector<P>>(
        self,
        weighted_selector: WS,
    ) -> Result<WeightedSelectorPair<P, Self, WS>, WeightedSelectorCreationError> {
        WeightedSelectorPair::new(self, weighted_selector)
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
        best::Best, random::Random, recursive_weighted::Weighted, tournament::Tournament,
        worst::Worst, Selector,
    };

    #[proptest]
    fn best_or_worst(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::thread_rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Best, 1).with_selector_and_weight(Worst, 1);
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
            .unwrap()
            .with_selector_and_weight(Tournament::of_size::<3>(), 2)
            .unwrap();
        let selection = weighted.select(&pop, &mut rng).unwrap();
        assert!(pop.contains(selection));
    }
}
