use super::ChildMaker;
use crate::{
    bitstring::Bitstring,
    individual::{ec::EcIndividual, Individual},
    recombinator::{
        mutate_with_one_over_length::MutateWithOneOverLength, two_point_xo::TwoPointXo,
        Recombinator,
    },
    selector::Selector,
    test_results::TestResults,
};
use rand::rngs::ThreadRng;
use std::iter::Sum;

#[derive(Clone)]
pub struct TwoPointXoMutate<'a> {
    pub scorer: &'a (dyn Fn(&[bool]) -> Vec<i64> + Sync),
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
        let mutator = MutateWithOneOverLength;

        let first_parent = selector.select(rng, population);
        let second_parent = selector.select(rng, population);
        let parent_genomes = [first_parent.genome(), second_parent.genome()];

        let xo_genome = TwoPointXo.recombine(&parent_genomes, rng);
        let mutated_genome = mutator.recombine(&[&xo_genome], rng);
        let test_results = (self.scorer)(&mutated_genome)
            .into_iter()
            .map(From::from)
            .sum();
        EcIndividual::new(mutated_genome, test_results)
    }
}
