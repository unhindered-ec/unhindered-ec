pub trait Scorer<G, R> {
    /// Take a reference to a genome and return some score type `R`.
    fn score(&self, genome: &G) -> R;
}

pub struct FnScorer<T>(pub T);

impl<G, R, T> Scorer<G, R> for FnScorer<T>
where
    T: Fn(&G) -> R,
{
    fn score(&self, genome: &G) -> R {
        self.0(genome)
    }
}

impl<G, R, T> Scorer<G, R> for &T
where
    T: Scorer<G, R>,
{
    fn score(&self, genome: &G) -> R {
        (**self).score(genome)
    }
}
