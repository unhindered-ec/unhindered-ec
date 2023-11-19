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
    push_vm::{push_state::PushState, State},
    push_vm::{HasStack, PushInteger},
};
use rand::thread_rng;
use std::ops::Not;

/*
 * This is an implementation of the "complex regression" problem from the Propeller implementation
 * of PushGP: https://github.com/lspector/propeller/blob/master/src/propeller/problems/complex_regression.cljc
 */

fn main() -> Result<()> {
    // Using `Error` in `TestResults<Error>` will have the run favor smaller
    // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
    // favor larger values.
    type Pop = Vec<EcIndividual<Plushy, TestResults<test_results::Error>>>;

    // The penalty value to use when an evolved program doesn't have an expected
    // "return" value on the appropriate stack at the end of its execution.
    const PENALTY_VALUE: i64 = 1_000;

    let args = Args::parse();

    let training_cases = (-4 * 4..4 * 4)
        .map(|n| f64::from(n) / 4.0)
        .collect::<Vec<_>>();

    /*
     * The `scorer` will need to take an evolved program (sequence of instructions) and run it
     * on all the inputs from -4 (inclusive) to 4 (exclusive) in increments of 0.25, collecting
     * together the 10 errors, i.e., the absolute difference between the returned value and the
     * expected value.
     *
     * The target polynomial is (x^3 + 1)^3 + 1
     */
    let scorer = |program: &Plushy| -> TestResults<test_results::Error> {
        let errors: TestResults<test_results::Error> = training_cases
            .iter()
            .map(|input| {
                #[allow(clippy::unwrap_used)]
                let state = PushState::builder()
                    .with_max_stack_size(1000)
                    .with_program(program.get_instructions())
                    // This will return an error if the program is longer than the allowed max stack size.
                    // We arguably should check that and return an error here.
                    .unwrap()
                    .with_float_input("x", *input)
                    .build();
                let sub_expr = input * input * input + 1.0;
                let expected = sub_expr * sub_expr * sub_expr + 1.0;
                #[allow(clippy::option_if_let_else)]
                match state.run_to_completion() {
                    Ok(final_state) => final_state
                        .stack::<f64>()
                        .top()
                        .map_or(PENALTY_VALUE, |answer| (answer - expected).abs()),
                    Err(_) => {
                        // Do some logging, perhaps?
                        PENALTY_VALUE
                    }
                }
            })
            .collect();
        errors
    };

    let selector = Lexicase::new(training_cases.len());

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
        println!("Generation {generation_number:2} best is {best:#?}");

        if best.test_results.total_result.error == 0 {
            break;
        }
    }

    Ok(())
}
