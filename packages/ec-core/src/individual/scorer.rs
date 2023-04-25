use crate::test_results::TestResults;

pub trait Scorer<G, R> {
    /// Take a reference to a genome and return some score type `R`.
    fn score(&self, genome: &G) -> R;
}

impl<G, R, T> Scorer<G, R> for T
where
    T: Fn(&G) -> R,
{
    fn score(&self, genome: &G) -> R {
        self(genome)
    }
}
