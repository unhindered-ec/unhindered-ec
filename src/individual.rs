#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::{borrow::Borrow, cmp::Ordering};

use rand::rngs::ThreadRng;

trait ScoreTrait {
    type Value: Ord;
    fn next(&mut self) -> Option<Self::Item>;
}

pub enum TestResult<V> {
    Score(Score<V>),
    Error(Error<V>)
}

impl<V> PartialOrd for TestResult<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Score(self_score), Self::Score(other_score)) 
                => Some(self_score.cmp(other_score)),
            (Self::Error(self_error), Self::Error(other_error))
                => Some(self_error.cmp(&other_error)),
            _ => None
        }
    }
}

/// Score implicitly follows a "bigger is better" model.
#[derive(Ord)]
pub struct Score<V: Ord> {
    pub score: V
}

type I64Score = Score<i64>;

/// Error implicitly follows a "smaller is better" model
pub struct Error<V: Ord> {
    pub error: V
}

type I64Error = Error<i64>;

impl<V> Ord for Error<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.error.cmp(&other.error).reverse()
    }
}

struct TestResults<V> {
    pub totalScore: TestResult<V>,
    pub scores: Vec<TestResult<V>>
}

impl<V> Ord for TestResults<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.totalScore.cmp(&other.error)
    }
}

#[derive(Debug, Clone)]
pub struct Individual<T, S> {
    pub genome: T,
    pub score: S,
    // TODO: Maybe make the score here a new generic type S
    // pub total_score: S,
    // pub scores: Vec<S>
}

impl<T, S: Ord> Ord for Individual<T, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<T, S: PartialOrd> PartialOrd for Individual<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<T, S> Individual<T, S> {
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
    pub fn new<R>(
            make_genome: impl Fn(&mut ThreadRng) -> T, 
            run_tests: impl Fn(&R) -> TestResult<S>,
            rng: &mut ThreadRng) 
        -> Self
    where
        T: Borrow<R>,
        R: ?Sized
    {
        let genome = make_genome(rng);
        let test_results = run_tests(genome.borrow());
        Self {
            genome,
            score: test_results,
            // total_score: scores.iter().sum(),
            // scores
        }
    }
}
