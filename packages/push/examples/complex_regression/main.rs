pub mod args;

use std::ops::Not;

use anyhow::{ensure, Result};
use clap::Parser;
use ec_core::{
    generation::Generation,
    generator::{collection::CollectionGenerator, Generator},
    individual::ec::{self},
    operator::{
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        selector::{best::Best, lexicase::Lexicase, Select, Selector},
        Composable,
    },
    test_results::{self, TestResults},
};
use ec_linear::mutator::umad::Umad;
use ordered_float::OrderedFloat;
use push::{
    genome::plushy::{GeneGenerator, Plushy},
    instruction::{variable_name::VariableName, FloatInstruction},
    push_vm::{program::PushProgram, push_state::PushState, HasStack, State},
    vec_into,
};
use rand::thread_rng;

use crate::args::{Args, RunModel};

/*
 * This is an implementation of the "complex regression" problem from the
 * Propeller implementation of PushGP:
 * https://github.com/lspector/propeller/blob/71d378f49fdf88c14dda88387291c9c7be0f1277/src/propeller/problems/complex_regression.cljc
 */

fn main() -> Result<()> {
    // The penalty value to use when an evolved program doesn't have an expected
    // "return" value on the appropriate stack at the end of its execution.
    const PENALTY_VALUE: f64 = 1_000.0;

    let args = Args::parse();

    // Inputs from -4 (inclusive) to 4 (exclusive) in increments of 0.25.
    let training_cases = (-4 * 4..4 * 4)
        .map(|n| OrderedFloat(f64::from(n) / 4.0))
        .collect::<Vec<_>>();

    /*
     * The `scorer` will need to take an evolved program (sequence of
     * instructions) and run it on all the inputs from -4 (inclusive) to 4
     * (exclusive) in increments of 0.25, collecting together the errors,
     * i.e., the absolute difference between the returned value and the
     * expected value.
     *
     * The target polynomial is (x^3 + 1)^3 + 1
     */
    let scorer = |genome: &Plushy| -> TestResults<test_results::Error<OrderedFloat<f64>>> {
        let program = Vec::<PushProgram>::from(genome.clone());
        let errors: TestResults<test_results::Error<OrderedFloat<f64>>> = training_cases
            .iter()
            .map(|input| {
                #[allow(clippy::unwrap_used)]
                let state = PushState::builder()
                    .with_max_stack_size(1000)
                    .with_program(program.clone())
                    // This will return an error if the program is longer than the allowed
                    //  max stack size.
                    // We arguably should check that and return an error here.
                    .unwrap()
                    .with_float_input("x", *input)
                    .build();
                let sub_expr = *input * *input * *input + 1.0;
                let expected = sub_expr * sub_expr * sub_expr + 1.0;
                #[allow(clippy::option_if_let_else)]
                match state.run_to_completion() {
                    Ok(final_state) => OrderedFloat(
                        final_state
                            .stack::<OrderedFloat<f64>>()
                            .top()
                            .map_or(PENALTY_VALUE, |answer| (answer - expected).abs()),
                    ),
                    Err(_) => {
                        // Do some logging, perhaps?
                        OrderedFloat(PENALTY_VALUE)
                    }
                }
            })
            .collect();
        errors
    };

    let selector = Lexicase::new(training_cases.len());

    let mut rng = thread_rng();

    let instruction_set = vec_into![
        FloatInstruction::Add,
        FloatInstruction::Subtract,
        FloatInstruction::Multiply,
        FloatInstruction::ProtectedDivide,
        FloatInstruction::Dup,
        FloatInstruction::Push(OrderedFloat(0.0)),
        FloatInstruction::Push(OrderedFloat(1.0)),
        VariableName::from("x")
    ];

    let gene_generator = GeneGenerator::with_uniform_close_probability(instruction_set);

    let plushy_generator = CollectionGenerator {
        size: args.max_initial_instructions,
        element_generator: gene_generator.clone(),
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

    let umad = Umad::new(0.1, 0.1, gene_generator);

    let make_new_individual = Select::new(selector)
        .then(GenomeExtractor)
        .then(Mutate::new(umad))
        .wrap::<GenomeScorer<_, _>>(scorer);

    let mut generation = Generation::new(make_new_individual, population);

    // TODO: It might be useful to insert some kind of logging system so we can
    //   make this less imperative in nature.

    for generation_number in 0..args.num_generations {
        match args.run_model {
            RunModel::Serial => generation.serial_next()?,
            RunModel::Parallel => generation.par_next()?,
        }

        let best = Best.select(generation.population(), &mut rng)?;
        // TODO: Change 2 to be the smallest number of digits needed for
        //  args.num_generations-1.
        println!("Generation {generation_number:2} best is {best:#?}");

        if best.test_results.total_result.error == OrderedFloat(0.0) {
            println!("SUCCESS");
            break;
        }
    }

    Ok(())
}
