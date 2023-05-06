use anyhow::Result;

use crate::{individual::ec::EcIndividual, population::Population, test_results::TestResults};

use super::{Composable, Operator, composable::Wrappable};

pub struct GenomeScorer<GM, S> {
    genome_maker: GM,
    scorer: S,
}

impl<G, S> GenomeScorer<G, S> {
    pub const fn new(genome_maker: G, scorer: S) -> Self {
        Self {
            genome_maker,
            scorer,
        }
    }
}

impl<G, S> Wrappable<G> for GenomeScorer<G, S> {
    type Context = S;

    fn construct(genome_maker: G, scorer: Self::Context) -> Self {
        Self::new(genome_maker, scorer)
    }
}

// scorer: &Genome -> TestResults<R>
impl<'pop, GM, S, R, P> Operator<&'pop P> for GenomeScorer<GM, S>
where
    P: Population,
    GM: Operator<&'pop P>,
    S: Fn(&GM::Output) -> TestResults<R>,
{
    type Output = EcIndividual<GM::Output, TestResults<R>>;

    fn apply(&self, population: &'pop P, rng: &mut rand::rngs::ThreadRng) -> Result<Self::Output> {
        let genome = self.genome_maker.apply(population, rng)?;
        let score = (self.scorer)(&genome);
        // TODO: We probably don't want to bake in `EcIndividual` here, but instead
        //   have things be more general than that.
        Ok(EcIndividual::new(genome, score))
    }
}
impl<GM, S> Composable for GenomeScorer<GM, S> {}
