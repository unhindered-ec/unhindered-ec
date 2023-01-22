use super::ChildMaker;
use crate::{
    bitstring::{Bitstring, LinearMutation},
    individual::{ec::EcIndividual, Individual},
    selector::Selector,
    test_results::TestResults, recombinator::{Recombinator, two_point_xo::TwoPointXo},
};
use rand::rngs::ThreadRng;
use std::iter::Sum;

#[derive(Clone)]
pub struct TwoPointXoMutate<'a> {
    scorer: &'a (dyn Fn(&[bool]) -> Vec<i64> + Sync),
}

impl<'a> TwoPointXoMutate<'a> {
    pub fn new(scorer: &'a (dyn Fn(&[bool]) -> Vec<i64> + Sync)) -> Self {
        Self { scorer }
    }
}

impl<'a, S, R> ChildMaker<Vec<EcIndividual<Bitstring, TestResults<R>>>, S> for TwoPointXoMutate<'a>
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
        let first_parent = selector.select(rng, population);
        let second_parent = selector.select(rng, population);
        let parent_genomes = [first_parent.genome(), second_parent.genome()];

        let xo_genome = TwoPointXo.recombine(parent_genomes, rng);
        let mutated_genome = xo_genome.mutate_one_over_length(rng);
        let test_results = (self.scorer)(&mutated_genome)
            .into_iter()
            .map(From::from)
            .sum();
        EcIndividual::new(mutated_genome, test_results)
    }
}
