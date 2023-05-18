#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

pub mod args;

use crate::args::Args;
use anyhow::{ensure, Result};
use clap::Parser;
use ec_core::{
    generator::Generator,
    individual::ec::{self, EcIndividual},
    operator::selector::{
        best::Best, lexicase::Lexicase, tournament::Tournament, weighted::Weighted, Selector,
    },
    population::{self},
    test_results::{self, TestResults},
};
use push::state::{
    push_state::{self, IntInstruction, PushInstruction, PushState},
    State,
};
use rand::thread_rng;
use std::ops::Not;

fn main() -> Result<()> {
    // Using `Error` in `TestResults<Error>` will have the run favor smaller
    // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
    // favor larger values.
    type Pop = Vec<EcIndividual<Vec<PushInstruction>, TestResults<test_results::Score>>>;

    let args = Args::parse();

    // let base_scorer = match args.target_problem {
    //     TargetProblem::DegreeThree => todo!(),
    // };

    /*
     * The `scorer` will need to take an evolved program (sequence of instructions) and run it
     * 10 times on each of the 10 test inputs (0 through 9), collecting together the 10 errors,
     * i.e., the absolute difference between the returned value and the expected value.
     *
     * The target polynomial is x^3 - 2x^2 - x
     */
    let scorer = |program: &Vec<PushInstruction>| -> TestResults<test_results::Error> {
        let errors: TestResults<test_results::Error> = (0..10)
            .map(|input| {
                let mut state = PushState::new(program.clone()).with_input("x", input);
                state.run_to_completion();
                // This is the degree 3 problem in https://github.com/lspector/Clojush/blob/master/src/clojush/problems/demos/simple_regression.clj
                let expected = input * input * input - 2 * input * input - input;
                state
                    .int()
                    .last()
                    .map_or(test_results::Error { error: 1_000 }, |answer| {
                        test_results::Error {
                            error: (answer - expected).abs(),
                        }
                    })
            })
            .collect();
        errors
    };

    // The degree 3 problem in https://github.com/lspector/Clojush/blob/master/src/clojush/problems/demos/simple_regression.clj
    // just uses 10 test cases, 0 to 9 (inclusive).
    let num_test_cases = 10;

    let lexicase = Lexicase::new(num_test_cases);
    let binary_tournament = Tournament::new(2);

    let _selector: Weighted<Pop> = Weighted::new(Best, 1)
        .with_selector(lexicase, 5)
        .with_selector(binary_tournament, args.population_size - 1);

    let mut rng = thread_rng();

    let push_program_context = push_state::GeneratorContext {
        max_initial_instructions: args.max_initial_instructions,
        instruction_set: vec![
            PushInstruction::InputVar("x".to_string()),
            PushInstruction::IntInstruction(IntInstruction::Add),
            PushInstruction::IntInstruction(IntInstruction::Subtract),
            PushInstruction::IntInstruction(IntInstruction::Multiply),
            PushInstruction::IntInstruction(IntInstruction::ProtectedDivide),
        ],
    };

    let individual_context = ec::GeneratorContext {
        genome_context: push_program_context,
        scorer,
    };

    let population_context = population::GeneratorContext {
        population_size: args.population_size,
        individual_context,
    };
    let population = rng.generate(&population_context);

    ensure!(population.is_empty().not());

    // println!("{population:?}");

    let best = Best.select(&population, &mut rng)?;
    // TODO: Change 2 to be the smallest number of digits needed for
    //  args.num_generations-1.
    println!("Best initial individual is {best:?}");

    // // Let's assume the process will be generational, i.e., we replace the entire
    // // population with newly created/selected individuals every generation.
    // // `generation` will be a mutable operator (containing the data structures for
    // // the population(s) and recombinators, scorers, etc.) that acts on a population
    // // returning a new population. We'll have different generation operators for
    // // serial vs. parallel generation of new individuals.

    // let make_new_individual = Select::new(selector)
    //     .apply_twice()
    //     .then_map(GenomeExtractor)
    //     .then(Recombine::new(TwoPointXo))
    //     .then(Mutate::new(WithOneOverLength))
    //     .wrap::<GenomeScorer<_, _>>(scorer);

    // // generation::new() will take
    // //   * a pipeline that gets us from population -> new individual
    // //   * an initial population.
    // let mut generation = Generation::new(make_new_individual, population);

    // // TODO: It might be useful to insert some kind of logging system so we can
    // //   make this less imperative in nature.

    // (0..args.num_generations).try_for_each(|generation_number| {
    //     match args.run_model {
    //         RunModel::Serial => generation.serial_next()?,
    //         RunModel::Parallel => generation.par_next()?,
    //     }

    //     let best = Best.select(generation.population(), &mut rng)?;
    //     // TODO: Change 2 to be the smallest number of digits needed for
    //     //  args.num_generations-1.
    //     println!("Generation {generation_number:2} best is {best:?}");

    //     Ok::<(), anyhow::Error>(())
    // })?;

    Ok(())
}
