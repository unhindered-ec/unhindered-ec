#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

pub mod args;

use crate::args::{Args, RunModel};
use anyhow::{ensure, Result};
use clap::Parser;
use ec_core::{
    generation::Generation,
    generator::{collection::CollectionGenerator, Generator},
    individual::ec::{self, EcIndividual},
    operator::{
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        selector::{
            best::Best, lexicase::Lexicase, tournament::Tournament, weighted::Weighted, Select,
            Selector,
        },
        Composable,
    },
    test_results::{self, TestResults},
};
use ec_linear::mutator::umad::Umad;
use push::{
    genome::plushy::Plushy,
    instruction::{IntInstruction, PushInstruction, VariableName},
    push_vm::{
        push_state::{HasStack, PushState},
        State,
    },
};
use rand::thread_rng;
use std::ops::Not;

fn main() -> Result<()> {
    // Using `Error` in `TestResults<Error>` will have the run favor smaller
    // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
    // favor larger values.
    type Pop = Vec<EcIndividual<Plushy, TestResults<test_results::Error>>>;

    // The penalty value to use when an evolved program doesn't have an expected
    // "return" value on the appropriate stack at the end of its execution.
    const PENALTY_VALUE: i64 = 1_000;

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
    let scorer = |program: &Plushy| -> TestResults<test_results::Error> {
        let errors: TestResults<test_results::Error> = (0..10)
            .map(|input| {
                let state = PushState::builder(program.get_instructions())
                    .with_int_input("x", input)
                    .build();
                // This is the degree 3 problem in https://github.com/lspector/Clojush/blob/master/src/clojush/problems/demos/simple_regression.clj
                let expected = input * input * input - 2 * input * input - input;
                state
                    .run_to_completion()
                    .stack_mut()
                    .top()
                    // If `last()` returns `None`, then there was nothing on top of the integer stack
                    // so we want to use the `PENALTY_VALUE` for the error on this test case.
                    .map_or(PENALTY_VALUE, |answer| (answer - expected).abs())
            })
            .collect();
        errors
    };

    // The degree 3 problem in https://github.com/lspector/Clojush/blob/master/src/clojush/problems/demos/simple_regression.clj
    // just uses 10 test cases, 0 to 9 (inclusive).
    let num_test_cases = 10;

    let lexicase = Lexicase::new(num_test_cases);
    let binary_tournament = Tournament::new(2);

    let selector: Weighted<Pop> = Weighted::new(Best, 1)
        .with_selector(lexicase, 5)
        .with_selector(binary_tournament, args.population_size - 1);

    let mut rng = thread_rng();

    let mut instruction_set = vec![
        PushInstruction::IntInstruction(IntInstruction::Add),
        PushInstruction::IntInstruction(IntInstruction::Subtract),
        PushInstruction::IntInstruction(IntInstruction::Multiply),
        PushInstruction::IntInstruction(IntInstruction::ProtectedDivide),
    ];
    instruction_set.push(PushInstruction::InputVar(VariableName::from("x")));

    let plushy_generator = CollectionGenerator {
        size: args.max_initial_instructions,
        element_generator: instruction_set.clone(),
    };

    let individual_generator = ec::IndividualGenerator {
        genome_generator: plushy_generator,
        scorer,
    };

    let population_generator = CollectionGenerator {
        size: args.population_size,
        element_generator: individual_generator,
    };

    let population = population_generator.generate(&mut rng)?;

    ensure!(population.is_empty().not());

    let best = Best.select(&population, &mut rng)?;
    println!("Best initial individual is {best:?}");

    let umad = Umad::new(0.1, 0.1, instruction_set);

    let make_new_individual = Select::new(selector)
        .then(GenomeExtractor)
        .then(Mutate::new(umad))
        .wrap::<GenomeScorer<_, _>>(scorer);

    let mut generation = Generation::new(make_new_individual, population);

    // // TODO: It might be useful to insert some kind of logging system so we can
    // //   make this less imperative in nature.

    for generation_number in 0..args.num_generations {
        match args.run_model {
            RunModel::Serial => generation.serial_next()?,
            RunModel::Parallel => generation.par_next()?,
        }

        let best = Best.select(generation.population(), &mut rng)?;
        // TODO: Change 2 to be the smallest number of digits needed for
        //  args.num_generations-1.
        println!("Generation {generation_number:2} best is {best:?}");

        if best.test_results.total_result.error == 0 {
            break;
        }
    }

    Ok(())
}
