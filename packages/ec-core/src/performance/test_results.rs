use std::{cmp::Ordering, fmt::Display, iter::Sum, ops::Index, slice::SliceIndex};

/// A collection of results for multiple test cases, keeping track of the total
/// result.
///
/// # Example
/// ```
/// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
/// let results = [ScoreValue(5), ScoreValue(10)]
///     .into_iter()
///     .collect::<TestResults<_>>();
///
/// assert_eq!(results.total(), Some(&ScoreValue(15)));
/// ```
// These are purposefully not pub fields to avoid breaking the invariants of this type.
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
#[non_exhaustive]
pub struct TestResults<R> {
    results: Vec<R>,
    /// This is an `Option` since the above collection may be empty and as such
    /// we then won't have a total result (and we don't want to assume some
    /// default for `R`.).
    // Invariant: if the collection above is non-empty,
    // this is Some(R).
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [0i32; 0]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert!(results.is_empty());
    /// ```
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
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
    /// Returns [`None`] when the collection is empty.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert_eq!(results.total(), Some(&ScoreValue(30)));
    /// ```
    #[must_use]
    pub const fn total(&self) -> Option<&R> {
        self.total_result.as_ref()
    }

    /// Get the total result calculated from all the test results
    /// in this collection, discarding the individual test results
    ///
    /// Returns [`None`] when the collection is empty.
    ///
    /// # Example
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert_eq!(results.into_total(), Some(ScoreValue(30)));
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert_eq!(results.get(1), Some(&ScoreValue(20)));
    /// ```
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert!(results.iter().eq([&ScoreValue(10), &ScoreValue(20)]));
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

    /// Get a reference to a single test result in this collection.
    ///
    /// See [`TestResults::get`] for a non-panicking alternative.
    ///
    /// # Panics
    /// Panics if index is out of bounds.
    ///
    /// # Examples
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert_eq!(results[1], ScoreValue(20));
    /// ```
    /// ```should_panic
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// dbg!(results[10]); // panics
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let first_results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// let second_results = [ScoreValue(1), ScoreValue(2), ScoreValue(6), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// # use std::cmp::Ordering;
    /// let first_results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// let second_results = [ScoreValue(1), ScoreValue(2), ScoreValue(6), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// # use std::cmp::Ordering;
    /// let first_results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert_eq!(
    ///     first_results.partial_cmp(&ScoreValue(5)),
    ///     Some(Ordering::Greater)
    /// );
    /// ```
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, error_value::ErrorValue};
    /// # use std::cmp::Ordering;
    /// let first_results = [ErrorValue(10), ErrorValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ErrorValue<i32>>>();
    ///
    /// assert_eq!(
    ///     first_results.partial_cmp(&ErrorValue(100)),
    ///     Some(Ordering::Greater)
    /// );
    /// ```
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, error_value::ErrorValue};
    /// # use std::cmp::Ordering;
    /// let first_results = [ErrorValue(10), ErrorValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ErrorValue<i32>>>();
    ///
    /// assert_eq!(
    ///     first_results.partial_cmp(&ErrorValue(0)),
    ///     Some(Ordering::Less)
    /// );
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let first_results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert_eq!(first_results, ScoreValue(30));
    /// ```
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, error_value::ErrorValue};
    /// let first_results = [ErrorValue(10), ErrorValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ErrorValue<i32>>>();
    ///
    /// assert_eq!(first_results, ErrorValue(30));
    /// ```
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, error_value::ErrorValue};
    /// let first_results = [ErrorValue(10), ErrorValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ErrorValue<i32>>>();
    ///
    /// assert_ne!(first_results, ErrorValue(0));
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// assert!(
    ///     TestResults::<ScoreValue<i32>>::default()
    ///         .iter()
    ///         .eq::<[&ScoreValue<_>; 0]>([])
    /// );
    /// assert_eq!(TestResults::<ScoreValue<i32>>::default().total(), None);
    /// ```
    fn default() -> Self {
        Self {
            results: vec![],
            total_result: None,
        }
    }
}

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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = TestResults::<ScoreValue<i32>>::from_iter([ScoreValue(10), ScoreValue(20)]);
    ///
    /// assert_eq!(results.total(), Some(&ScoreValue(30)));
    /// ```
    /// ```
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert_eq!(results.total(), Some(&ScoreValue(30)));
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// assert!(results.into_iter().eq([ScoreValue(10), ScoreValue(20)]));
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
    /// # use ec_core::performance::{test_results::TestResults, score_value::ScoreValue};
    /// let results = [ScoreValue(10), ScoreValue(20)]
    ///     .into_iter()
    ///     .collect::<TestResults<ScoreValue<i32>>>();
    ///
    /// let results_reference = &results;
    ///
    /// assert!(
    ///     results_reference
    ///         .into_iter()
    ///         .eq([&ScoreValue(10), &ScoreValue(20)])
    /// );
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.results.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::performance::{error_value::ErrorValue, score_value::ScoreValue};

    #[test]
    fn create_test_results_from_errors() {
        let errors = [5, 8, 0, 9];
        let test_results: TestResults<ErrorValue<i32>> = errors.into_iter().collect();
        assert!(test_results.results.iter().map(|r| r.0).eq(errors));
        assert_eq!(test_results.total_result, Some(errors.into_iter().sum()));
    }

    #[test]
    fn create_test_results_from_scores() {
        let scores = [5, 8, 0, 9];
        let test_results: TestResults<ScoreValue<i32>> = scores.into_iter().collect();
        assert!(test_results.results.iter().map(|r| r.0).eq(scores));
        assert_eq!(test_results.total_result, Some(scores.into_iter().sum()));
    }

    #[test]
    fn create_test_results_from_iter_errors() {
        let errors = [5, 8, 0, 9];
        let results = errors.iter().copied().map(ErrorValue::from);
        let test_results: TestResults<ErrorValue<i32>> = results.clone().collect();
        assert!(test_results.results.into_iter().eq(results));
        assert_eq!(test_results.total_result, Some(errors.into_iter().sum()));
    }

    #[test]
    fn create_test_results_from_iter_scores() {
        let scores = [5, 8, 0, 9];
        let results = scores.iter().copied().map(ScoreValue::from);
        let test_results: TestResults<ScoreValue<i32>> = results.clone().collect();
        assert!(test_results.results.into_iter().eq(results));
        assert_eq!(test_results.total_result, Some(scores.into_iter().sum()));
    }
}
