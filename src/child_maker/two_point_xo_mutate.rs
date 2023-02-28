use super::ChildMaker;
use crate::{
    bitstring::Bitstring,
    individual::{ec::EcIndividual, Individual},
    operator::{recombinator::{
        two_point_xo::TwoPointXo, Recombine,
    }, mutator::{with_one_over_length::WithOneOverLength, Mutate}},
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

// TODO: Try this as a closure and see if we still get lifetime
//   capture problems.
fn make_child_genome(parent_genomes: [Bitstring; 2], rng: &mut ThreadRng) -> Bitstring {
    Recombine::new(TwoPointXo)
        .then(Mutate::new(WithOneOverLength))
        .apply(parent_genomes, rng)
}

impl<'scorer, S, R> ChildMaker<Vec<EcIndividual<Bitstring, TestResults<R>>>, S>
    for TwoPointXoMutate<'scorer>
where
    S: for<'pop> Operator<
        &'pop Vec<EcIndividual<Bitstring, TestResults<R>>>,
        Output = &'pop EcIndividual<Bitstring, TestResults<R>>,
    >,
    R: Sum + Copy + From<i64>,
{
    fn make_child(
        &self,
        rng: &mut ThreadRng,
        population: &Vec<EcIndividual<Bitstring, TestResults<R>>>,
        selector: &S,
    ) -> EcIndividual<Bitstring, TestResults<R>> {
        let first_parent = selector.apply(population, rng);
        let second_parent = selector.apply(population, rng);

        let parent_genomes = [
            first_parent.genome().clone(),
            second_parent.genome().clone(),
        ];

        let mutated_genome = make_child_genome(parent_genomes, rng);

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

    use crate::bitstring::count_ones;

    use super::*;

    #[test]
    fn smoke_test() {
        let mut rng = thread_rng();

        let first_parent = EcIndividual::new_bitstring(100, count_ones, &mut rng);
        let second_parent = EcIndividual::new_bitstring(100, count_ones, &mut rng);

        let first_genome = first_parent.genome().clone();
        let second_genome = second_parent.genome().clone();

        let child_genome = make_child_genome([first_genome, second_genome], &mut rng);

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
