#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::iter::Sum;

use args::{RunModel, TargetProblem, Args};
use rand::rngs::ThreadRng;

use bitstring::{Bitstring, LinearCrossover, LinearMutation, count_ones, hiff};
use population::Population;
use generation::{Generation, WeightedSelector, ChildMaker}; 
use test_results::{TestResults, Score, Error};
use individual::{Individual};

pub mod args;
pub mod test_results;
pub mod individual;
pub mod population;
pub mod generation;
pub mod bitstring;

/// # Panics
/// 
/// This can panic for a whole host of reasons, mostly because the
/// population or the collection of selectors is empty.
pub fn do_main(args: Args) {
    let scorer = match args.target_problem {
        TargetProblem::CountOnes => count_ones,
        TargetProblem::Hiff => hiff
    };

    // Use lexicase selection almost exclusively, but typically carry forward
    // at least one copy of the best individual (as measured by total fitness).
    let weighted_selectors: Vec<WeightedSelector<Bitstring, TestResults<_>>> =
        vec![
                (&Population::best_individual, 1),
                (&Population::lexicase, args.population_size-1)
            ];

    let population
        = Population::new_bitstring_population(
            args.population_size, 
            args.bit_length, 
            // TODO: I should really have a function somewhere that converts functions
            //   that return vectors of scores to `TestResults` structs.
            |bitstring| {
                scorer(bitstring).into_iter()
                    .map(From::from)
                    .sum()
            });
    assert!(!population.is_empty());

    // Using `Error` in `TestResults<Error>` will have the run favor smaller
    // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
    // favor larger values.
    // TODO: We probably want `scorer` to be generating the `TestResults` values
    //   and have it be "in charge" of whether we're using `Score` or `Error`. Then
    //   the child maker shouldn't need to care and we can just use `TestResults<R>` here.
    let child_maker: &dyn ChildMaker<Bitstring, TestResults<Error>>
        = &TwoPointXoMutateChildMaker::new(&scorer);

    let mut generation = Generation::new(
        population,
        &weighted_selectors,
        child_maker
    );

    assert!(!generation.population.is_empty());
    // let best = generation.best_individual();
    // println!("{}", best);
    // println!("Pop size = {}", generation.population.size());
    // println!("Bit length = {}", best.genome.len());

    (0..args.num_generations).for_each(|generation_number| {
        generation = match args.run_model {
            RunModel::Serial => generation.next(),
            RunModel::Parallel => generation.par_next()
        };
        let best = generation.best_individual();
        // TODO: Change 2 to be the smallest number of digits needed for
        //  args.num_generations-1.
        println!("Generation {:2} best is {}", generation_number, best);
    });
}

struct TwoPointXoMutateChildMaker<'a> {
    scorer: &'a (dyn Fn(&[bool]) -> Vec<i64> + Sync)
}

impl<'a> TwoPointXoMutateChildMaker<'a> {
    fn new(scorer: &(dyn Fn(&[bool]) -> Vec<i64> + Sync)) -> TwoPointXoMutateChildMaker {
        TwoPointXoMutateChildMaker { scorer }
    }
}

impl<'a, R: Ord + Sum + Copy + From<i64>> ChildMaker<Bitstring, TestResults<R>> for TwoPointXoMutateChildMaker<'a> {
    //     fn make_child(&self, rng: &mut ThreadRng, generation: &Generation<G, R>) -> Individual<G, R>;
    fn make_child(&self, 
                rng: &mut ThreadRng, 
                generation: &Generation<Bitstring, TestResults<R>>) -> Individual<Bitstring, TestResults<R>> {
        let first_parent = generation.get_parent(rng);
        let second_parent = generation.get_parent(rng);

        let genome
            = first_parent.genome
                .two_point_xo(&second_parent.genome, rng)
                .mutate_one_over_length(rng);
        let test_results = (self.scorer)(&genome)
            .into_iter()
            .map(From::from)
            .sum();
        Individual { 
            genome,
            test_results
        }
    }
}