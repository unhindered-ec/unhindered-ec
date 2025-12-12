use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
    ops::Index,
    slice::SliceIndex,
};

// TODO: We can probably use things in the `num` family of traits
//   (https://github.com/rust-num/num) to genericize `Score` and
//   `Error` so they're not tied to `i64`s anymore.

// TODO: Should there just be one struct (e.g., `Result<T>` with a `result: T`
// field)   and then `Error` and `Score` should be traits that these structs can
//   implement? I feel like that might avoid some duplication here.

/// A result of a single test, bigger is better.
///
/// See also [`Error`], for which smaller is better.
///
/// # Examples
/// ```
/// # use ec_core::test_results::Score;
/// #
/// assert!(Score(5) > Score(-4));
/// ```
/// ```
/// # use ec_core::test_results::Score;
/// #
/// assert!(Score(5) == Score(5));
/// ```
/// ```
/// # use ec_core::test_results::Score;
/// #
/// assert!(Score(-100) < Score(-4));
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy, Hash, Default)]
#[repr(transparent)]
pub struct Score<T>(pub T);

// We need `Score` to be cloneable in many of our applications,
// even if it's not needed here in `ec_core`. For `Score` to be
// cloneable, the generic type must also be cloneable.
static_assertions::assert_impl_all!(Score<()>: Clone);

impl<T: Display> Display for Score<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score (higher is better): {}", self.0)
    }
}

// TODO: Write tests for the `From` and `Sum` trait implementations.

impl<T> Score<T> {
    /// Create a new [`Score`] from the given value
    ///
    /// Also see [`Score::from`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Score;
    /// #
    /// assert_eq!(Score::new(5), Score(5));
    /// ```
    #[must_use]
    pub const fn new(score: T) -> Self {
        Self(score)
    }
}

impl<T: PartialOrd> PartialOrd<T> for Score<T> {
    /// Compares the value of a score
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Score;
    /// #
    /// assert!(Score(100) < 1000);
    /// assert!(Score(10) > 1);
    /// ```
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: PartialEq> PartialEq<T> for Score<T> {
    /// Checks the value of a score for equality
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Score;
    /// #
    /// assert_eq!(Score(100), 100);
    /// assert_ne!(Score(10), 1);
    /// ```
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T> From<T> for Score<T> {
    /// Create a new [`Score`] from the given value
    ///
    /// Also see [`Score::new`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Score;
    /// #
    /// assert_eq!(Score::from(5), Score(5));
    /// ```
    fn from(score: T) -> Self {
        Self(score)
    }
}

impl<T: Sum> Sum<T> for Score<T> {
    /// Create a new [`Score`] from summing up a iterator of values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Score;
    /// #
    /// assert_eq!([5, 8, -3, 10].into_iter().sum::<Score<_>>(), Score(20));
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self(iter.sum())
    }
}

impl<T: Sum> Sum for Score<T> {
    /// Sum up a Iterator of [`Score`]'s.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Score;
    /// #
    /// assert_eq!(
    ///     [Score(5), Score(8), Score(-3), Score(10)]
    ///         .into_iter()
    ///         .sum::<Score<i32>>(),
    ///     Score(20)
    /// );
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.0).sum()
    }
}

impl<'a, T> Sum<&'a Self> for Score<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    /// Sum up a Iterator of references to [`Score`]'s.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Score;
    /// #
    /// assert_eq!(
    ///     [&Score(5), &Score(8), &Score(-3), &Score(10)]
    ///         .into_iter()
    ///         .sum::<Score<_>>(),
    ///     Score(20)
    /// );
    /// ```
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.0.to_owned()).sum()
    }
}

/// A result of a single test, smaller is better.
///
/// See also [`Score`], for which bigger is better.
///
/// # Examples
/// ```
/// # use ec_core::test_results::Error;
/// #
/// assert!(Error(5) < Error(-4));
/// ```
/// ```
/// # use ec_core::test_results::Error;
/// #
/// assert!(Error(5) == Error(5));
/// ```
/// ```
/// # use ec_core::test_results::Error;
/// #
/// assert!(Error(-100) > Error(-4));
/// ```
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Default)]
#[repr(transparent)]
pub struct Error<T>(pub T);

// We need `Error` to be cloneable in many of our applications,
// even if it's not needed here in `ec_core`. For `Error` to be
// cloneable, the generic type must also be cloneable.
static_assertions::assert_impl_all!(Error<()>: Clone);

impl<T: Ord> Ord for Error<T> {
    /// Compares two errors.
    ///
    /// Errors are ordered in reverse wrt. [`Score<T>`](Score) because higher
    /// error values are considered worse.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// # use std::cmp::Ordering;
    /// #
    /// assert!(Error(100) < Error(10));
    /// assert!(Error(20) > Error(1000));
    /// assert_eq!(Error(20).cmp(&Error(100)), Ordering::Greater);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl<T: PartialOrd> PartialOrd for Error<T> {
    /// Compares two errors.
    ///
    /// Errors are ordered in reverse wrt. [`Score<T>`](Score) because higher
    /// error values are considered worse.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// # use std::cmp::Ordering;
    /// #
    /// assert!(Error(100) < Error(10));
    /// assert!(Error(20) > Error(1000));
    /// assert_eq!(Error(20).partial_cmp(&Error(100)), Some(Ordering::Greater));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0
            .partial_cmp(&other.0)
            .map(std::cmp::Ordering::reverse)
    }
}

impl<T: PartialOrd> PartialOrd<T> for Error<T> {
    /// Compares the value of an error
    ///
    /// In contrast to comparing two errors, this is *not* ordered in reverse.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// #
    /// assert!(Error(100) < 1000);
    /// assert!(Error(10) > 1);
    /// ```
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: PartialEq> PartialEq<T> for Error<T> {
    /// Checks the value of an error for equality
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// #
    /// assert_eq!(Error(100), 100);
    /// assert_ne!(Error(10), 1);
    /// ```
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T: Display> Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error (lower is better): {}", self.0)
    }
}
// TODO: Write tests for the `From` and `Sum` trait implementations.

impl<T> Error<T> {
    /// Create a new [`Error`] from the given value
    ///
    /// Also see [`Error::from`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// #
    /// assert_eq!(Error::new(5), Error(5));
    /// ```
    #[must_use]
    pub const fn new(score: T) -> Self {
        Self(score)
    }
}

impl<T> From<T> for Error<T> {
    /// Create a new [`Error`] from the given value
    ///
    /// Also see [`Error::new`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// #
    /// assert_eq!(Error::from(5), Error(5));
    /// ```
    fn from(error: T) -> Self {
        Self(error)
    }
}

impl<T: Sum> Sum<T> for Error<T> {
    /// Create a new [`Error`] from summing up a iterator of values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// #
    /// assert_eq!([5, 8, -3, 10].into_iter().sum::<Error<_>>(), Error(20));
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self(iter.sum())
    }
}

impl<T: Sum> Sum for Error<T> {
    /// Sum up a Iterator of [`Error`]'s.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// #
    /// assert_eq!(
    ///     [Error(5), Error(8), Error(-3), Error(10)]
    ///         .into_iter()
    ///         .sum::<Error<i32>>(),
    ///     Error(20)
    /// );
    /// ```
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.0).sum()
    }
}

impl<'a, T> Sum<&'a Self> for Error<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    /// Sum up a Iterator of references to [`Error`]'s.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::Error;
    /// #
    /// assert_eq!(
    ///     [&Error(5), &Error(8), &Error(-3), &Error(10)]
    ///         .into_iter()
    ///         .sum::<Error<_>>(),
    ///     Error(20)
    /// );
    /// ```
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.0.to_owned()).sum()
    }
}

#[cfg(test)]
mod score_error_tests {
    use super::*;

    #[test]
    fn score_bigger_is_better() {
        let first = Score(37);
        let second = Score(82);
        // These use `Ord`
        assert_eq!(first.cmp(&second), Ordering::Less);
        assert_eq!(second.cmp(&first), Ordering::Greater);
        assert_eq!(first.cmp(&first), Ordering::Equal);
        // Now use `PartialOrd`
        assert_eq!(first.partial_cmp(&second), Some(Ordering::Less));
        assert_eq!(second.partial_cmp(&first), Some(Ordering::Greater));
        assert_eq!(first.partial_cmp(&first), Some(Ordering::Equal));
    }

    #[test]
    fn error_smaller_is_better() {
        let first = Error(37);
        let second = Error(82);
        // These use `Ord`
        assert_eq!(first.cmp(&second), Ordering::Greater);
        assert_eq!(second.cmp(&first), Ordering::Less);
        assert_eq!(first.cmp(&first), Ordering::Equal);
        // Now use `PartialOrd`
        assert_eq!(first.partial_cmp(&second), Some(Ordering::Greater));
        assert_eq!(second.partial_cmp(&first), Some(Ordering::Less));
        assert_eq!(first.partial_cmp(&first), Some(Ordering::Equal));
    }
}

/// A test result for a single test. Can be either a [`Score`] or [`Error`].
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
/// - [`Score`] (bigger is better), or
/// - [`Error`] (smaller is better)
///
/// directly instead, which are probably better suited for your needs (and
/// as a bonus also implement [`Ord`].)
///
/// # Examples
/// ```
/// # use ec_core::test_results::TestResult;
/// # use std::cmp::Ordering;
/// assert_eq!(
///     TestResult::<i32, i32>::score(5).partial_cmp(&TestResult::<i32, i32>::score(10)),
///     Some(Ordering::Less),
/// );
///
/// assert_eq!(
///     TestResult::<i32, i32>::error(5).partial_cmp(&TestResult::<i32, i32>::error(10)),
///     Some(Ordering::Greater),
/// );
///
/// assert_eq!(
///     TestResult::<i32, i32>::score(5).partial_cmp(&TestResult::<i32, i32>::error(10)),
///     None,
/// );
/// ```
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum TestResult<S, E> {
    // Score, bigger is better semantics
    Score(Score<S>),
    // Error, smaller is better semantics
    Error(Error<E>),
}

impl<S, E> From<Score<S>> for TestResult<S, E> {
    /// Create a new [`TestResult::Score`] from a [`Score`] value.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResult, Score};
    ///
    /// assert_eq!(
    ///     TestResult::<i32, i32>::from(Score(5)),
    ///     TestResult::Score(Score(5))
    /// );
    /// ```
    fn from(value: Score<S>) -> Self {
        Self::Score(value)
    }
}

impl<S, E> From<Error<E>> for TestResult<S, E> {
    /// Create a new [`TestResult::Error`] from a [`Error`] value.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResult, Error};
    ///
    /// assert_eq!(
    ///     TestResult::<i32, i32>::from(Error(5)),
    ///     TestResult::Error(Error(5))
    /// );
    /// ```
    fn from(value: Error<E>) -> Self {
        Self::Error(value)
    }
}

impl<S, E> TestResult<S, E> {
    /// Create a new [`TestResult::Score`] from the given value, with bigger is
    /// better semantics.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResult, Score};
    /// assert_eq!(
    ///     TestResult::<i32, i32>::score(5),
    ///     TestResult::Score(Score(5))
    /// );
    /// ```
    #[must_use]
    pub const fn score(score: S) -> Self {
        Self::Score(Score(score))
    }

    /// Create a new [`TestResult::Error`] from the given value, with smaller is
    /// better semantics.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResult, Error};
    /// assert_eq!(
    ///     TestResult::<i32, i32>::error(5),
    ///     TestResult::Error(Error(5))
    /// );
    /// ```
    #[must_use]
    pub const fn error(error: E) -> Self {
        Self::Error(Error(error))
    }
}

impl<S: Default, E> Default for TestResult<S, E> {
    /// Create a new [`TestResult`], defaulting to
    /// [`TestResult::Score(Score::default())`](TestResult::Score)
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::TestResult;
    /// assert_eq!(TestResult::<i32, i32>::default(), TestResult::score(0));
    /// ```
    fn default() -> Self {
        Self::Score(Score::default())
    }
}

impl<S: PartialOrd, E: PartialOrd> PartialOrd for TestResult<S, E> {
    /// Compare two [`TestResult`].
    ///
    /// - [`TestResult::Score`] and [`TestResult::Error`] are not comparable and
    ///   will always return [`None`].
    /// - Two [`TestResult::Score`]'s will be compared by comparing the inner
    ///   [`Score<S>`](Score) values.
    /// - Two [`TestResult::Error`]'s will be compared by comparing the inner
    ///   [`Error<E>`](Error) values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::TestResult;
    /// # use std::cmp::Ordering;
    /// assert_eq!(
    ///     TestResult::<i32, i32>::score(5).partial_cmp(&TestResult::<i32, i32>::score(10)),
    ///     Some(Ordering::Less),
    /// );
    ///
    /// assert_eq!(
    ///     TestResult::<i32, i32>::error(5).partial_cmp(&TestResult::<i32, i32>::error(10)),
    ///     Some(Ordering::Greater),
    /// );
    ///
    /// assert_eq!(
    ///     TestResult::<i32, i32>::score(5).partial_cmp(&TestResult::<i32, i32>::error(10)),
    ///     None,
    /// );
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
mod test_result_tests {
    use std::ops::Not;

    use super::*;

    #[test]
    fn score_compares_to_score() {
        let first: TestResult<i32, i32> = TestResult::Score(Score(32));
        let second = TestResult::Score(Score(87));
        assert!(first < second);
        assert!(first != second);
        assert!((first > second).not());
    }

    #[test]
    fn error_compares_to_error() {
        let first: TestResult<i32, i32> = TestResult::Error(Error(32));
        let second = TestResult::Error(Error(87));
        assert!(first > second);
        assert!(first != second);
        assert!((first < second).not());
    }

    #[test]
    fn error_and_score_incomparable() {
        let first = TestResult::Score(Score(32));
        let second = TestResult::Error(Error(87));
        assert!((first > second).not());
        assert!(first != second);
        assert!((first < second).not());
        assert!(first.partial_cmp(&second).is_none());
        assert!(second.partial_cmp(&first).is_none());
    }
}

/// A collection of results for multiple test cases, keeping track of the total
/// result.
///
/// # Example
/// ```
/// # use ec_core::test_results::{TestResults, Score};
/// let results = [Score(5), Score(10)]
///     .into_iter()
///     .collect::<TestResults<_>>();
///
/// assert_eq!(results, Score(15));
/// ```
// These are purposefully not pub fields to avoid breaking the invariants of this type.
//
// TODO:
// We should think again about the .total() getters if they should return options, returning None if
// `results.is_empty()`, since if we don't do that a empty `TestResults<Error<i32>>` collection
// would for example return a Error(0) which is probably not what was intended.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[non_exhaustive]
pub struct TestResults<R> {
    results: Vec<R>,
    total_result: Option<R>,
}

// We need `TestResults` to be cloneable in many of our applications,
// even if it's not needed here in `ec_core`. For `TestResults` to be
// cloneable, the generic type must also be cloneable.
static_assertions::assert_impl_all!(TestResults<()>: Clone);

impl<R> TestResults<R> {
    /// Get the number of test results stored in this collection
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(results.len(), 2);
    /// ```
    #[must_use]
    pub const fn len(&self) -> usize {
        self.results.len()
    }

    /// Check if this collection is empty
    ///
    /// # Examples
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [0i32; 0].into_iter().collect::<TestResults<Score<i32>>>();
    ///
    /// assert!(results.is_empty());
    /// ```
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert!(!results.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Get a reference to the total result calculated from all the test results
    /// in this collection.
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(results.total(), Some(&Score(30)));
    /// ```
    #[must_use]
    pub const fn total(&self) -> Option<&R> {
        self.total_result.as_ref()
    }

    /// Get the total result calculated from all the test results
    /// in this collection, discarding the individual test results
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(results.into_total(), Some(Score(30)));
    /// ```
    #[must_use]
    pub fn into_total(self) -> Option<R> {
        self.total_result
    }

    /// Get a reference to a single test result in this collection
    ///
    /// Returns [`None`] if the index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(results.get(1), Some(&Score(20)));
    /// ```
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(results.get(10), None);
    /// ```
    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[R]>,
    {
        self.results.get(index)
    }

    /// Iterate over references to the test results in this collection
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert!(results.iter().eq([&Score(10), &Score(20)]));
    /// ```
    #[must_use]
    pub fn iter(&self) -> <&'_ Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    // TODO: For now no iter_mut() and get_mut() functions since those would need to
    // modify the total result after they are called and are thus more difficult to
    // implement (although certainly possible as well)
}

impl<R, I> Index<I> for TestResults<R>
where
    I: SliceIndex<[R]>,
{
    type Output = I::Output;

    /// Get a reference to a single test result in this collection
    ///
    /// # Panics
    /// When the index is out of bounds for this collection
    ///
    /// # Examples
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(results.get(1), Some(&Score(20)));
    /// ```
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(results.get(10), None);
    /// ```
    fn index(&self, index: I) -> &Self::Output {
        &self.results[index]
    }
}

impl<R: Ord> Ord for TestResults<R> {
    /// Compare multiple [`TestResults`] collections by their total result.
    ///
    /// # Examples
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let first_results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// let second_results = [Score(1), Score(2), Score(6), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert!(first_results > second_results);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_result.cmp(&other.total_result)
    }
}

impl<R: PartialOrd> PartialOrd for TestResults<R> {
    /// Compare multiple [`TestResults`] collections by their total result.
    ///
    /// # Examples
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// # use std::cmp::Ordering;
    /// let first_results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// let second_results = [Score(1), Score(2), Score(6), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(
    ///     first_results.partial_cmp(&second_results),
    ///     Some(Ordering::Greater)
    /// );
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_result.partial_cmp(&other.total_result)
    }
}

impl<R: PartialOrd> PartialOrd<R> for TestResults<R> {
    /// Compare a [`TestResults`] collection with a single `R` result by the
    /// total result.
    ///
    /// # Examples
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// # use std::cmp::Ordering;
    /// let first_results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(
    ///     first_results.partial_cmp(&Score(5)),
    ///     Some(Ordering::Greater)
    /// );
    /// ```
    /// ```
    /// # use ec_core::test_results::{TestResults, Error};
    /// # use std::cmp::Ordering;
    /// let first_results = [Error(10), Error(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Error<i32>>>();
    ///
    /// assert_eq!(
    ///     first_results.partial_cmp(&Error(100)),
    ///     Some(Ordering::Greater)
    /// );
    /// ```
    /// ```
    /// # use ec_core::test_results::{TestResults, Error};
    /// # use std::cmp::Ordering;
    /// let first_results = [Error(10), Error(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Error<i32>>>();
    ///
    /// assert_eq!(first_results.partial_cmp(&Error(0)), Some(Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &R) -> Option<Ordering> {
        self.total_result
            .as_ref()
            .and_then(|result| result.partial_cmp(other))
    }
}

impl<R: PartialEq> PartialEq<R> for TestResults<R> {
    /// Compare a [`TestResults`] collection with a single `R` result for
    /// equality by the total result.
    ///
    /// # Examples
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let first_results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(first_results, Score(30));
    /// ```
    /// ```
    /// # use ec_core::test_results::{TestResults, Error};
    /// let first_results = [Error(10), Error(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Error<i32>>>();
    ///
    /// assert_eq!(first_results, Error(30));
    /// ```
    /// ```
    /// # use ec_core::test_results::{TestResults, Error};
    /// let first_results = [Error(10), Error(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Error<i32>>>();
    ///
    /// assert_ne!(first_results, Error(0));
    /// ```
    fn eq(&self, other: &R) -> bool {
        self.total_result
            .as_ref()
            .is_some_and(|result| result.eq(other))
    }
}

impl<R: Display> Display for TestResults<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.total_result.as_ref() {
            Some(result) => {
                write!(f, "Total result: {result}")
            }
            None => f.write_str("Total result: N/A (empty result set)"),
        }
    }
}

impl<R> Default for TestResults<R> {
    /// Create a new empty [`TestResults`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// assert!(
    ///     TestResults::<Score<i32>>::default()
    ///         .iter()
    ///         .eq::<[&Score<_>; 0]>([])
    /// );
    /// assert_eq!(TestResults::<Score<i32>>::default().total(), None);
    /// ```
    fn default() -> Self {
        Self {
            results: vec![],
            total_result: None,
        }
    }
}

/*
 * I can't implement `From` for both a `Vec` and an `Iterator` because there
 * are potentially conflicting implementations then. (The reasons are a bit
 * complex, but essentially [I think] someone could implement `Iterator` for
 * `Vec` upstream, and then we wouldn't know which implementation to use
 * here.) I _think_ it makes more sense to keep the `Iterator` one since it's
 * cheap to go from `Vec` to `Iterator`, but "expensive" (we have to do an
 * allocation) to go the other way around. Also, we'll often build our list
 * of values with an iterator, and then we just have to add `.into()` at the
 * end instead of converting into a `Vec` first.
 */

impl<V, R> FromIterator<V> for TestResults<R>
where
    // TODO: I would really reconsider the from here, since
    // it messes up type inference a lot, and I would
    // say that then a .map(Into::into) can just be
    // done before the .collect() call.
    for<'a> R: From<V> + Sum<&'a R> + 'a,
{
    /// Create a new [`TestResults`] from a iterator of test results,
    /// summing the test results to get the total result.
    /// # Examples
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = TestResults::<Score<i32>>::from_iter([Score(10), Score(20)]);
    ///
    /// assert_eq!(results.total(), Some(&Score(30)));
    /// ```
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert_eq!(results.total(), Some(&Score(30)));
    /// ```
    fn from_iter<T: IntoIterator<Item = V>>(values: T) -> Self {
        let results: Vec<R> = values.into_iter().map(Into::into).collect();
        let total_result = (!results.is_empty()).then(|| results.iter().sum());
        Self {
            results,
            total_result,
        }
    }
}

impl<R> IntoIterator for TestResults<R> {
    type Item = R;

    type IntoIter = <Vec<R> as IntoIterator>::IntoIter;

    /// Turn this collection into a iterator over the individual results,
    /// discarding the calculated total result
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// assert!(results.into_iter().eq([Score(10), Score(20)]));
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

impl<'a, R> IntoIterator for &'a TestResults<R> {
    type Item = &'a R;

    type IntoIter = <&'a Vec<R> as IntoIterator>::IntoIter;

    /// Iterate over references to the test results in this collection
    ///
    /// # Example
    /// ```
    /// # use ec_core::test_results::{TestResults, Score};
    /// let results = [Score(10), Score(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<Score<i32>>>();
    ///
    /// let results_reference = &results;
    ///
    /// assert!(results_reference.into_iter().eq([&Score(10), &Score(20)]));
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.results.iter()
    }
}

#[cfg(test)]
mod test_results_tests {
    use super::*;

    #[test]
    fn create_test_results_from_errors() {
        let errors = [5, 8, 0, 9];
        let test_results: TestResults<Error<i32>> = errors.into_iter().collect();
        assert!(test_results.results.iter().map(|r| r.0).eq(errors));
        assert_eq!(test_results.total_result, Some(errors.into_iter().sum()));
    }

    #[test]
    fn create_test_results_from_scores() {
        let scores = [5, 8, 0, 9];
        let test_results: TestResults<Score<i32>> = scores.into_iter().collect();
        assert!(test_results.results.iter().map(|r| r.0).eq(scores));
        assert_eq!(test_results.total_result, Some(scores.into_iter().sum()));
    }

    #[test]
    fn create_test_results_from_iter_errors() {
        let errors = [5, 8, 0, 9];
        let results = errors.iter().copied().map(Error::from);
        let test_results: TestResults<Error<i32>> = results.clone().collect();
        assert!(test_results.results.into_iter().eq(results));
        assert_eq!(test_results.total_result, Some(errors.into_iter().sum()));
    }

    #[test]
    fn create_test_results_from_iter_scores() {
        let scores = [5, 8, 0, 9];
        let results = scores.iter().copied().map(Score::from);
        let test_results: TestResults<Score<i32>> = results.clone().collect();
        assert!(test_results.results.into_iter().eq(results));
        assert_eq!(test_results.total_result, Some(scores.into_iter().sum()));
    }
}
