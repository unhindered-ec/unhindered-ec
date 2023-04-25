use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{Debug, Display},
};

use rand::rngs::ThreadRng;

use super::{Generate, Individual};

#[derive(Debug, Eq, PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct EcIndividual<G, R> {
    pub genome: G,
    pub test_results: R,
}

impl<G, R> Individual for EcIndividual<G, R> {
    type Genome = G;
    type TestResults = R;

    fn genome(&self) -> &Self::Genome {
        &self.genome
    }

    fn test_results(&self) -> &Self::TestResults {
        &self.test_results
    }
}

impl<G, R> EcIndividual<G, R> {
    pub const fn new(genome: G, test_results: R) -> Self {
        Self {
            genome,
            test_results,
        }
    }
}

impl<G, R> Generate for EcIndividual<G, R> {
    /*
     * The type `H` is needed for circumstances where `G` is a "costly"
     * (to quote the documentation for the `Borrow` trait) type like
     * `Vec<bool>` when a "cheaper" type like `[bool]` would do. We might,
     * for example, prefer to have `compute_score` take a type like `&[bool]`,
     * but have everything written in terms of a more general (and "expensive")
     * type like `Vec<bool>`. If we use `Vec<bool>` for `G`, but specify
     * `compute_score` to take `&[bool]`, then the type checker won't be able
     * to link those things up.
     *
     * The use of `H` fixes that. Saying `G: Borrow<H>` says that `G` (e.g.,
     * `Vec<bool>`) can be borrowed as a reference to the simpler type (e.g.,
     * `[bool]`). So we can use `Vec<bool>` as our "general" type, but this
     * allows the system to know that it can convert (through borrowing) instances
     * of that to `[bool]`. Thus `compute_score` can now take `&[bool]` as an
     * argument and the types will work out.
     *
     * The `H: ?Sized` comes from the definition of the `Borrow` trait and is
     * necessary to say that `H` doesn't necessarily have a size that is known
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
    fn generate<H>(
        make_genome: impl Fn(&mut ThreadRng) -> G,
        // TODO: Should this be a special EC-specific trait instead of the general `Fn`?
        run_tests: impl Fn(&H) -> R,
        rng: &mut ThreadRng,
    ) -> Self
    where
        G: Borrow<H>,
        H: ?Sized,
    {
        let genome = make_genome(rng);
        let test_results = run_tests(genome.borrow());
        Self {
            genome,
            test_results,
        }
    }
}

impl<G: Eq, R: Ord> Ord for EcIndividual<G, R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.test_results.cmp(&other.test_results)
    }
}

impl<G: PartialEq, R: PartialOrd> PartialOrd for EcIndividual<G, R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.test_results.partial_cmp(&other.test_results)
    }
}

// TODO: Maybe change R to implement `Display` and have `TestResults` have a
//   nice-ish display function.
impl<G: Display, R: Debug> Display for EcIndividual<G, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]\n{:?}", self.genome(), self.test_results())
    }
}
