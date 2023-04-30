use anyhow::Result;
use ec_core::{
    child_maker::ChildMaker,
    individual::ec::EcIndividual,
    operator::{
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        recombinator::Recombine,
        selector::{Select, Selector},
        Composable, Operator,
    },
    test_results::TestResults,
};
use rand::rngs::ThreadRng;
use std::{iter::Sum, ops::Not};

use crate::{
    mutator::with_one_over_length::WithOneOverLength,
    recombinator::{crossover::Crossover, two_point_xo::TwoPointXo},
};

#[derive(Clone)]
pub struct TwoPointXoMutate<Sc> {
    pub scorer: Sc,
}

impl<Sc> TwoPointXoMutate<Sc> {
    pub const fn new(scorer: Sc) -> Self {
        Self { scorer }
    }
}

impl<G, S, R, Sc> ChildMaker<Vec<EcIndividual<G, TestResults<R>>>, S> for TwoPointXoMutate<Sc>
where
    G: Crossover + FromIterator<G::Gene> + IntoIterator<Item = G::Gene> + Clone,
    G::Gene: Not<Output = G::Gene>,
    S: Selector<Vec<EcIndividual<G, TestResults<R>>>>,
    R: Sum + Copy + From<i64>,
    Sc: Fn(&G) -> Vec<i64>,
{
    fn make_child(
        &self,
        rng: &mut ThreadRng,
        population: &Vec<EcIndividual<G, TestResults<R>>>,
        selector: &S,
    ) -> Result<EcIndividual<G, TestResults<R>>> {
        let selector = Select::new(selector);
        // Population -> child genome
        let make_mutated_genome = selector
            .apply_twice()
            .then_map(GenomeExtractor)
            .then(Recombine::new(TwoPointXo))
            .then(Mutate::new(WithOneOverLength));

        let make_test_results = |genome: &G| -> TestResults<R> {
            (self.scorer)(genome).into_iter().map(From::from).sum()
        };

        let genome_scorer = GenomeScorer::new(make_mutated_genome, make_test_results);
        // Operator::<_>::apply(&genome_scorer, population, rng)
        genome_scorer.apply(population, rng)
    }
}

#[cfg(test)]
mod tests {
    use ec_core::{
        generator::Generator,
        individual::{ec, Individual},
        operator::identity::Identity,
    };
    use rand::thread_rng;

    use crate::genome::{
        bitstring::{self, Bitstring},
        bitstring_vec::count_ones,
    };

    use super::*;

    #[test]
    fn smoke_test() {
        let mut rng = thread_rng();
        let bit_length = 100;

        let bitstring_context = bitstring::GeneratorContext {
            num_bits: bit_length,
            probability: 0.5,
        };
        let individual_context = ec::GeneratorContext {
            genome_context: bitstring_context,
            scorer: |bitstring: &Bitstring| count_ones(&bitstring.bits),
        };
        let first_parent = rng.generate(&individual_context);
        let second_parent = rng.generate(&individual_context);

        #[allow(clippy::unwrap_used)]
        let child_genome = Identity::new((&first_parent, &second_parent))
            .then_map(GenomeExtractor)
            .then(Recombine::new(TwoPointXo))
            .then(Mutate::new(WithOneOverLength))
            .apply((), &mut rng)
            .unwrap();

        let first_genome = first_parent.genome();
        assert_eq!(bit_length, first_genome.bits.len());
        let second_genome = second_parent.genome();
        assert_eq!(bit_length, second_genome.bits.len());

        let num_in_either_parent = child_genome
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(pos, val)| {
                *val == first_genome.bits[*pos] || *val == second_genome.bits[*pos]
            })
            .count();
        let target_range = (bit_length - 5)..=bit_length;
        assert!(
            target_range.contains(&num_in_either_parent),
            "{num_in_either_parent} wasn't in the expected range {target_range:?}, \n{first_genome:?}, \n{second_genome:?}, \n{child_genome:?}"
        );
    }
}
