use std::iter::repeat_with;

use rand::rngs::ThreadRng;

use super::Generator;

#[allow(clippy::module_name_repetitions)]
pub struct CollectionGenerator<C> {
    pub size: usize,
    pub element_generator: C,
}

impl<T, C> Generator<Vec<T>> for CollectionGenerator<C>
where
    C: Generator<T>,
{
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<Vec<T>> {
        repeat_with(|| self.element_generator.generate(rng))
            .take(self.size)
            .collect()
    }
}
