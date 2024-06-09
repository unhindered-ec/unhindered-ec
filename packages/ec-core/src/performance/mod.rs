use std::{cmp::Ordering, fmt::Display, iter::Sum};

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

pub mod error;
pub mod score;

mod test_result;

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
        let test_results: TestResults<error::ErrorValue<i32>> = errors.clone().into();
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
        let test_results: TestResults<score::ScoreValue<i32>> = scores.clone().into();
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
        let results = errors.iter().copied().map(error::ErrorValue::from);
        let test_results: TestResults<error::ErrorValue<i32>> = results.clone().collect();
        assert_eq!(test_results.results, results.collect::<Vec<_>>());
        assert_eq!(test_results.total_result, errors.into_iter().sum());
    }

    #[test]
    fn create_test_results_from_iter_scores() {
        let scores = vec![5, 8, 0, 9];
        let results = scores.iter().copied().map(score::ScoreValue::from);
        let test_results: TestResults<score::ScoreValue<i32>> = results.clone().collect();
        assert_eq!(test_results.results, results.collect::<Vec<_>>());
        assert_eq!(test_results.total_result, scores.into_iter().sum());
    }
}
