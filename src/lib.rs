#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::ops::Not;

use args::{Args, RunModel, TargetProblem};

use bitstring::{count_ones, hiff, Bitstring};
use generation::Generation;
use individual::ec::EcIndividual;
use operator::selector::lexicase::Lexicase;
#[allow(unused_imports)]
use test_results::{Error, Score, TestResults};

use crate::bitstring::new_bitstring_population;
use crate::child_maker::two_point_xo_mutate::TwoPointXoMutate;
use crate::operator::Operator;
use crate::operator::selector::best::Best;
use crate::operator::selector::tournament::Tournament;
use crate::operator::selector::weighted::Weighted;

pub mod args;
pub mod bitstring;
pub mod child_maker;
pub mod generation;
pub mod individual;
pub mod operator;
pub mod population;
pub mod test_results;

/// # Panics
///
/// This can panic for a whole host of reasons, mostly because the
/// population or the collection of selectors is empty.
pub fn do_main(args: Args) {
    let scorer = match args.target_problem {
        TargetProblem::CountOnes => count_ones,
        TargetProblem::Hiff => hiff,
    };

    let num_test_cases = match args.target_problem {
        TargetProblem::CountOnes => args.bit_length,
        TargetProblem::Hiff => 2 * args.bit_length - 1,
    };

    let lexicase = Lexicase::new(num_test_cases);
    let binary_tournament = Tournament::new(2);

    let selector = Weighted::new(&Best, 1)
        .with_selector(&lexicase, 5)
        .with_selector(&binary_tournament, args.population_size - 1);

    // Using `Error` in `TestResults<Error>` will have the run favor smaller
    // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
    // favor larger values.
    let population: Vec<EcIndividual<Bitstring, TestResults<Error>>> = new_bitstring_population(
        args.population_size,
        args.bit_length,
        // TODO: I should really have a function somewhere that converts functions
        //   that return vectors of scores to `TestResults` structs.
        |bitstring| scorer(bitstring).into_iter().map(From::from).sum(),
    );
    assert!(population.is_empty().not());

    let child_maker = TwoPointXoMutate::new(&scorer);

    let mut generation = Generation::new(population, selector, child_maker);

    let mut rng = rand::thread_rng();

    assert!(generation.population().is_empty().not());

    (0..args.num_generations).for_each(|generation_number| {
        generation = match args.run_model {
            RunModel::Serial => generation.next(),
            RunModel::Parallel => generation.par_next(),
        };
        let best = Best.apply(generation.population(), &mut rng);
        // TODO: Change 2 to be the smallest number of digits needed for
        //  args.num_generations-1.
        println!("Generation {generation_number:2} best is {best}");
    });
}
