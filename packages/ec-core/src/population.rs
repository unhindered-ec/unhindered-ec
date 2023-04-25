#![allow(clippy::missing_panics_doc)]

use std::iter::repeat_with;

use rand::rngs::ThreadRng;

use crate::generator::Generator;

pub trait Population {
    type Individual;

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn size(&self) -> usize;
}

impl<I> Population for Vec<I> {
    type Individual = I;

    fn size(&self) -> usize {
        self.len()
    }
}

pub struct GeneratorContext<IC> {
    pub population_size: usize,
    pub individual_context: IC,
}

impl<I, IC> Generator<Vec<I>, GeneratorContext<IC>> for ThreadRng
where
    Self: Generator<I, IC>,
    Vec<I>: Population,
{
    // We could implement this using Rayon's `par_bridge`, but we
    // have to replace `self.generate` with `thread_rng().generate`
    // since we can't pass `self` (a `ThreadRng`) around between
    // threads. Since we only generate populations rarely (typically
    // just once at the beginning) there's not a huge value in
    // parallelizing this action.
    fn generate(&mut self, context: &GeneratorContext<IC>) -> Vec<I> {
        repeat_with(|| self.generate(&context.individual_context))
            .take(context.population_size)
            .collect()
    }
}

#[cfg(test)]
mod generator_trait_tests {
    use core::ops::Range;

    use rand::{thread_rng, Rng};

    use super::*;

    struct RandValue {
        val: i32,
    }

    impl Generator<RandValue, Range<i32>> for ThreadRng {
        fn generate(&mut self, range: &Range<i32>) -> RandValue {
            RandValue {
                val: self.gen_range(range.clone()),
            }
        }
    }

    #[test]
    fn generator_works() {
        let mut rng = thread_rng();
        let population_size = 10;
        let range = -10..25;
        let pop_context = GeneratorContext {
            population_size,
            individual_context: range.clone(),
        };
        let vec_pop = rng.generate(&pop_context);
        assert_eq!(population_size, vec_pop.size());
        for i in vec_pop {
            assert!(range.contains(&i.val));
        }
    }
}
