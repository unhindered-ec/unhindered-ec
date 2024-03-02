pub trait Scorer<Genome, Score> {
    /// Take a reference to a genome and return some score type `R`.
    fn score(&self, genome: &Genome) -> Score;
}

#[derive(Clone, Copy)]
pub struct FnScorer<T>(pub T);

impl<G, T, S> Scorer<G, S> for FnScorer<T>
where
    T: Fn(&G) -> S,
{
    fn score(&self, genome: &G) -> S {
        self.0(genome)
    }
}

impl<G, T, S> Scorer<G, S> for &T
where
    T: Scorer<G, S>,
{
    fn score(&self, genome: &G) -> S {
        (**self).score(genome)
    }
}
