use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
    ops::Index,
    slice::SliceIndex,
};

use error_value::ErrorValue;
use score_value::ScoreValue;

// TODO: We can probably use things in the `num` family of traits
//   (https://github.com/rust-num/num) to genericize `Score` and
//   `Error` so they're not tied to `i64`s anymore.

pub mod error_value;
pub mod score_value;
// I don't think we ever _use_ `TestResult` (singular), so maybe it goes away.
//mod test_result;

// TODO: Should there just be one struct (e.g., `Result<T>` with a `result: T`
// field)   and then `Error` and `Score` should be traits that these structs can
//   implement? I feel like that might avoid some duplication here.

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
    /// Get the number of test results
    #[must_use]
    pub const fn len(&self) -> usize {
        self.results.len()
    }

    /// Check if no test results were stored
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    #[must_use]
    pub const fn total(&self) -> Option<&R> {
        self.total_result.as_ref()
    }

    #[must_use]
    pub fn into_total(self) -> Option<R> {
        self.total_result
    }

    #[must_use]
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[R]>,
    {
        self.results.get(index)
    }

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

    fn index(&self, index: I) -> &Self::Output {
        &self.results[index]
    }
}

impl<R: Ord> Ord for TestResults<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_result.cmp(&other.total_result)
    }
}

impl<R: PartialOrd> PartialOrd for TestResults<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_result.partial_cmp(&other.total_result)
    }
}

impl<R: PartialOrd> PartialOrd<R> for TestResults<R> {
    fn partial_cmp(&self, other: &R) -> Option<Ordering> {
        self.total_result
            .as_ref()
            .and_then(|result| result.partial_cmp(other))
    }
}

impl<R: PartialEq> PartialEq<R> for TestResults<R> {
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
    /// # use ec_core::performance::{TestResults, score_value::ScoreValue};
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

    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

impl<'a, R> IntoIterator for &'a TestResults<R> {
    type Item = &'a R;

    type IntoIter = <&'a Vec<R> as IntoIterator>::IntoIter;

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
