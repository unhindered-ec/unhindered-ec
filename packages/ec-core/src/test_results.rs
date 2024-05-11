use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    iter::Sum,
};

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

// TODO: Should there just be one struct (e.g., `Result<T>` with a `result: T`
// field)   and then `Error` and `Score` should be traits that these structs can
//   implement? I feel like that might avoid some duplication here.

// TODO: I'm not convinced that `Score` & `Error` need `Clone` and `Copy`
//   anymore. At a minimum we should try to push those requirements
//   closer to where they're actually needed.

/// Score implicitly follows a "bigger is better" model.
#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Score<T> {
    pub score: T,
}

impl<T: Debug> Debug for Score<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.score))
    }
}

impl<T: Display> Display for Score<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score (higher is better): {}", self.score)
    }
}

// TODO: Write tests for the `From` and `Sum` trait implementations.

impl<T> From<T> for Score<T> {
    fn from(score: T) -> Self {
        Self { score }
    }
}

impl<T: Sum> Sum<T> for Score<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self { score: iter.sum() }
    }
}

impl<T: Sum> Sum for Score<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.score).sum()
    }
}

impl<'a, T> Sum<&'a Self> for Score<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.score.to_owned()).sum()
    }
}

// TODO: Rewrite `Error` using the std::cmp::Reverse type
//   to convert `Score` to `Error`.
#[derive(Eq, PartialEq)]
pub struct Error<T> {
    pub error: T,
}

impl<T: Debug> Debug for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.error))
    }
}

impl<T: Ord> Ord for Error<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.error.cmp(&other.error).reverse()
    }
}

impl<T: PartialOrd> PartialOrd for Error<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.error
            .partial_cmp(&other.error)
            .map(std::cmp::Ordering::reverse)
    }
}

impl<T: Display> Display for Error<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error (lower is better): {}", self.error)
    }
}
// TODO: Write tests for the `From` and `Sum` trait implementations.

impl<T> From<T> for Error<T> {
    fn from(error: T) -> Self {
        Self { error }
    }
}

impl<T: Sum> Sum<T> for Error<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        Self { error: iter.sum() }
    }
}

impl<T: Sum> Sum for Error<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.map(|s| s.error).sum()
    }
}

impl<'a, T> Sum<&'a Self> for Error<T>
where
    T: ToOwned,
    Self: Sum<<T as ToOwned>::Owned>,
{
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.map(|s| s.error.to_owned()).sum()
    }
}

#[cfg(test)]
mod score_error_tests {
    use super::*;

    #[test]
    fn score_bigger_is_better() {
        let first = Score { score: 37 };
        let second = Score { score: 82 };
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
        let first = Error { error: 37 };
        let second = Error { error: 82 };
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

#[derive(Eq, PartialEq)]
pub enum TestResult<S, E> {
    Score(Score<S>),
    Error(Error<E>),
}

impl<S: PartialOrd, E: PartialOrd> PartialOrd for TestResult<S, E> {
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
        let first: TestResult<i32, i32> = TestResult::Score(Score { score: 32 });
        let second = TestResult::Score(Score { score: 87 });
        assert!(first < second);
        assert!(first != second);
        assert!((first > second).not());
    }

    #[test]
    fn error_compares_to_error() {
        let first: TestResult<i32, i32> = TestResult::Error(Error { error: 32 });
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

impl<R: Display> Display for TestResults<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test result: {}", self.total_result)
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
    for<'a> R: From<V> + Sum<&'a R> + 'a,
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
        let test_results: TestResults<Error<i32>> = errors.clone().into();
        assert_eq!(
            test_results
                .results
                .iter()
                .map(|r| r.error)
                .collect::<Vec<_>>(),
            errors
        );
        assert_eq!(test_results.total_result, errors.into_iter().sum());
    }

    #[test]
    fn create_test_results_from_scores() {
        let scores = vec![5, 8, 0, 9];
        let test_results: TestResults<Score<i32>> = scores.clone().into();
        assert_eq!(
            test_results
                .results
                .iter()
                .map(|r| r.score)
                .collect::<Vec<_>>(),
            scores
        );
        assert_eq!(test_results.total_result, scores.into_iter().sum());
    }

    #[test]
    fn create_test_results_from_iter_errors() {
        let errors = vec![5, 8, 0, 9];
        let results = errors.iter().copied().map(Error::from);
        let test_results: TestResults<Error<i32>> = results.clone().collect();
        assert_eq!(test_results.results, results.collect::<Vec<_>>());
        assert_eq!(test_results.total_result, errors.into_iter().sum());
    }

    #[test]
    fn create_test_results_from_iter_scores() {
        let scores = vec![5, 8, 0, 9];
        let results = scores.iter().copied().map(Score::from);
        let test_results: TestResults<Score<i32>> = results.clone().collect();
        assert_eq!(test_results.results, results.collect::<Vec<_>>());
        assert_eq!(test_results.total_result, scores.into_iter().sum());
    }
}
