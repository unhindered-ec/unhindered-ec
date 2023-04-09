pub mod args;

use crate::args::{Args, RunModel, TargetProblem};
use anyhow::{ensure, Result};
use clap::Parser;
use ec_core::{
    generation::Generation,
    individual::ec::EcIndividual,
    operator::selector::{
        best::Best, lexicase::Lexicase, tournament::Tournament, weighted::Weighted, Selector,
    },
    test_results::{self, TestResults},
};
use ec_linear::{
    bitstring::{count_ones, hiff, new_bitstring_population, Bitstring},
    child_maker::two_point_xo_mutate::TwoPointXoMutate,
};
use std::ops::Not;

// fn main() -> Result<()> {
//     let args = Args::parse();

//     let scorer = match args.target_problem {
//         TargetProblem::CountOnes => count_ones,
//         TargetProblem::Hiff => hiff,
//     };

//     let num_test_cases = match args.target_problem {
//         TargetProblem::CountOnes => args.bit_length,
//         TargetProblem::Hiff => 2 * args.bit_length - 1,
//     };

//     let lexicase = Lexicase::new(num_test_cases);
//     let binary_tournament = Tournament::new(2);

//     // Using `Error` in `TestResults<Error>` will have the run favor smaller
//     // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
//     // favor larger values.
//     type Pop = Vec<EcIndividual<Bitstring, TestResults<test_results::Error>>>;

//     let selector: Weighted<Pop> = Weighted::new(Best, 1)
//         .with_selector(lexicase, 5)
//         .with_selector(binary_tournament, args.population_size - 1);

//     // Change the code below to initially just generate a set of genomes (which I think
//     // the `Generate` trait does for us, but may that should be an `Operator`?) and then
//     // score them in a separate step.

//     let population: Pop = new_bitstring_population(
//         args.population_size,
//         args.bit_length,
//         // TODO: I should really have a function somewhere that converts functions
//         //   that return vectors of scores to `TestResults` structs.
//         |bitstring| scorer(bitstring).into_iter().map(From::from).sum(),
//     );
//     ensure!(population.is_empty().not());

//     todo!()
// }

/// # Errors
///
/// This can return an error for a whole host of reasons, mostly because the
/// population or the collection of selectors is empty.
pub fn main() -> Result<()> {
    let args = Args::parse();

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

    // Using `Error` in `TestResults<Error>` will have the run favor smaller
    // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
    // favor larger values.
    type Pop = Vec<EcIndividual<Bitstring, TestResults<test_results::Error>>>;

    let selector: Weighted<Pop> = Weighted::new(Best, 1)
        .with_selector(lexicase, 5)
        .with_selector(binary_tournament, args.population_size - 1);

    // Change the code below to initially just generate a set of genomes (which I think
    // the `Generate` trait does for us, but may that should be an `Operator`?) and then
    // score them in a separate step.

    let population: Pop = new_bitstring_population(
        args.population_size,
        args.bit_length,
        // TODO: I should really have a function somewhere that converts functions
        //   that return vectors of scores to `TestResults` structs.
        |bitstring| scorer(bitstring).into_iter().map(From::from).sum(),
    );
    ensure!(population.is_empty().not());

    let child_maker = TwoPointXoMutate::new(scorer);

    let mut generation = Generation::new(population, &selector, child_maker);

    let mut rng = rand::thread_rng();

    ensure!(generation.population().is_empty().not());

    (0..args.num_generations).try_for_each(|generation_number| {
        generation = match args.run_model {
            RunModel::Serial => generation.next()?,
            RunModel::Parallel => generation.par_next()?,
        };
        let best = Best.select(generation.population(), &mut rng)?;
        // TODO: Change 2 to be the smallest number of digits needed for
        //  args.num_generations-1.
        println!("Generation {generation_number:2} best is {best:?}");

        Ok(())
    })
}
