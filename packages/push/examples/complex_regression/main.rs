#![expect(
    clippy::arithmetic_side_effects,
    reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing side \
              for example code."
)]

pub mod args;

use clap::Parser;
use ec_core::{
    distributions::collection::ConvertToCollectionDistribution,
    generation::Generation,
    individual::{ec::WithScorer, scorer::FnScorer},
    operator::{
        Composable,
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        selector::{Select, Selector, best::Best, lexicase::Lexicase},
    },
    performance::{error_value::ErrorValue, test_results::TestResults},
    uniform_distribution_of,
};
use ec_linear::mutator::umad::Umad;
use miette::ensure;
use num_traits::Float;
use ordered_float::OrderedFloat;
use push::{
    error::{into_state::IntoState, logging::PrintError},
    evaluation::{Case, Cases, WithTargetFn},
    genome::plushy::{ConvertToGeneGenerator, Plushy},
    instruction::{FloatInstruction, PushInstruction, with_input::WithInputInstruction},
    push_vm::{HasStack, State, program::PushProgram, push_state::PushState, stack::StackError},
};
use rand::{prelude::Distribution, rng};

use crate::args::{CliArgs, RunModel};

/*
 * This is an implementation of the "complex regression" problem from the
 * Propeller implementation of PushGP:
 * https://github.com/lspector/propeller/blob/71d378f49fdf88c14dda88387291c9c7be0f1277/src/propeller/problems/complex_regression.cljc
 */

type Of64 = OrderedFloat<f64>;

// The penalty value to use when an evolved program doesn't have an expected
// "return" value on the appropriate stack at the end of its execution, or when
// running the program generates a fatal error.
const PENALTY_VALUE: Of64 = OrderedFloat(1e9);

/// The target polynomial is (x^3 + 1)^3 + 1
/// i.e., x^9 + 3x^6 + 3x^3 + 2
fn target_fn(input: Of64) -> Of64 {
    (input.powi(3) + 1.0).powi(3) + 1.0
}

fn run_case(program: &[PushProgram], Case { input, output }: Case<Of64>) -> Of64 {
    let Ok(start_state) = build_state(program, input).print_error() else {
        // If we fail to correctly build the initial state (because, for
        // example, the initial program is longer than the maximum size
        // of the `exec` stack), then we just return the
        // `penalty_value`.
        return PENALTY_VALUE;
    };

    let state = start_state
        .run_to_completion()
        // If running the program leads to a fatal error, then we print the error, and then extract
        // the state from the error, and compute the error using that state, i.e., the
        // values on the stacks when the error occurred.
        .print_error()
        .unwrap_or_else(IntoState::into_state);
    compute_error(&state, PENALTY_VALUE, output)
}

fn build_state(program: &[PushProgram], input: Of64) -> Result<PushState, StackError> {
    Ok(PushState::builder()
        .with_max_stack_size(1_000)
        .with_program(program.to_vec())?
        .with_float_input("x", input)
        .with_instruction_step_limit(1_000)
        .build())
}

fn compute_error(final_state: &PushState, penalty_value: Of64, expected: Of64) -> Of64 {
    final_state.stack::<Of64>().top().map_or_else(
        |_| {
            // TODO: When we introduce proper logging, we probably want to bring
            // this message back at some (generally ignored) log
            // level. eprintln!("INFO: Int stack was empty at end of
            // program evaluation");
            penalty_value
        },
        |answer| (answer - expected).abs(),
    )
}

fn score_genome(genome: &Plushy, training_cases: &Cases<Of64>) -> TestResults<ErrorValue<Of64>> {
    let program = Vec::<PushProgram>::from(genome.clone());

    training_cases
        .iter()
        .map(|&case: &Case<Of64, Of64>| run_case(&program, case))
        .collect()
}

fn main() -> miette::Result<()> {
    // FIXME: Respect the max_genome_length input
    let CliArgs {
        run_model,
        population_size,
        max_initial_instructions,
        max_generations,
        ..
    } = CliArgs::parse();

    let mut rng = rng();

    // Inputs from -4 (inclusive) to 4 (exclusive) in increments of 0.25.
    let training_cases = (-4 * 4..4 * 4)
        .map(|n| Of64::from(n) / 4.0)
        .with_target_fn(|&i| target_fn(i));

    // The range want is -4 1/8, -3 7/8, -3 5/8, ..., 3 7/8, 4 1/8.
    // I have to multiply that by 8 to get integer values, so:
    // -33, -31, -29, ..., 31, 33.
    let _testing_cases = (-33..=33)
        .step_by(2)
        .map(|n| Of64::from(n) / 8.0)
        .with_target_fn(|&i| target_fn(i));

    /*
     * The `scorer` will need to take an evolved program (sequence of
     * instructions) and run it on all the inputs from -4 (inclusive) to 4
     * (exclusive) in increments of 0.25, collecting together the errors,
     * i.e., the absolute difference between the returned value and the
     * expected value.
     */
    let scorer = FnScorer(|genome: &Plushy| score_genome(genome, &training_cases));

    let selector = Lexicase::new(training_cases.len());

    let gene_generator = uniform_distribution_of![<PushInstruction>
        FloatInstruction::Add,
        FloatInstruction::Subtract,
        FloatInstruction::Multiply,
        FloatInstruction::ProtectedDivide,
        FloatInstruction::dup(),
        FloatInstruction::push(0.0),
        FloatInstruction::push(1.0),
        WithInputInstruction::from("x")
    ]
    .into_gene_generator();

    let population: Vec<_> = gene_generator
        .to_collection(max_initial_instructions)
        .with_scorer(scorer)
        .into_collection(population_size)
        .sample(&mut rng);

    ensure!(
        !population.is_empty(),
        "An initial population is always required"
    );

    let best = Best.select(&population, &mut rng)?;
    println!("Best initial individual is {best}");

    let umad = Umad::new_with_balanced_deletion(0.1, &gene_generator);

    let make_new_individual = Select::new(selector)
        .then(GenomeExtractor)
        .then(Mutate::new(umad))
        .wrap::<GenomeScorer<_, _>>(scorer);

    let mut generation = Generation::new(make_new_individual, population);

    // TODO: It might be useful to insert some kind of logging system so we can
    // make this less imperative in nature.

    for generation_number in 0..max_generations {
        match run_model {
            RunModel::Serial => generation.serial_next()?,
            RunModel::Parallel => generation.par_next()?,
        }

        let best = Best.select(generation.population(), &mut rng)?;
        // TODO: Change 2 to be the smallest number of digits needed for
        // max_generations-1.
        println!("Generation {generation_number:2} best is {best}");
        eprintln!("INFO: Completed generation {generation_number:2}");

        if best
            .test_results
            .total()
            .is_some_and(|error| error == &OrderedFloat(0.0f64))
        {
            println!("SUCCESS");
            break;
        }
    }

    Ok(())
}
