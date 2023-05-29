use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use rand::rngs::ThreadRng;

use crate::generator::Generator;

use super::{scorer::Scorer, Individual};

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

pub struct GeneratorContext<GC, S> {
    pub genome_context: GC,
    pub scorer: S,
}

// G is Genome
// GC is GenomeContext
// S is Scorer
// R is the TestResult type
impl<G, GC, S, R> Generator<EcIndividual<G, R>, GeneratorContext<GC, S>> for ThreadRng
where
    Self: Generator<G, GC>,
    S: Scorer<G, R>,
{
    fn generate(&mut self, context: &GeneratorContext<GC, S>) -> EcIndividual<G, R> {
        let genome = self.generate(&context.genome_context);
        let test_results = context.scorer.score(&genome);
        EcIndividual::new(genome, test_results)
    }
}
