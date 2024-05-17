pub mod args;

use std::ops::Not;

use anyhow::{ensure, Result};
use clap::Parser;
use ec_core::{
    distributions::{collection::ConvertToCollectionGenerator, conversion::IntoDistribution},
    generation::Generation,
    individual::{ec::WithScorer, scorer::FnScorer},
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
use push::{
    evaluation::cases::{Case, Cases, WithTargetFn},
    genome::plushy::{GeneGenerator, Plushy},
    instruction::{
        variable_name::VariableName, BoolInstruction, ExecInstruction, IntInstruction,
        PushInstruction,
    },
    push_vm::{program::PushProgram, push_state::PushState, stack::StackError, HasStack, State},
};
use rand::{distributions::Distribution, rngs::ThreadRng, thread_rng, RngCore};
use strum::IntoEnumIterator;

use crate::args::{Args, RunModel};

// An input for this problem is a tuple of four `i64`s.
#[derive(Copy, Clone)]
struct Input(i64, i64, i64, i64);
// An output for this problem is an `i64`.
struct Output(i64);

// This is an implementation of the "smallest" problem from Tom Helmuth's
// software synthesis benchmark suite (PSB1):
//
// T. Helmuth and L. Spector. General Program Synthesis Benchmark Suite. In
// GECCO '15: Proceedings of the 17th annual conference on Genetic and
// evolutionary computation. July 2015. ACM.
//
// Problem Source: C. Le Goues et al., "The ManyBugs and IntroClass
// Benchmarks for Automated Repair of C Programs," in IEEE Transactions on
// Software Engineering, vol. 41, no. 12, pp. 1236-1256, Dec. 1 2015.
// doi: 10.1109/TSE.2015.2454513
//
// This problem is quite easy if you have a `Min` instruction, but can be
// more a bit more difficult without that instruction.
fn main() -> Result<()> {
    let mut rng = thread_rng();

    let args = Args::parse();

    let training_cases = training_cases(&mut rng);

    let scorer = FnScorer(|genome: &Plushy| -> TestResults<test_results::Error<i64>> {
        score_genome(genome, &training_cases)
    });

    let lexicase = Lexicase::new(training_cases.len());

    let instruction_set = instructions();

    let gene_generator =
        GeneGenerator::with_uniform_close_probability(instruction_set.into_distribution()?);

    let population = gene_generator
        .to_collection_generator(args.max_initial_instructions)
        .with_scorer(scorer)
        .into_collection_generator(args.population_size)
        .sample(&mut rng);

    ensure!(population.is_empty().not());

    let best = Best.select(&population, &mut rng)?;
    println!("Best initial individual is {best}");

    let umad = Umad::new(0.1, 0.1, &gene_generator);

    let make_new_individual = Select::new(lexicase)
        .then(GenomeExtractor)
        .then(Mutate::new(umad))
        .wrap::<GenomeScorer<_, _>>(scorer);

    let mut generation = Generation::new(make_new_individual, population);

    for generation_number in 0..args.num_generations {
        match args.run_model {
            RunModel::Serial => generation.serial_next()?,
            RunModel::Parallel => generation.par_next()?,
        }

        let best = Best.select(generation.population(), &mut rng)?;
        println!("Generation {generation_number:4} best is {best}");

        if best.test_results.total_result.error == 0 {
            println!("SUCCESS");
            break;
        }
    }

    Ok(())
}

fn score_genome(
    genome: &Plushy,
    training_cases: &Cases<Input, Output>,
) -> TestResults<test_results::Error<i64>> {
    // The penalty value to use when an evolved program doesn't have an expected
    // "return" value on the appropriate stack at the end of its execution.
    const PENALTY_VALUE: i64 = 1_000;
    let program = Vec::<PushProgram>::from(genome.clone());
    let mut errors: TestResults<test_results::Error<i64>> = training_cases
        .iter()
        .map(
            |&Case {
                 input,
                 output: Output(expected),
             }: &Case<Input, Output>| {
                run_case(&program, input, PENALTY_VALUE, expected)
            },
        )
        .collect();
    // This is a total hack, but `.sum()` doesn't support saturating and thus can
    // wrap when we add up all the errors to get the total error, yielding a
    // (very) negative value. This can then "confuse" selectors like `Best`
    // into returning really "bad" individuals as the "best" in the
    // population.
    if errors.total_result.error < 0 {
        errors.total_result.error = i64::MAX;
    }
    errors
}

fn run_case(program: &[PushProgram], input: Input, penalty_value: i64, expected: i64) -> i64 {
    build_state(program, input).map_or(penalty_value, |start_state| {
        // I don't think we're properly handling things like exceeding maximum
        // stack size. I think the "Push way" here would be to take whatever
        // value is on top of the relevant stack and go with it, but we instead
        // return the penalty value.
        start_state
            .run_to_completion()
            .map_or(penalty_value, |final_state| {
                compute_error(&final_state, penalty_value, expected)
            })
    })
}

fn compute_error(final_state: &PushState, penalty_value: i64, expected: i64) -> i64 {
    final_state
        .stack::<i64>()
        .top()
        .map_or(penalty_value, |answer| {
            answer.saturating_sub(expected).abs()
        })
}

fn build_state(program: &[PushProgram], Input(a, b, c, d): Input) -> Result<PushState, StackError> {
    Ok(PushState::builder()
        .with_max_stack_size(1000)
        .with_program(program.to_vec())?
        .with_int_input("a", a)
        .with_int_input("b", b)
        .with_int_input("c", c)
        .with_int_input("d", d)
        .build())
}

fn training_inputs(rng: &mut ThreadRng) -> Vec<Input> {
    const NUM_TRAINING_CASES: usize = 100;
    (0..NUM_TRAINING_CASES)
        .map(|_| {
            Input(
                (rng.next_u32() % 100).into(),
                (rng.next_u32() % 100).into(),
                (rng.next_u32() % 100).into(),
                (rng.next_u32() % 100).into(),
            )
        })
        .collect::<Vec<_>>()
}

fn smallest(&Input(a, b, c, d): &Input) -> Output {
    #[allow(clippy::unwrap_used)]
    Output([a, b, c, d].into_iter().min().unwrap())
}

fn training_cases(rng: &mut ThreadRng) -> Cases<Input, Output> {
    training_inputs(rng).with_target_fn(smallest)
}

fn instructions() -> Vec<PushInstruction> {
    let int_instructions = IntInstruction::iter()
        // Restore this line to remove `Min` from the instruction set.
        .filter(|&i| i != IntInstruction::Min)
        .map(Into::into);
    let bool_instructions = BoolInstruction::iter().map(Into::into);
    let exec_instructions = ExecInstruction::iter()
        // The `ExecInstruction::DupBlock` instruction often leads to substantially more complicated
        // evolved programs which take much longer to run. Restore this `filter` line
        // to remove it from the instruction set.
        .filter(|&i| i != ExecInstruction::dup_block())
        .map(Into::into);
    let variables = ["a", "b", "c", "d"]
        .into_iter()
        .map(VariableName::from)
        .map(Into::into);
    int_instructions
        .chain(bool_instructions)
        .chain(exec_instructions)
        .chain(variables)
        .collect()
}
