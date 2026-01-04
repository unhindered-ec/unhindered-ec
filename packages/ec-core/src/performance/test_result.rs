use std::cmp::Ordering;

use crate::performance::{error_value::ErrorValue, score_value::ScoreValue};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum TestResult<S, E> {
    Score(ScoreValue<S>),
    Error(ErrorValue<E>),
}

impl<S, E> From<ScoreValue<S>> for TestResult<S, E> {
    fn from(value: ScoreValue<S>) -> Self {
        Self::Score(value)
    }
}

impl<S, E> From<ErrorValue<E>> for TestResult<S, E> {
    fn from(value: ErrorValue<E>) -> Self {
        Self::Error(value)
    }
}

impl<S, E> TestResult<S, E> {
    #[must_use]
    pub const fn score(score: S) -> Self {
        Self::Score(ScoreValue(score))
    }

    #[must_use]
    pub const fn error(error: E) -> Self {
        Self::Error(ErrorValue(error))
    }
}

impl<S: Default, E> Default for TestResult<S, E> {
    fn default() -> Self {
        Self::Score(ScoreValue::default())
    }
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
