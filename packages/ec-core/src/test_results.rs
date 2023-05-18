use std::{cmp::Ordering, fmt::Debug, iter::Sum};

// TODO: We can probably use things in the `num` family of traits
//   (https://github.com/rust-num/num) to genericize `Score` and
//   `Error` so they're not tied to `i64`s anymore.
// TODO: I think that we want `Score` and `Error` to implement
//   some common trait so that we can mixed vectors of `Score`s
//   and `Error`s. Or maybe we already have that? Do (Partial)Ord,
//   (Partial)Eq, Ord, and Sum get us where we need to be? That's
//   lot to keep track of, so it might be useful to have a named
//   trait that has all those as super-traits so we have one name
//   that pulls them all together.

// TODO: Should there just be one struct (e.g., `Result<T>` with a `result: T` field)
//   and then `Error` and `Score` should be traits that these structs can
//   implement? I feel like that might avoid some duplication here.

/// Score implicitly follows a "bigger is better" model.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Score {
    pub score: i64,
}

impl Debug for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.score))
    }
}

// TODO: Write tests for the `From` and `Sum` trait implementations.

impl From<i64> for Score {
    fn from(score: i64) -> Self {
        Self { score }
    }
}

impl From<&i64> for Score {
    fn from(score: &i64) -> Self {
        (*score).into()
    }
}

impl Sum<i64> for Score {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = i64>,
    {
        Self { score: iter.sum() }
    }
}

impl Sum for Score {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.score).sum()
    }
}

impl<'a> Sum<&'a Self> for Score {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.score).sum()
    }
}

// TODO: This (and score) should probably be generic so we can
//   use `i64` or `i28` or unsigned values, etc.
/// Error implicitly follows a "smaller is better" model
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Error {
    pub error: i64,
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.error))
    }
}

impl Ord for Error {
    fn cmp(&self, other: &Self) -> Ordering {
        self.error.cmp(&other.error).reverse()
    }
}

impl PartialOrd for Error {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// TODO: Write tests for the `From` and `Sum` trait implementations.

impl From<i64> for Error {
    fn from(error: i64) -> Self {
        Self { error }
    }
}

impl From<&i64> for Error {
    fn from(error: &i64) -> Self {
        (*error).into()
    }
}

impl Sum<i64> for Error {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = i64>,
    {
        Self { error: iter.sum() }
    }
}

impl Sum for Error {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.error).sum()
    }
}

impl<'a> Sum<&'a Self> for Error {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.error).sum()
    }
}

#[cfg(test)]
mod score_error_tests {
    use std::ops::Not;

    use super::*;

    #[test]
    fn score_bigger_is_better() {
        let first = Score { score: 37 };
        let second = Score { score: 82 };
        assert!(first < second);
        assert!(first != second);
        assert!((first > second).not());
    }

    #[test]
    fn error_smaller_is_better() {
        let first = Error { error: 37 };
        let second = Error { error: 82 };
        assert!(first > second);
        assert!(first != second);
        assert!((first < second).not());
    }
}

// type I64Error = Error<i64>;

#[derive(Eq, PartialEq)]
pub enum TestResult {
    Score(Score),
    Error(Error),
}

impl PartialOrd for TestResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Score(self_score), Self::Score(other_score)) => {
                Some(self_score.cmp(other_score))
            }
            (Self::Error(self_error), Self::Error(other_error)) => {
                Some(self_error.cmp(other_error))
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
        let first = TestResult::Score(Score { score: 32 });
        let second = TestResult::Score(Score { score: 87 });
        assert!(first < second);
        assert!(first != second);
        assert!((first > second).not());
    }

    #[test]
    fn error_compares_to_error() {
        let first = TestResult::Error(Error { error: 32 });
        let second = TestResult::Error(Error { error: 87 });
        assert!(first > second);
        assert!(first != second);
        assert!((first < second).not());
    }

    #[test]
    fn error_and_score_incomparable() {
        let first = TestResult::Score(Score { score: 32 });
        let second = TestResult::Error(Error { error: 87 });
        assert!((first > second).not());
        assert!(first != second);
        assert!((first < second).not());
        assert!(first.partial_cmp(&second).is_none());
        assert!(second.partial_cmp(&first).is_none());
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct TestResults<R> {
    pub results: Vec<R>,
    pub total_result: R,
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

/*
 * I can't implement `From` for both a `Vec` and an `Iterator` because there are
 * potentially conflicting implementations then. (The reasons are a bit complex,
 * but essentially [I think] someone could implement `Iterator` for `Vec` upstream,
 * and then we wouldn't know which implementation to use here.) I _think_ it makes
 * more sense to keep the `Iterator` one since it's cheap to go from `Vec` to `Iterator`,
 * but "expensive" (we have to do an allocation) to go the other way around. Also, we'll
 * often build our list of values with an iterator, and then we just have to add
 * `.into()` at the end instead of converting into a `Vec` first.
 */

// impl<V, R> From<Vec<V>> for TestResults<R>
// where
//     R: From<V> + Copy + Sum,
// {
//     fn from(values: Vec<V>) -> Self {
//         let results: Vec<R> = values.into_iter().map(Into::into).collect();
//         let total_result: R = results.iter().copied().sum();
//         Self {
//             results,
//             total_result
//         }
//     }
// }

impl<I, V, R> From<I> for TestResults<R>
where
    for<'a> R: From<V> + Sum<&'a R> + 'a,
    I: IntoIterator<Item = V>,
{
    fn from(values: I) -> Self {
        let results: Vec<R> = values.into_iter().map(Into::into).collect();
        let total_result = results.iter().sum();
        Self {
            results,
            total_result,
        }
    }
}

impl<V, R> FromIterator<V> for TestResults<R>
where
    for<'a> R: From<V> + Copy + Sum<&'a R> + 'a,
{
    fn from_iter<T: IntoIterator<Item = V>>(values: T) -> Self {
        values.into()
    }
}

#[cfg(test)]
mod test_results_from_vec {
    use super::*;

    #[test]
    fn create_test_results_from_errors() {
        let errors = vec![5, 8, 0, 9];
        let test_results: TestResults<Error> = errors.clone().into();
        assert_eq!(test_results.results.iter().map(|r| r.error).collect::<Vec<_>>(), errors);
        assert_eq!(test_results.total_result, errors.into_iter().sum());
    }

    #[test]
    fn create_test_results_from_scores() {
        let scores = vec![5, 8, 0, 9];
        let test_results: TestResults<Score> = scores.clone().into();
        assert_eq!(test_results.results.iter().map(|r| r.score).collect::<Vec<_>>(), scores);
        assert_eq!(test_results.total_result, scores.into_iter().sum());
    }

    #[test]
    fn create_test_results_from_iter_errors() {
        let errors = vec![5, 8, 0, 9];
        let results = errors.iter().map(Into::<Error>::into);
        let test_results: TestResults<Error> = results.clone().collect();
        assert_eq!(test_results.results, results.collect::<Vec<_>>());
        assert_eq!(test_results.total_result, errors.into_iter().sum());
    }

    #[test]
    fn create_test_results_from_iter_scores() {
        let scores = vec![5, 8, 0, 9];
        let results = scores.iter().map(Into::<Score>::into);
        let test_results: TestResults<Score> = results.clone().collect();
        assert_eq!(test_results.results, results.collect::<Vec<_>>());
        assert_eq!(test_results.total_result, scores.into_iter().sum());
    }
}
