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
