/// This module contains the implementation of the `Ec` struct and related
/// functionality. The `Ec` struct represents an individual in an evolutionary
/// computation system. It provides methods for comparing individuals and
/// displaying their contents.
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use rand::prelude::Distribution;

use super::{scorer::Scorer, Individual};

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
pub trait WithScorer<Scorer> {
    /// Add a scorer to the genome generator, creating an `IndividualGenerator`.
    fn with_scorer(self, scorer: Scorer) -> IndividualGenerator<Self, Scorer>
    where
        Self: Sized;
}

impl<GG, S> WithScorer<S> for GG {
    /// Add a scorer to the genome generator, creating an `IndividualGenerator`.
    fn with_scorer(self, scorer: S) -> IndividualGenerator<GG, S> {
        IndividualGenerator::new(self, scorer)
    }
}

// G is Genome
impl<GenomeT, GenomeGeneratorT, ScorerT, ResultT> Distribution<EcIndividual<GenomeT, ResultT>>
    for IndividualGenerator<GenomeGeneratorT, ScorerT>
where
    GenomeGeneratorT: Distribution<GenomeT>,
    ScorerT: Scorer<GenomeT, ResultT>,
{
    /// Generate a new, random, individual.
    ///
    /// This creates a new genome of type `G` using the genome generator of
    /// type `GG`, and then scores the genome using the scorer of type `S`.
    /// The genome and the test results (of type `R`) are then
    /// used to create a new `EcIndividual`.
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> EcIndividual<GenomeT, ResultT> {
        let genome = self.genome_generator.sample(rng);
        let test_results = self.scorer.score(&genome);

        EcIndividual::new(genome, test_results)
    }
}
