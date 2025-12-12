/// Score a genome by some metric.
///
/// See [`FnScorer`] for a wrapper which can convert any function to a scorer.
///
/// Note that similarly to the `Read` and `Write` traits in the
/// standard library, the [`Scorer`] trait is implemented for references to
/// scorers. That means, if you don't wish to consume a scorer and want to
/// re-use it later, you can pass a reference to that scorer to any function
/// expecting a [`Scorer`] instead.
///
/// # Example
/// ```
/// # use ec_core::individual::scorer::{Scorer, FnScorer};
/// #
/// let scorer_function = |x: &i32| x.abs_diff(10);
/// let scorer = FnScorer(scorer_function);
///
/// let score = scorer.score(&8);
///
/// assert_eq!(score, 2);
/// ```
pub trait Scorer<G> {
    type Score;

    /// Take a reference to a genome and return some score of type
    /// `Self::Score`.
    fn score(&self, genome: &G) -> Self::Score;
}

static_assertions::assert_obj_safe!(Scorer<(), Score = ()>);

/// Wrapper to use a `Fn(&Genome) -> Score` as a [`Scorer`].
///
/// # Example
/// ```
/// # use ec_core::individual::scorer::{Scorer, FnScorer};
/// #
/// let scorer_function = |x: &i32| x.abs_diff(15);
/// let scorer = FnScorer(scorer_function);
///
/// let score = scorer.score(&8);
///
/// assert_eq!(score, 7);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FnScorer<T>(pub T);

impl<G, R, T> Scorer<G> for FnScorer<T>
where
    T: Fn(&G) -> R,
{
    type Score = R;

    /// Calculate the score for the passed genome using this [`FnScorer`]'s
    /// function.
    ///
    /// # Example
    /// ```
    /// # use ec_core::{test_results::Error, individual::scorer::{Scorer, FnScorer}};
    /// #
    /// let scorer = FnScorer(|genome: &i32| Error(-*genome));
    ///
    /// let score = scorer.score(&10);
    /// assert_eq!(score, -10);
    /// ```
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
