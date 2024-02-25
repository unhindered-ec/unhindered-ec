/// This module contains the implementation of the `Ec` struct and related
/// functionality. The `Ec` struct represents an individual in an evolutionary
/// computation system. It provides methods for comparing individuals and
/// displaying their contents.
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use rand::rngs::ThreadRng;

use super::{
    scorer::{FnScorer, Scorer},
    Individual,
};
use crate::generator::Generator;

/// `EcIndividual` is a struct that represents an individual in an evolutionary
/// computation system. It contains a genome and the results of scoring the
/// genome.
#[derive(Debug, Eq, PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct EcIndividual<G, R> {
    pub genome: G,
    pub test_results: R,
}

impl<G, R> Individual for EcIndividual<G, R> {
    type Genome = G;
    type TestResults = R;

    /// Get the genome of the individual.
    fn genome(&self) -> &Self::Genome {
        &self.genome
    }

    /// Get the test results of the individual.
    fn test_results(&self) -> &Self::TestResults {
        &self.test_results
    }
}

impl<G, R> EcIndividual<G, R> {
    /// Create a new `EcIndividual` with the given genome and test results.
    pub const fn new(genome: G, test_results: R) -> Self {
        Self {
            genome,
            test_results,
        }
    }
}

impl<G: Eq, R: Ord> Ord for EcIndividual<G, R> {
    /// Compare two individuals based on their test results.
    fn cmp(&self, other: &Self) -> Ordering {
        self.test_results.cmp(&other.test_results)
    }
}

impl<G: PartialEq, R: PartialOrd> PartialOrd for EcIndividual<G, R> {
    /// Compare two individuals based on their test results.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.test_results.partial_cmp(&other.test_results)
    }
}

// TODO: Maybe change R to implement `Display` and have `TestResults` have a
//   nice-ish display function.
impl<G: Display, R: Debug> Display for EcIndividual<G, R> {
    /// Display the genome and test results of the individual.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]\n{:?}", self.genome(), self.test_results())
    }
}

/// A generator for creating individuals, which requires a genome generator and
/// a scorer.
///
/// The genome generator is used to create the genome of the individual, and the
/// scorer is used to score the genome.
pub struct IndividualGenerator<GG, S> {
    pub genome_generator: GG,
    pub scorer: S,
}

impl<GG, S> IndividualGenerator<GG, S> {
    /// Create a new `IndividualGenerator` with the given genome generator and
    /// scorer.
    pub const fn new(genome_generator: GG, scorer: S) -> Self {
        Self {
            genome_generator,
            scorer,
        }
    }
}

/// A trait for adding a scorer to a genome generator, creating
/// an `IndividualGenerator`.
pub trait WithScorer {
    /// Add a scorer to the genome generator, creating an `IndividualGenerator`.
    fn with_scorer<S, G>(self, scorer: S) -> IndividualGenerator<Self, S>
    where
        Self: Sized,
        S: Scorer<G>;

    fn with_scorer_fn<F, G, R>(self, f: F) -> IndividualGenerator<Self, FnScorer<F>>
    where
        Self: Sized,
        F: Fn(&G) -> R,
    {
        self.with_scorer(FnScorer(f))
    }
}

impl<GG> WithScorer for GG {
    /// Add a scorer to the genome generator, creating an `IndividualGenerator`.
    fn with_scorer<S, G>(self, scorer: S) -> IndividualGenerator<GG, S>
    where
        S: Scorer<G>,
    {
        IndividualGenerator::new(self, scorer)
    }
}

// G is Genome
// GG is Genome generator
// S is Scorer
impl<G, GG, S> Generator<EcIndividual<G, S::Score>> for IndividualGenerator<GG, S>
where
    GG: Generator<G>,
    S: Scorer<G>,
{
    /// Generate a new, random, individual.
    ///
    /// This creates a new genome of type `G` using the genome generator of
    /// type `GG`, and then scores the genome using the scorer of type `S`.
    /// The genome and the test results (of type `S::Score`) are then
    /// used to create a new `EcIndividual`.
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<EcIndividual<G, S::Score>> {
        let genome = self.genome_generator.generate(rng)?;
        let test_results = self.scorer.score(&genome);
        Ok(EcIndividual::new(genome, test_results))
    }
}
