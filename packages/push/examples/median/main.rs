pub mod args;

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
use push::{
    evaluation::{Case, Cases, WithTargetFn},
    genome::plushy::{GeneGenerator, Plushy},
    instruction::{
        BoolInstruction, ExecInstruction, IntInstruction, PushInstruction,
        variable_name::VariableName,
    },
    push_vm::{HasStack, State, program::PushProgram, push_state::PushState, stack::StackError},
};
use rand::{
    Rng,
    distr::{Distribution, Uniform},
    rng,
};
use strum::IntoEnumIterator;

use crate::args::{CliArgs, RunModel};

#[derive(Copy, Clone)]
struct Input([i64; 3]);

#[derive(Copy, Clone)]
struct Output(i64);

impl Input {
    fn median(&self) -> Output {
        let Self(mut input) = self;
        input.sort_unstable();
        Output(input[1])
    }
}

impl Distribution<Input> for Uniform<i64> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Input {
        Input(std::array::from_fn(|_| self.sample(rng)))
    }
}

// This is an implementation of the "median" problem from Tom Helmuth's
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
// This problem is quite easy if you have both `Min` and `Max` instructions, but
// can be more difficult without those instruction.
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

    let training_cases = Uniform::new(lower_input_bound, upper_input_bound)
        .into_diagnostic()?
        .sample_iter(&mut rng)
        .take(num_training_cases)
        .with_target_fn(Input::median);

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
        "An initial populaiton is always required"
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
    penalty_value: i128,
) -> TestResults<test_results::Error<i128>> {
    let program = Vec::<PushProgram>::from(genome.clone());
    training_cases
        .iter()
        .map(|&case: &Case<Input, Output>| run_case(case, &program, penalty_value))
        .collect()
}

fn run_case(
    Case {
        input,
        output: Output(expected),
    }: Case<Input, Output>,
    program: &[PushProgram],
    penalty_value: i128,
) -> i128 {
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

fn build_state(program: &[PushProgram], Input([a, b, c]): Input) -> Result<PushState, StackError> {
    Ok(PushState::builder()
        .with_max_stack_size(1000)
        .with_program(program.to_vec())?
        .with_int_input("a", a)
        .with_int_input("b", b)
        .with_int_input("c", c)
        .with_instruction_step_limit(1_000)
        .build())
}

fn compute_error(final_state: &PushState, penalty_value: i128, expected: i64) -> i128 {
    final_state
        .stack::<i64>()
        .top()
        .map_or(penalty_value, |answer| {
            i128::from(*answer)
                .saturating_sub(i128::from(expected))
                .abs()
        })
}

fn instructions() -> impl Iterator<Item = PushInstruction> {
    let int_instructions = IntInstruction::iter()
        // Restore this line to remove `Min` from the instruction set.
        // .filter(|&i| i != IntInstruction::Min)
        // Restore this line to remove `Clamp` from the instruction set.
        // .filter(|&i| i != IntInstruction::clamp())
        .map(Into::into);
    let bool_instructions = BoolInstruction::iter().map(Into::into);
    let exec_instructions = ExecInstruction::iter()
        // The `ExecInstruction::DupBlock` instruction often leads to substantially more complicated
        // evolved programs which take much longer to run. Restore this `filter` line
        // to remove it from the instruction set.
        // .filter(|&i| i != ExecInstruction::dup_block())
        .map(Into::into);

    let variables = ["a", "b", "c"]
        .into_iter()
        .map(VariableName::from)
        .map(Into::into);

    int_instructions
        .chain(bool_instructions)
        .chain(exec_instructions)
        .chain(variables)
}
