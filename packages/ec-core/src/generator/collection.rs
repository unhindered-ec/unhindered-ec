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
        // Doing some reading, I _think_ this will properly pre-allocate an
        // appropriately sized `Vec` to collect into.
        // https://users.rust-lang.org/t/collect-for-exactsizediterator/54367/2
        // says, for example, that collecting into a `Vec` will pre-allocate to the
        // minimum returned by `type_hints`. Looking at the code,
        // it seems that `repeat_with` returns "infinity" for the
        // minimum size, and `take` returns the `min` of `self.size` and the minimum
        // size from the preceding iterator (infinity). This will always be
        // `self.size`, which is just what we'd want the size allocation to be.
        repeat_with(|| self.element_generator.generate(rng))
            .take(self.size)
            .collect()
    }
}
