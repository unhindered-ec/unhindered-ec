use std::{borrow::Borrow, cmp::Ordering};

use rand::rngs::ThreadRng;

/// Score implicitly follows a "bigger is better" model.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Score {
    pub score: i64
}

// type I64Score = Score<i64>;

/// Error implicitly follows a "smaller is better" model
#[derive(Debug, Eq, PartialEq)]
pub struct Error {
    pub error: i64
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

#[cfg(test)]
mod score_error_tests {
    use super::*;

    #[test]
    fn score_bigger_is_better() {
        let first = Score { score: 37 };
        let second = Score { score: 82 };
        assert!(first < second);
        assert!(first != second);
        assert!(!(first > second));
    }

    #[test]
    fn error_smaller_is_better() {
        let first = Error { error: 37 };
        let second = Error { error: 82 };
        assert!(first > second);
        assert!(first != second);
        assert!(!(first < second));
    }
}

// type I64Error = Error<i64>;

#[derive(Eq, PartialEq)]
pub enum TestResult {
    Score(Score),
    Error(Error)
}

impl PartialOrd for TestResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Score(self_score), Self::Score(other_score)) 
                => Some(self_score.cmp(other_score)),
            (Self::Error(self_error), Self::Error(other_error))
                => Some(self_error.cmp(other_error)),
            _ => None
        }
    }
}

#[cfg(test)]
mod test_result_tests {
    use super::*;

    #[test]
    fn score_compares_to_score() {
        let first = TestResult::Score(Score { score: 32 });
        let second = TestResult::Score(Score { score: 87 });
        assert!(first < second);
        assert!(first != second);
        assert!(!(first > second));
    }

    #[test]
    fn error_compares_to_error() {
        let first = TestResult::Error(Error { error: 32 });
        let second = TestResult::Error(Error { error: 87 });
        assert!(first > second);
        assert!(first != second);
        assert!(!(first < second));
    }

    #[test]
    fn error_and_score_incomparable() {
        let first = TestResult::Score(Score { score: 32 });
        let second = TestResult::Error(Error { error: 87 });
        assert!(!(first > second));
        assert!(first != second);
        assert!(!(first < second));
        assert!(first.partial_cmp(&second).is_none());
        assert!(second.partial_cmp(&first).is_none());
    }

}

#[derive(Debug, Eq, PartialEq)]
pub struct TestResults<R> {
    pub total_result: R,
    pub results: Vec<R>
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Individual<G, R> {
    pub genome: G,
    pub test_results: R,
    // pub total_score: i64,
    // pub scores: Vec<i64>,
}

impl<G, R> Individual<G, R> {
    /*
     * The type `R` is needed for circumstances where `T` is a "costly"
     * (to quote the documentation for the `Borrow` trait) type like
     * `Vec<bool>` when a "cheaper" type like `[bool]` would do. We might,
     * for example, prefer to have `compute_score` take a type like `&[bool]`,
     * but have everything written in terms of a more general (and "expensive")
     * type like `Vec<bool>`. If we use `Vec<bool>` for `T`, but specify
     * `compute_score` to take `&[bool]`, then the type checker won't be able
     * to link those things up.
     * 
     * The use of `R` fixes that. Saying `T: Borrow<R>` says that `T` (e.g.,
     * `Vec<bool>`) can be borrowed as a reference to the simpler type (e.g.,
     * `[bool]`). So we can use `Vec<bool>` as our "general" type, but this
     * allows the system to know that it can convert (through borrowing) instances
     * of that to `[bool]`. Thus `compute_score` can now take `&[bool]` as an
     * argument and the types will work out.
     * 
     * The `R: ?Sized` comes from the definition of the `Borrow` trait and is
     * necessary to say that `R` doesn't necessarily have a size that is known
     * at compile time. This is important because we're borrowing from `Vec<bool>`
     * (which has a known size) to `[bool]` (whose size depends on how many items
     * there are in the array, i.e., it's not known at compile time). Type generics
     * are assumed by default to be `Sized`, but we can make that optional with the
     * question mark `?Sized`.
     * 
     * The idea for this came from @scottmcm's answer to a question on the 
     * Rust user forum:
     * https://users.rust-lang.org/t/problem-passing-functions-as-arguments-and-deref/79491/2?u=nicmcphee
     * The documentation for the `Borrow` trait was very helpful: 
     * https://doc.rust-lang.org/std/borrow/trait.Borrow.html
     */
    pub fn new<H>(
            make_genome: impl Fn(&mut ThreadRng) -> G,
            // TODO: Should this be a special EC-specific trait instead of the general `Fn`?
            run_tests: impl Fn(&H) -> R,
            rng: &mut ThreadRng) 
        -> Self
    where
        G: Borrow<H>,
        H: ?Sized
    {
        let genome = make_genome(rng);
        let test_results = run_tests(genome.borrow());
        Self {
            genome,
            test_results
        }
    }
}

impl<G: Eq, R: Ord> Ord for Individual<G, R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.test_results.cmp(&other.test_results)
    }
}

impl<G: PartialEq, R: PartialOrd> PartialOrd for Individual<G, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.test_results.partial_cmp(&other.test_results)
    }
}
