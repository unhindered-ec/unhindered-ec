use std::borrow::Borrow;

use rand::rngs::ThreadRng;

pub mod ec;

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
}