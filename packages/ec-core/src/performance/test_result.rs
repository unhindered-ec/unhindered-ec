use std::cmp::Ordering;

use crate::performance::{error_value::ErrorValue, score_value::ScoreValue};

/// A test result for a single test. Can be either a [`ScoreValue`] or
/// [`ErrorValue`].
///
/// This type doesn't implement [`Ord`], since the [`TestResult::Score`] and
/// [`TestResult::Error`] variants are not comparable.
///
/// If comparing two [`TestResult::Score`] variants, bigger is better,
/// if comparing two [`TestResult::Error`] variants, smaller is better.
///
/// If you don't need the ability to have both bigger-is-better and
/// smaller-is-better semantics, take a look at using
///
/// - [`ScoreValue`] (bigger is better), or
/// - [`ErrorValue`] (smaller is better)
///
/// directly instead, which are probably better suited for your needs (and
/// as a bonus also implement [`Ord`].)
///
/// # Examples
/// ```
/// # use ec_core::performance::test_result::TestResult;
/// # use std::cmp::Ordering;
/// #
/// let score = |x| TestResult::<i32, i32>::score(x);
/// let error = |x| TestResult::<i32, i32>::error(x);
///
/// assert_eq!(score(5).partial_cmp(&score(10)), Some(Ordering::Less));
/// assert_eq!(error(5).partial_cmp(&error(10)), Some(Ordering::Greater));
/// assert_eq!(score(5).partial_cmp(&error(10)), None);
/// ```
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum TestResult<S, E> {
    // Score, bigger is better semantics
    Score(ScoreValue<S>),
    // Error, smaller is better semantics
    Error(ErrorValue<E>),
}

impl<S, E> From<ScoreValue<S>> for TestResult<S, E> {
    /// Create a new [`TestResult::Score`] from a [`ScoreValue`] value.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::{test_result::TestResult, score_value::ScoreValue};
    ///
    /// assert_eq!(
    ///     TestResult::<i32, i32>::from(ScoreValue(5)),
    ///     TestResult::Score(ScoreValue(5))
    /// );
    /// ```
    fn from(value: ScoreValue<S>) -> Self {
        Self::Score(value)
    }
}

impl<S, E> From<ErrorValue<E>> for TestResult<S, E> {
    /// Create a new [`TestResult::Error`] from a [`ErrorValue`] value.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::{test_result::TestResult, error_value::ErrorValue};
    ///
    /// assert_eq!(
    ///     TestResult::<i32, i32>::from(ErrorValue(5)),
    ///     TestResult::Error(ErrorValue(5))
    /// );
    /// ```
    fn from(value: ErrorValue<E>) -> Self {
        Self::Error(value)
    }
}

impl<S, E> TestResult<S, E> {
    /// Create a new [`TestResult::Score`] from the given value, with bigger is
    /// better semantics.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::{test_result::TestResult, score_value::ScoreValue};
    /// assert_eq!(
    ///     TestResult::<i32, i32>::score(5),
    ///     TestResult::Score(ScoreValue(5))
    /// );
    /// ```
    #[must_use]
    pub const fn score(score: S) -> Self {
        Self::Score(ScoreValue(score))
    }

    /// Create a new [`TestResult::Error`] from the given value, with smaller is
    /// better semantics.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::{test_result::TestResult, error_value::ErrorValue};
    /// assert_eq!(
    ///     TestResult::<i32, i32>::error(5),
    ///     TestResult::Error(ErrorValue(5))
    /// );
    /// ```
    #[must_use]
    pub const fn error(error: E) -> Self {
        Self::Error(ErrorValue(error))
    }
}

impl<S: Default, E> Default for TestResult<S, E> {
    /// Create a new [`TestResult`], defaulting to
    /// [`TestResult::Score(ScoreValue::default())`](TestResult::Score).
    ///
    /// This decision is somewhat arbitrary, but ensures that users of
    /// [`TestResult`] have an implementation of [`Default`] if they need one.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::test_result::TestResult;
    /// assert_eq!(TestResult::<i32, i32>::default(), TestResult::score(0));
    /// ```
    fn default() -> Self {
        Self::Score(ScoreValue::default())
    }
}

impl<S: PartialOrd, E: PartialOrd> PartialOrd for TestResult<S, E> {
    /// Compare two [`TestResult`]s.
    ///
    /// - [`TestResult::Score`] and [`TestResult::Error`] are not comparable and
    ///   will always return [`None`].
    /// - Two [`TestResult::Score`]'s will be compared by comparing the inner
    ///   [`ScoreValue<S>`](ScoreValue) values.
    /// - Two [`TestResult::Error`]'s will be compared by comparing the inner
    ///   [`ErrorValue<E>`](ErrorValue) values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::test_result::TestResult;
    /// # use std::cmp::Ordering;
    /// #
    /// let score = |x| TestResult::<i32, i32>::score(x);
    /// let error = |x| TestResult::<i32, i32>::error(x);
    ///
    /// assert_eq!(score(5).partial_cmp(&score(10)), Some(Ordering::Less));
    /// assert_eq!(error(5).partial_cmp(&error(10)), Some(Ordering::Greater));
    /// assert_eq!(score(5).partial_cmp(&error(10)), None);
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Score(self_score), Self::Score(other_score)) => {
                self_score.partial_cmp(other_score)
            }
            (Self::Error(self_error), Self::Error(other_error)) => {
                self_error.partial_cmp(other_error)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
pub(crate) mod test_result_tests {
    use std::ops::Not;

    use super::*;

    #[test]
    fn score_compares_to_score() {
        let first: TestResult<i32, i32> = TestResult::Score(ScoreValue(32));
        let second = TestResult::Score(ScoreValue(87));
        assert!(first < second);
        assert!(first != second);
        assert!((first > second).not());
    }

    #[test]
    fn error_compares_to_error() {
        let first: TestResult<i32, i32> = TestResult::Error(ErrorValue(32));
        let second = TestResult::Error(ErrorValue(87));
        assert!(first > second);
        assert!(first != second);
        assert!((first < second).not());
    }

    #[test]
    fn error_and_score_incomparable() {
        let first = TestResult::Score(ScoreValue(32));
        let second = TestResult::Error(ErrorValue(87));
        assert!((first > second).not());
        assert!(first != second);
        assert!((first < second).not());
        assert!(first.partial_cmp(&second).is_none());
        assert!(second.partial_cmp(&first).is_none());
    }
}
