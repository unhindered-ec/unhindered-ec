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

#[cfg(test)]
mod generator_trait_tests {
    use core::ops::Range;

    use rand::{rngs::ThreadRng, thread_rng, Rng};

    use super::*;
    use crate::generator::{collection::CollectionGenerator, Generator};

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
        let pop_generator = CollectionGenerator {
            size: population_size,
            element_generator: range.clone(),
        };
        let vec_pop = pop_generator.generate(&mut rng).unwrap();
        assert_eq!(population_size, vec_pop.size());
        for i in vec_pop {
            assert!(range.contains(&i.val));
        }
    }
}
