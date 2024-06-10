use std::cmp::Ordering;

use super::{error, score};

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
pub enum TestResult<S, E> {
    Score(score::ScoreValue<S>),
    Error(error::ErrorValue<E>),
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
        let first: TestResult<i32, i32> = TestResult::Score(score::ScoreValue { score: 32 });
        let second = TestResult::Score(score::ScoreValue { score: 87 });
        assert!(first < second);
        assert!(first != second);
        assert!((first > second).not());
    }

    #[test]
    fn error_compares_to_error() {
        let first: TestResult<i32, i32> = TestResult::Error(error::ErrorValue(32));
        let second = TestResult::Error(error::ErrorValue(87));
        assert!(first > second);
        assert!(first != second);
        assert!((first < second).not());
    }

    #[test]
    fn error_and_score_incomparable() {
        let first = TestResult::Score(score::ScoreValue { score: 32 });
        let second = TestResult::Error(error::ErrorValue(87));
        assert!((first > second).not());
        assert!(first != second);
        assert!((first < second).not());
        assert!(first.partial_cmp(&second).is_none());
        assert!(second.partial_cmp(&first).is_none());
    }
}
