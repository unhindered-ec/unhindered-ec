use super::ChildMaker;
use crate::{
    bitstring::Bitstring,
    individual::ec::EcIndividual,
    operator::{
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
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
pub struct TwoPointXoMutate<Sc> {
    pub scorer: Sc,
}

impl<Sc> TwoPointXoMutate<Sc> {
    pub const fn new(scorer: Sc) -> Self {
        Self { scorer }
    }
}

impl<S, R, Sc> ChildMaker<Vec<EcIndividual<Bitstring, TestResults<R>>>, S> for TwoPointXoMutate<Sc>
where
    S: Selector<Vec<EcIndividual<Bitstring, TestResults<R>>>>,
    R: Sum + Copy + From<i64>,
    Sc: Fn(&[bool]) -> Vec<i64>,
{
    fn make_child(
        &self,
        rng: &mut ThreadRng,
        population: &Vec<EcIndividual<Bitstring, TestResults<R>>>,
        selector: &S,
    ) -> EcIndividual<Bitstring, TestResults<R>> {
        let selector = Select::new(selector);
        // Population -> child genome
        let make_mutated_genome = selector
            .apply_twice()
            .then_map(GenomeExtractor)
            .then(Recombine::new(TwoPointXo))
            .then(Mutate::new(WithOneOverLength));

        let make_test_results =
            |genome: &Vec<bool>| (self.scorer)(genome).into_iter().map(From::from).sum();

        let genome_scorer = GenomeScorer::new(make_mutated_genome, make_test_results);
        genome_scorer.apply(population, rng)
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use crate::{bitstring::count_ones, individual::Individual, operator::identity::Identity};

    use super::*;

    #[test]
    fn smoke_test() {
        let mut rng = thread_rng();

        let first_parent = EcIndividual::new_bitstring(100, count_ones, &mut rng);
        let second_parent = EcIndividual::new_bitstring(100, count_ones, &mut rng);

        let child_genome = Identity::new((&first_parent, &second_parent))
            .then_map(GenomeExtractor)
            .then(Recombine::new(TwoPointXo))
            .then(Mutate::new(WithOneOverLength))
            .apply((), &mut rng);

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
