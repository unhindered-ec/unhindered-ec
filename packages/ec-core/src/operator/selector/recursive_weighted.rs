use std::marker::PhantomData;

use rand::{seq::IndexedRandom, Rng};

use super::Selector;
use crate::{operator::selector::weighted::WeightedError, population::Population};

trait WithWeight<P>: Selector<P>
where
    P: Population,
{
    fn weight(&self) -> usize;
}

struct Weighted<T> {
    weight: usize,
    selector: T,
}

impl<P, T> WithWeight<P> for Weighted<T>
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

struct WeightedSelectors<P, A, B>
where
    P: Population,
    A: WithWeight<P>,
    B: WithWeight<P>,
{
    a: A,
    b: B,
    _p: PhantomData<P>,
}

enum WeightedSelectorsError<P, A, B>
where
    P: Population,
    A: Selector<P>,
    B: Selector<P>,
{
    A(A::Error),
    B(B::Error),
    NonZeroSum,
    // #[error("Adding the weights {0} and {1} overflows")]
    WeightSumOverflows(usize, usize),
}

impl<P, A, B> Selector<P> for WeightedSelectors<P, A, B>
where
    P: Population,
    A: WithWeight<P>,
    B: WithWeight<P>,
{
    type Error = WeightedSelectorsError<P, A, B>;

    fn select<'pop>(
        &self,
        population: &'pop P,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Result<&'pop <P as Population>::Individual, Self::Error> {
        let a_weight = self.a.weight();
        let b_weight = self.b.weight();
        let weight_sum = a_weight
            .checked_add(b_weight)
            .ok_or_else(|| WeightedSelectorsError::WeightSumOverflows(a_weight, b_weight))?;
        if weight_sum == 0 {
            return Err(WeightedSelectorsError::NonZeroSum);
        }
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

// impl<A: Selector, B: Selector> Selector for WeightedSelector<A, B> {
//     type Error = WSelectorError<A, B>;
// }

// impl<A: WithWeight, B: WithWeight> WithWeight for WeightedSelector<A, B> {
//     const WEIGHT: usize = { A::WEIGHT + B::WEIGHT };
// }

// impl<A: WithWeight, B: WithWeight> WeightedSelector<A, B> {
//     fn new<const WEIGHT_A: usize, const WEIGHT_B: usize, Sel_A: Selector,
// Sel_B: Selector>(         selector1: Sel_A,
//         selector2: Sel_B,
//     ) -> WeightedSelector<Weight<WEIGHT_A, Sel_A>, Weight<WEIGHT_B, Sel_B>> {
//         WeightedSelector {
//             a: Weight {
//                 selector: selector1,
//             },
//             b: Weight {
//                 selector: selector2,
//             },
//         }
//     }

//     fn with_selector<const WEIGHT: usize, const Sel: Selector>(
//         selector: A,
//     ) -> WeightedSelector<Weight<WEIGHT, Sel>, Self> {
//         WeightedSelector {
//             a: Weight { selector },
//             b: self,
//         }
//     }

//     // this should be in the trait but for simplicities sake this is
// pseudocode     fn select() {
//         let weight_a = A::WEIGHT;
//         let weight_b = B::WEIGHT;

//         if choose(weight_a, weight_b) {
//             self.a.select()
//         } else {
//             self.b.select()
//         }
//     }
// }
