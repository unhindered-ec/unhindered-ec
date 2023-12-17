use std::borrow::Borrow;

use rand::rngs::ThreadRng;

pub mod ec;
pub mod scorer;

// TODO: I need to add a `new()` (or similar) method
//   that takes a genome and returns a (scored) individual
//   containing that genome. It's not clear to me at the
//   whether that should be:
//     * An entirely new trait (like `Generate`)
//     * A method in `Generate`
//     * A method in `Individual`
#[deprecated(note = "Use `Generator` trait instead")]
pub trait Generate: Individual {
    fn generate<H>(
        make_genome: impl Fn(&mut ThreadRng) -> Self::Genome,
        // TODO: Should this be a special EC-specific trait instead of the general `Fn`?
        run_tests: impl Fn(&H) -> Self::TestResults,
        rng: &mut ThreadRng,
    ) -> Self
    where
        Self::Genome: Borrow<H>,
        H: ?Sized;
}

pub trait Individual {
    type Genome;
    type TestResults;

    fn genome(&self) -> &Self::Genome;
    fn test_results(&self) -> &Self::TestResults;
}
