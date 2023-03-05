use super::ChildMaker;
use crate::{
    bitstring::Bitstring,
    individual::ec::EcIndividual,
    operator::{
        composable::map::Map,
        genome_extractor::GenomeExtractor,
        mutator::{with_one_over_length::WithOneOverLength, Mutate},
        recombinator::{two_point_xo::TwoPointXo, Recombine},
        selector::{Select, Selector},
    },
    operator::{Composable, Operator},
    test_results::TestResults,
};
use rand::rngs::ThreadRng;
use std::iter::Sum;

#[derive(Clone)]
pub struct TwoPointXoMutate<'scorer> {
    pub scorer: &'scorer (dyn Fn(&[bool]) -> Vec<i64> + Sync),
}

impl<'scorer> TwoPointXoMutate<'scorer> {
    pub fn new(scorer: &'scorer (dyn Fn(&[bool]) -> Vec<i64> + Sync)) -> Self {
        Self { scorer }
    }
}

impl<'scorer, S, R> ChildMaker<Vec<EcIndividual<Bitstring, TestResults<R>>>, S>
    for TwoPointXoMutate<'scorer>
where
    S: Selector<Vec<EcIndividual<Bitstring, TestResults<R>>>>,
    R: Sum + Copy + From<i64>,
{
    fn make_child(
        &self,
        rng: &mut ThreadRng,
        population: &Vec<EcIndividual<Bitstring, TestResults<R>>>,
        selector: &S,
    ) -> EcIndividual<Bitstring, TestResults<R>> {
        let selector = Select::new(selector);
        let mutated_genome = selector
            .clone()
            .and(selector)
            // Can I get rid of the curly braces {}?
            .then(Map::new(GenomeExtractor {}))
            .then(Recombine::new(TwoPointXo))
            .then(Mutate::new(WithOneOverLength))
            .apply(population, rng);

        let test_results = (self.scorer)(&mutated_genome)
            .into_iter()
            .map(From::from)
            .sum();
        EcIndividual::new(mutated_genome, test_results)
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::{bitstring::count_ones, individual::Individual};

    use super::*;

    #[test]
    fn smoke_test() {
        let mut rng = thread_rng();

        let first_parent = EcIndividual::new_bitstring(100, count_ones, &mut rng);
        let second_parent = EcIndividual::new_bitstring(100, count_ones, &mut rng);

        let child_genome = 
            Map::new(GenomeExtractor {})
                .then(Recombine::new(TwoPointXo))
                .then(Mutate::new(WithOneOverLength))
                .apply((&first_parent, &second_parent), &mut rng);

        let first_genome = first_parent.genome();
        let second_genome = second_parent.genome();

        let num_in_either_parent = child_genome
            .into_iter()
            .enumerate()
            .filter(|(pos, val)| *val == first_genome[*pos] || *val == second_genome[*pos])
            .count();
        assert!(
            num_in_either_parent > 95 && num_in_either_parent <= 100,
            "{num_in_either_parent} wasn't in the expected range"
        );
    }
}
