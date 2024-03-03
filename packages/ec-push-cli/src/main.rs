pub mod args;

use std::ops::Not;

use anyhow::{ensure, Result};
use clap::Parser;
use ec_core::{
    distributions::collection::ConvertToCollectionGenerator,
    generation::Generation,
    individual::ec::{EcIndividual, WithScorer},
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
    arr_into,
    genome::plushy::{GeneGenerator, Plushy},
    instruction::{variable_name::VariableName, IntInstruction},
    push_vm::{program::PushProgram, push_state::PushState, HasStack, State},
};
use rand::{prelude::Distribution, thread_rng};

use crate::args::{Args, RunModel};

fn main() -> Result<()> {
    // Using `Error` in `TestResults<Error>` will have the run favor smaller
    // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
    // favor larger values.
    type Pop = Vec<EcIndividual<Plushy, TestResults<test_results::Error<i64>>>>;

    // The penalty value to use when an evolved program doesn't have an expected
    // "return" value on the appropriate stack at the end of its execution.
    const PENALTY_VALUE: i64 = 1_000;

    let args = Args::parse();

    // let base_scorer = match args.target_problem {
    //     TargetProblem::DegreeThree => todo!(),
    // };

    /*
     * The `scorer` will need to take an evolved program (sequence of
     * instructions) and run it 10 times on each of the 10 test inputs (0
     * through 9), collecting together the 10 errors, i.e., the absolute
     * difference between the returned value and the expected value.
     *
     * The target polynomial is x^3 - 2x^2 - x
     */
    let scorer = |genome: &Plushy| -> TestResults<test_results::Error<i64>> {
        let program = Vec::<PushProgram>::from(genome.clone());
        let errors: TestResults<test_results::Error<i64>> = (0..10)
            .map(|input| {
                #[allow(clippy::unwrap_used)]
                let state: PushState = PushState::builder()
                    .with_max_stack_size(1000)
                    .with_program(program.clone())
                    // This will return an error if the program is longer than
                    //  the allowed max stack size.
                    // We arguably should check that and return an error here.
                    .unwrap()
                    .with_int_input("x", input)
                    .build();
                // This is the degree 3 problem in
                // https://github.com/lspector/Clojush/blob/e2c9d8c830715f7d1e644f6205c192b9e5ceead2/src/clojush/problems/demos/simple_regression.clj
                let expected = input * input * input - 2 * input * input - input;
                #[allow(clippy::option_if_let_else)]
                match state.run_to_completion() {
                    Ok(final_state) => final_state
                        .stack::<i64>()
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

    // The degree 3 problem in
    // https://github.com/lspector/Clojush/blob/
    // e2c9d8c830715f7d1e644f6205c192b9e5ceead2/src/
    // clojush/problems/demos/simple_regression.clj
    // just uses 10 test cases, 0 to 9 (inclusive).
    let num_test_cases = 10;

    let lexicase = Lexicase::new(num_test_cases);
    let binary_tournament = Tournament::new(2);

    let selector: Weighted<Pop> = Weighted::new(Best, 1)
        .with_selector(lexicase, 5)
        .with_selector(binary_tournament, args.population_size - 1);

    let mut rng = thread_rng();

    let instruction_set = arr_into![
        IntInstruction::Add,
        IntInstruction::Subtract,
        IntInstruction::Multiply,
        IntInstruction::ProtectedDivide,
        VariableName::from("x")
    ];

    let gene_generator = GeneGenerator::with_uniform_close_probability(&instruction_set)?;

    let population = gene_generator
        .to_collection_generator(args.max_initial_instructions)
        .with_scorer(scorer)
        .into_collection_generator(args.population_size)
        .sample(&mut rng);

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

        if best.test_results.total_result.error == 0 {
            break;
        }
    }

    Ok(())
}
