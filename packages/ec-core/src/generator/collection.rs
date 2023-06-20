use std::iter::repeat_with;

use rand::rngs::ThreadRng;

use super::Generator;

/// Information for generating a collection of random elements.
///
/// `size` indicates how many elements to generate.
/// `element_generator` is used to generate individual elements.
#[allow(clippy::module_name_repetitions)]
pub struct CollectionGenerator<C> {
    pub size: usize,
    pub element_generator: C,
}

/// Generate a `Vec` of random elements.
///
/// The number of element and the mechanism for generating
/// random elements are specified in the `CollectionGenerator`
/// struct.
///
/// # Errors
///
/// This returns an `anyhow::Error` generating any of
/// the elements returns an error.
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
