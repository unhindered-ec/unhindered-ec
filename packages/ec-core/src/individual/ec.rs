/// This module contains the implementation of the `Ec` struct and related
/// functionality. The `Ec` struct represents an individual in an evolutionary
/// computation system. It provides methods for comparing individuals and
/// displaying their contents.
use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use rand::prelude::Distribution;

use super::{
    Individual,
    scorer::{FnScorer, Scorer},
};

/// `EcIndividual` is a struct that represents an individual in an evolutionary
/// computation system. It contains a genome and the results of scoring the
/// genome.
#[derive(Debug, Eq, PartialEq, Clone)]
#[expect(
    clippy::module_name_repetitions,
    reason = "This is legacy and arguably should be changed. Tracked in #221"
)]
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

impl<G, R> From<(G, R)> for EcIndividual<G, R> {
    fn from((genome, test_results): (G, R)) -> Self {
        Self::new(genome, test_results)
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

impl<G: Display, R: Display> Display for EcIndividual<G, R> {
    /// Display the genome and test results of the individual.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]\n{}", self.genome(), self.test_results())
    }
}

/// A [`Distribution`] for creating individuals, which requires a genome
/// [`Distribution`] and a scorer.
///
/// The genome [`Distribution`] is used to create the genome of the individual,
/// and then the scorer is used to score the genome.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndividualDistribution<GD, S> {
    pub genome_distribution: GD,
    pub scorer: S,
}

impl<GD, S> IndividualDistribution<GD, S> {
    /// Create a new `IndividualGenerator` with the given genome generator and
    /// scorer.
    pub const fn new(genome_distribution: GD, scorer: S) -> Self {
        Self {
            genome_distribution,
            scorer,
        }
    }
}

/// A trait for adding a scorer to a genome [`Distribution`], creating
/// an [`IndividualDistribution`].
///
/// This trait is blanket-implemented for any `T` as such it's not meant to be
/// implemented externally.
pub trait WithScorer {
    /// Add a scorer to the genome generator, creating an `IndividualGenerator`.
    fn with_scorer<S, G>(self, scorer: S) -> IndividualDistribution<Self, S>
    where
        Self: Sized,
        S: Scorer<G>;

    fn with_scorer_fn<F, G, R>(self, f: F) -> IndividualDistribution<Self, FnScorer<F>>
    where
        Self: Sized,
        F: Fn(&G) -> R,
    {
        self.with_scorer(FnScorer(f))
    }
}

impl<GD> WithScorer for GD {
    fn with_scorer<S, G>(self, scorer: S) -> IndividualDistribution<GD, S>
    where
        S: Scorer<G>,
    {
        IndividualDistribution::new(self, scorer)
    }
}

impl<GenomeT, GenomeDistributionT, ScorerT> Distribution<EcIndividual<GenomeT, ScorerT::Score>>
    for IndividualDistribution<GenomeDistributionT, ScorerT>
where
    GenomeDistributionT: Distribution<GenomeT>,
    ScorerT: Scorer<GenomeT>,
{
    /// Generate a new, random, individual.
    ///
    /// This creates a new genome of type `G` using the genome generator of
    /// type `GD`, and then scores the genome using the scorer of type `S`.
    /// The genome and the test results (of type `S::Score`) are then
    /// used to create a new `EcIndividual`.
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> EcIndividual<GenomeT, ScorerT::Score> {
        let genome = self.genome_distribution.sample(rng);
        let test_results = self.scorer.score(&genome);

        EcIndividual::new(genome, test_results)
    }
}
