pub trait Scorer<G> {
    type Score;

    /// Take a reference to a genome and return some score type `R`.
    fn score(&self, genome: &G) -> Self::Score;
}

static_assertions::assert_obj_safe!(Scorer<(), Score = ()>);

#[derive(Clone, Copy)]
pub struct FnScorer<T>(pub T);

impl<G, R, T> Scorer<G> for FnScorer<T>
where
    T: Fn(&G) -> R,
{
    type Score = R;

    fn score(&self, genome: &G) -> Self::Score {
        self.0(genome)
    }
}

impl<G, T> Scorer<G> for &T
where
    T: Scorer<G>,
{
    type Score = T::Score;

    fn score(&self, genome: &G) -> Self::Score {
        (**self).score(genome)
    }
}
