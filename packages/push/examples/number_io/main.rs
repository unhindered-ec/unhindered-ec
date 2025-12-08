pub mod args;

use std::fmt::{Display, Formatter};

use clap::Parser;
use ec_core::{
    distributions::{collection::ConvertToCollectionGenerator, conversion::IntoDistribution},
    generation::Generation,
    individual::{ec::WithScorer, scorer::FnScorer},
    operator::{
        Composable,
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        selector::{Select, Selector, best::Best, lexicase::Lexicase},
    },
    test_results::{self, TestResults},
};
use ec_linear::mutator::umad::Umad;
use miette::{IntoDiagnostic, ensure};
use ordered_float::OrderedFloat;
use push::{
    evaluation::{Case, Cases, WithTargetFn},
    genome::plushy::{GeneGenerator, Plushy},
    instruction::{FloatInstruction, IntInstruction, PushInstruction, variable_name::VariableName},
    push_vm::{State, program::PushProgram, push_state::PushState, stack::StackError},
};
use rand::{
    distr::{Distribution, Uniform},
    rng,
};
use strsim::damerau_levenshtein;
use strum::IntoEnumIterator;

use crate::args::{CliArgs, RunModel};

// An input for this problem is a single integer
// (`i64`) and float (`f64`).
#[derive(Debug, Copy, Clone)]
struct Input {
    i: i64,
    f: f64,
}

impl Input {
    #[expect(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "These conversions should generally be safe because the default values are in \
                  the range -100..100"
    )]
    fn number_io(&self) -> Output {
        let Self { i, f } = self;
        let expected_output = (*i as f64 + f).to_string();
        Output(expected_output)
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.i, self.f)
    }
}

impl Distribution<Input> for (Uniform<i64>, Uniform<f64>) {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Input {
        let i = self.0.sample(rng);
        let f = self.1.sample(rng);
        Input { i, f }
    }
}

// An output for this problem is a `String`, namely the
// value that is "printed" to the `PushState::Stdout`.
#[derive(Debug, Clone)]
struct Output(String);

// This is an implementation of the Number IO problem from Tom Helmuth's
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
// This problem is quite easy as it only requires teh ability to convert
// an integer value to a float, add the two values, and print the result.
fn main() -> miette::Result<()> {
    let CliArgs {
        run_model,
        population_size,
        max_initial_instructions,
        max_generations,
        num_training_cases,
        lower_input_bound,
        upper_input_bound,
        penalty_value,
        ..
    } = CliArgs::parse();

    let mut rng = rng();

    #[expect(
        clippy::as_conversions,
        clippy::cast_precision_loss,
        reason = "I'm OK with potentially losing precision here because the bounds are usually \
                  small and easily convert to `f64`s."
    )]
    let training_cases = (
        Uniform::new(lower_input_bound, upper_input_bound).into_diagnostic()?,
        Uniform::new(lower_input_bound as f64, upper_input_bound as f64).into_diagnostic()?,
    )
        .sample_iter(&mut rng)
        .take(num_training_cases)
        .with_target_fn(Input::number_io);

    let scorer = FnScorer(|genome: &Plushy| score_genome(genome, &training_cases, penalty_value));

    let lexicase = Lexicase::new(training_cases.len());

    let instruction_set = instructions().collect::<Vec<_>>();

    let gene_generator =
        GeneGenerator::with_uniform_close_probability(instruction_set.into_distribution()?);

    let population = gene_generator
        .to_collection_generator(max_initial_instructions)
        .with_scorer(scorer)
        .into_collection_generator(population_size)
        .sample(&mut rng);

    ensure!(
        !population.is_empty(),
        "An initial population is always required"
    );

    let best = Best.select(&population, &mut rng)?;
    println!("Best initial individual is {best}");

    let umad = Umad::new_with_balanced_deletion(0.1, &gene_generator);

    let make_new_individual = Select::new(lexicase)
        .then(GenomeExtractor)
        .then(Mutate::new(umad))
        .wrap::<GenomeScorer<_, _>>(scorer);

    let mut generation = Generation::new(make_new_individual, population);

    for generation_number in 0..max_generations {
        match run_model {
            RunModel::Serial => generation.serial_next()?,
            RunModel::Parallel => generation.par_next()?,
        }

        let best = Best.select(generation.population(), &mut rng)?;
        println!("Generation {generation_number:4} best is {best}");

        #[expect(
            clippy::unwrap_used,
            reason = "There should be at least one training case, so this shouldn't fail"
        )]
        let first_input = *training_cases.inputs().next().unwrap();
        #[expect(
            clippy::unwrap_used,
            reason = "The 'best' program should run successfully"
        )]
        let mut state = build_state(&Vec::<PushProgram>::from(best.genome.clone()), first_input)?
            .run_to_completion()
            .unwrap();
        #[expect(
            clippy::unwrap_used,
            reason = "The 'best' program should run successfully"
        )]
        let output = state.stdout_string().unwrap();
        println!("Stdout for input {first_input}: {output}");

        if best.test_results.total().is_some_and(|error| error == &0) {
            println!("SUCCESS");
            break;
        }
    }

    Ok(())
}

fn score_genome(
    genome: &Plushy,
    training_cases: &Cases<Input, Output>,
    penalty_value: usize,
) -> TestResults<test_results::Error<usize>> {
    let program = Vec::<PushProgram>::from(genome.clone());
    training_cases
        .iter()
        .map(|case: &Case<Input, Output>| run_case(case, &program, penalty_value))
        .collect()
}

fn run_case(
    Case {
        input,
        output: Output(expected),
    }: &Case<Input, Output>,
    program: &[PushProgram],
    penalty_value: usize,
) -> usize {
    build_state(program, *input).map_or(penalty_value, |start_state| {
        // I don't think we're properly handling things like exceeding maximum
        // stack size. I think the "Push way" here would be to take whatever
        // value is on top of the relevant stack and go with it, but we instead
        // return the penalty value.
        start_state
            .run_to_completion()
            .map_or(penalty_value, |mut final_state| {
                compute_error(&mut final_state, penalty_value, expected)
            })
    })
}

fn build_state(program: &[PushProgram], Input { i, f }: Input) -> Result<PushState, StackError> {
    Ok(PushState::builder()
        .with_max_stack_size(1000)
        .with_program(program.to_vec())?
        .with_int_input("i", i)
        .with_float_input("f", OrderedFloat(f))
        .with_instruction_step_limit(1_000)
        .build())
}

fn compute_error(final_state: &mut PushState, penalty_value: usize, expected: &str) -> usize {
    let Ok(output) = final_state.stdout_string() else {
        return penalty_value;
    };
    damerau_levenshtein(&output, expected)
}

fn instructions() -> impl Iterator<Item = PushInstruction> {
    let int_instructions = IntInstruction::iter().map(Into::into);
    let float_instruction = FloatInstruction::iter().map(Into::into);

    let variables = ["i", "f"]
        .into_iter()
        .map(VariableName::from)
        .map(Into::into);

    int_instructions.chain(float_instruction).chain(variables)
}
