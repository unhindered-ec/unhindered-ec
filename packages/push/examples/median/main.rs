pub mod args;

use std::ops::Not;

use anyhow::{ensure, Result};
use clap::Parser;
use ec_core::{
    generation::Generation,
    generator::{collection::ConvertToCollectionGenerator, Generator},
    individual::{
        ec::{EcIndividual, WithScorer},
        scorer::FnScorer,
    },
    operator::{
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        selector::{
            best::Best, lexicase::Lexicase, weighted::Weighted, Select,
            Selector,
        },
        Composable,
    },
    test_results::{self, TestResults},
};
use ec_linear::mutator::umad::Umad;
use push::{
    genome::plushy::{GeneGenerator, Plushy},
    instruction::{variable_name::VariableName, BoolInstruction, IntInstruction, PushInstruction},
    push_vm::{program::PushProgram, push_state::PushState, HasStack, State},
    vec_into,
};
use rand::{rngs::ThreadRng, thread_rng, Rng,};

use crate::args::{Args, RunModel};

fn training_inputs(num_cases: usize, rng: &mut ThreadRng) -> Vec<(i8, i8, i8)> {
    // Inputs from in the range [-100, 100] inclusive
    (0..num_cases)
    .map(|_| {
        (
            rng.gen_range(-100..=100),
            rng.gen_range(-100..=100),
            rng.gen_range(-100..=100),
        )
    })
    .collect()
}

fn median((x, y, z): (i8, i8, i8)) -> i8 {
    let mut sorted_values = [x, y, z];
    sorted_values.sort();
    return sorted_values[1];
}

fn training_cases(num_cases: usize, rng: &mut ThreadRng) -> Vec<((i8, i8, i8), i8)> {
    let inputs = training_inputs(num_cases, rng);
    inputs.into_iter().map(|input| (input, median(input))).collect()
}


fn main() -> Result <()> {
    let args = Args::parse();

    type Pop = Vec<EcIndividual<Plushy, TestResults<test_results::Error<i64>>>>;
    let mut rng = thread_rng();

    let penalty_value: i64 = 1_000;

    let training_cases = training_cases(args.population_size, &mut rng);

    println!("Training cases: {training_cases:#?}");

    // We're defining a scorer function to be used to score a given generation
    let scorer = FnScorer(|genome: &Plushy| -> TestResults<test_results::Error<i8>> {
        // We need to clone our program so we can score it.
        let program = Vec::<PushProgram>::from(genome.clone());
        let errors: TestResults<test_results::Error<i8>> = training_cases
            .iter()
            .map(|&((a, b, c), expected)| {
                #[allow(clippy::unwrap_used)]
                let state = PushState::builder()
                .with_max_stack_size(args.max_initial_instructions)
                .with_program(program.clone())
                .unwrap()
                    .with_int_input("a", a.into())
                    .with_int_input("b", b.into())
                    .with_int_input("c", c.into())
                    .build();

                    match state.run_to_completion() {
                        Ok(final_state) => final_state
                        .stack::<i8>()
                        .top()
                        .map_or(penalty_value, |answer | (answer - expected).abs()),
                        Err(_) => {
                            penalty_value
                        }
                    }
                
                
            })
            .collect();
    errors
    });

    let num_test_cases = training_cases.len();
    let lexicase = Lexicase::new(num_test_cases);

    let selector: Weighted<Pop> = Weighted::new(Best, 1)
    .with_selector(lexicase, 5);

    let gene_generator = GeneGenerator::with_uniform_close_probability(instructions());

    let population = gene_generator
    .to_collection_generator(args.max_initial_instructions)
    .with_scorer(scorer)
    .into_collection_generator(args.population_size)
    .generate(&mut rng)?;

ensure!(population.is_empty().not());

let best = Best.select(&population, &mut rng)?;
println!("Best initial individual is {best:?}");

let umad = Umad::new(0.1, 0.1, &gene_generator);

let make_new_individual = Select::new(selector)
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
    println!("Generation {generation_number:2} best is {best:#?}");

    if best.test_results.total_result.error == 0 {
        println!("SUCCESS");
        break;
    }
}
    Ok(())
}

fn instructions() -> Vec<PushInstruction> {
    // Use `strum` to generate this list of instructions.
    vec_into![
        IntInstruction::Negate,
        IntInstruction::Abs,
        IntInstruction::Min,
        IntInstruction::Max,
        IntInstruction::Inc,
        IntInstruction::Dec,
        IntInstruction::Add,
        IntInstruction::Subtract,
        IntInstruction::Multiply,
        IntInstruction::ProtectedDivide,
        IntInstruction::Mod,
        IntInstruction::Power,
        IntInstruction::Square,
        IntInstruction::IsZero,
        IntInstruction::IsPositive,
        IntInstruction::IsNegative,
        IntInstruction::IsEven,
        IntInstruction::IsOdd,
        IntInstruction::Equal,
        IntInstruction::NotEqual,
        IntInstruction::LessThan,
        IntInstruction::LessThanEqual,
        IntInstruction::GreaterThan,
        IntInstruction::GreaterThanEqual,
        IntInstruction::FromBoolean,
        BoolInstruction::Not,
        BoolInstruction::Or,
        BoolInstruction::And,
        BoolInstruction::Xor,
        BoolInstruction::Implies,
        BoolInstruction::FromInt,
        // ExecInstruction::IfElse,
        VariableName::from("a"),
        VariableName::from("b"),
        VariableName::from("c"),
    ]
}
