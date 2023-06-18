#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

pub mod args;

use crate::args::{Args, RunModel, TargetProblem};
use anyhow::{ensure, Result};
use clap::Parser;
use ec_core::{
    generation::Generation,
    generator::Generator,
    individual::ec::{self, EcIndividual},
    operator::{
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        recombinator::Recombine,
        selector::{
            best::Best, lexicase::Lexicase, tournament::Tournament, weighted::Weighted, Select,
            Selector,
        },
        Composable,
    },
    population::{self},
    test_results::{self, TestResults},
};
use ec_linear::{
    genome::{
        bitstring::{BitContext, Bitstring},
        demo_scorers::{count_ones, hiff},
        LinearContext,
    },
    mutator::with_one_over_length::WithOneOverLength,
    recombinator::two_point_xo::TwoPointXo,
};
use rand::thread_rng;
use std::ops::Not;

fn main() -> Result<()> {
    // Using `Error` in `TestResults<Error>` will have the run favor smaller
    // values, where using `Score` (e.g., `TestResults<Score>`) will have the run
    // favor larger values.
    type Pop = Vec<EcIndividual<Bitstring, TestResults<test_results::Score>>>;

    let args = Args::parse();

    let base_scorer = match args.target_problem {
        TargetProblem::CountOnes => count_ones,
        TargetProblem::Hiff => hiff,
    };
    let scorer = |bitstring: &Bitstring| base_scorer(&bitstring.bits);

    let num_test_cases = match args.target_problem {
        TargetProblem::CountOnes => args.bit_length,
        TargetProblem::Hiff => 2 * args.bit_length - 1,
    };

    let lexicase = Lexicase::new(num_test_cases);
    let binary_tournament = Tournament::new(2);

    let selector: Weighted<Pop> = Weighted::new(Best, 1)
        .with_selector(lexicase, 5)
        .with_selector(binary_tournament, args.population_size - 1);

    let mut rng = thread_rng();

    let bitstring_context = LinearContext {
        length: args.bit_length,
        element_context: BitContext { probability: 0.5 },
    };

    let individual_context = ec::GeneratorContext {
        genome_context: bitstring_context,
        scorer,
    };

    let population_context = population::GeneratorContext {
        population_size: args.population_size,
        individual_context,
    };
    let population = population_context.generate(&mut rng)?;

    ensure!(population.is_empty().not());

    println!("{population:?}");

    // Let's assume the process will be generational, i.e., we replace the entire
    // population with newly created/selected individuals every generation.
    // `generation` will be a mutable operator (containing the data structures for
    // the population(s) and recombinators, scorers, etc.) that acts on a population
    // returning a new population. We'll have different generation operators for
    // serial vs. parallel generation of new individuals.

    let make_new_individual = Select::new(selector)
        .apply_twice()
        .then_map(GenomeExtractor)
        .then(Recombine::new(TwoPointXo))
        .then(Mutate::new(WithOneOverLength))
        .wrap::<GenomeScorer<_, _>>(scorer);

    // generation::new() will take
    //   * a pipeline that gets us from population -> new individual
    //   * an initial population.
    let mut generation = Generation::new(make_new_individual, population);

    // TODO: It might be useful to insert some kind of logging system so we can
    //   make this less imperative in nature.

    (0..args.num_generations).try_for_each(|generation_number| {
        match args.run_model {
            RunModel::Serial => generation.serial_next()?,
            RunModel::Parallel => generation.par_next()?,
        }

        let best = Best.select(generation.population(), &mut rng)?;
        // TODO: Change 2 to be the smallest number of digits needed for
        //  args.num_generations-1.
        println!("Generation {generation_number:2} best is {best:?}");

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}
