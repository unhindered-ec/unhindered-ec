use std::{cmp::Ordering, fmt::Display, iter::Sum};

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
pub(crate) mod test_results_from_vec {
    use super::*;
    use crate::performance::{error::ErrorValue, score::ScoreValue};

    #[test]
    fn create_test_results_from_errors() {
        let errors = vec![5, 8, 0, 9];
        let test_results: TestResults<ErrorValue<i32>> = errors.clone().into();
        assert_eq!(
            test_results.results.iter().map(|r| r.0).collect::<Vec<_>>(),
            errors
        );
        assert_eq!(test_results.total_result, errors.into_iter().sum::<i32>());
    }

    #[test]
    fn create_test_results_from_scores() {
        let scores = vec![5, 8, 0, 9];
        let test_results: TestResults<ScoreValue<i32>> = scores.clone().into();
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
        let results = errors.iter().copied().map(ErrorValue::from);
        let test_results: TestResults<ErrorValue<i32>> = results.clone().collect();
        assert_eq!(test_results.results, results.collect::<Vec<_>>());
        assert_eq!(test_results.total_result, errors.into_iter().sum::<i32>());
    }

    #[test]
    fn create_test_results_from_iter_scores() {
        let scores = vec![5, 8, 0, 9];
        let results = scores.iter().copied().map(ScoreValue::from);
        let test_results: TestResults<ScoreValue<i32>> = results.clone().collect();
        assert_eq!(test_results.results, results.collect::<Vec<_>>());
        assert_eq!(test_results.total_result, scores.into_iter().sum());
    }
}
