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

impl<I, IC> Generator<Vec<I>> for GeneratorContext<IC>
where
    IC: Generator<I>,
    Vec<I>: Population,
{
    // We could implement this using Rayon's `par_bridge`, but we
    // have to replace `self.generate` with `thread_rng().generate`
    // since we can't pass `self` (a `ThreadRng`) around between
    // threads. Since we only generate populations rarely (typically
    // just once at the beginning) there's not a huge value in
    // parallelizing this action.
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<Vec<I>> {
        repeat_with(|| self.individual_context.generate(rng))
            .take(self.population_size)
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

    impl Generator<RandValue> for Range<i32> {
        fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<RandValue> {
            Ok(RandValue {
                val: rng.gen_range(self.clone()),
            })
        }
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn generator_works() {
        let mut rng = thread_rng();
        let population_size = 10;
        let range = -10..25;
        let pop_context = GeneratorContext {
            population_size,
            individual_context: range.clone(),
        };
        let vec_pop = pop_context.generate(&mut rng).unwrap();
        assert_eq!(population_size, vec_pop.size());
        for i in vec_pop {
            assert!(range.contains(&i.val));
        }
    }
}
