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
    pub fn new(scorer: Sc) -> Self {
        Self { scorer }
    }
}

fn make_tr_scorer<R>(scorer: impl Fn(&Bitstring) -> Vec<R>) -> impl Fn(&Bitstring) -> TestResults<R>
where
    R: Sum + Copy,
{
    move |genome| {
        scorer(genome)
            .into_iter()
            .map(From::from)
            .sum::<TestResults<R>>()
    }
}

impl<S, R, Sc> ChildMaker<Vec<EcIndividual<Bitstring, TestResults<R>>>, S> for TwoPointXoMutate<Sc>
where
    S: Selector<Vec<EcIndividual<Bitstring, TestResults<R>>>>,
    R: Sum + Copy + From<i64>,
    Sc: Fn(&Bitstring) -> Vec<R> + Clone,
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

        let tr_scorer = make_tr_scorer(self.scorer.clone());
        // let make_test_results = |genome: &Bitstring| {
        //     (self.scorer)(genome)
        //         .into_iter()
        //         .map(From::from)
        //         .sum::<TestResults<R>>()
        //     // todo!()
        // };
        GenomeScorer::new(make_mutated_genome, tr_scorer).apply(population, rng)

        // Create the individual
        // Score the individual, creating a scored individual

        // let test_results = (self.scorer)(&mutated_genome)
        //     .into_iter()
        //     .map(From::from)
        //     .sum();
        // EcIndividual::new(mutated_genome, test_results)
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
