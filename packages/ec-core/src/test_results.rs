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
    pub total_result: R,
    pub results: Vec<R>,
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

// TODO: Write tests for the `Sum` trait implementations.

// TODO: Consider going from the `Sum` trait to the
//   `FromIterator` trait and `.collect()`. TestResults
//   aren't really the _sum_ of a a collection of `R`,
//   but more built out of one.
impl<R: Sum + Copy> Sum<R> for TestResults<R> {
    fn sum<I: Iterator<Item = R>>(iter: I) -> Self {
        let results: Vec<R> = iter.collect();
        let total_result: R = results.iter().copied().sum();
        Self {
            total_result,
            results,
        }
    }
}

impl<V, R> From<Vec<V>> for TestResults<R>
where
    R: Copy + Sum + From<V>,
{
    fn from(values: Vec<V>) -> Self {
        values.into_iter().map(From::from).sum()
    }
}
