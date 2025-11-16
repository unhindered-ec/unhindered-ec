use ec_core::{
    distributions::collection::ConvertToCollectionGenerator,
    generation::Generation,
    individual::{ec::WithScorer, scorer::FnScorer},
    operator::{
        Composable,
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        recombinator::Recombine,
        selector::{
            Select, Selector, best::Best,
            tournament::Tournament,
        },
    },
    test_results::{Score, TestResults},
};
use ec_linear::{
    genome::bitstring::Bitstring, mutator::with_one_over_length::WithOneOverLength,
    recombinator::two_point_xo::TwoPointXo,
};
// use miette::ensure;
use rand::{
    distr::{Distribution, StandardUniform},
    rng,
};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set some basic parameters for the EC run
    // The number of individuals in the population
    let population_size = 100;
    // The number of bits in the evolved bitstring
    let bit_length = 100;
    // The maximum number of generations for this EC run
    let max_generations = 100;

    // Create an instance of a random number generator
    let mut rng = rng();

    // Create a scorer from the scoring function above. For Count Ones, this just
    // counts the number of `true` values in the bitstring.
    let scorer = FnScorer(|bitstring: &Bitstring| count_ones(&bitstring.bits));

    // Use binary tournament selection to select parents
    let selector = Tournament::binary();

    // Creating an initial population.
    //
    // To create a individual population we generate a Distribution of Populations
    // from which we can then sample a single initial, random population:
    // - start with the `StandardUniform` distribution which among other things
    //   allows sampling of bools
    // - turn that into a Distribution of bitstring genomes
    // - score each of these genomes to get a distribution of individuals
    // - turn that into a distribution of a collection of individuals, aka a
    //   Population
    //
    // and then finally we sample a single initial population from that
    // distribution.
    let initial_population = StandardUniform // impl Distribution<bool>
        .to_collection_generator(bit_length) // impl Distribution<Bitstring>
        .with_scorer(scorer) // impl Distribution<Individual>
        .into_collection_generator(population_size) // impl Distribution<Population>
        .sample(&mut rng); // a specific Population

    println!("{initial_population:?}");

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
    let mut generation = Generation::new(make_new_individual, initial_population);

    // TODO: It might be useful to insert some kind of logging system so we can
    //   make this less imperative in nature.

    for generation_number in 0..max_generations {
        generation.par_next()?;

        let best = Best.select(generation.population(), &mut rng)?;
        // TODO: Change 2 to be the smallest number of digits needed for
        //  max_generations-1.
        println!("Generation {generation_number:2} best is {best}");
    }

    Ok(())
}

// The scoring function which, for Count Ones, just counts the number of
// `true` values in the given bitstring.
#[must_use]
pub fn count_ones(bits: &[bool]) -> TestResults<Score<i64>> {
    bits.iter().copied().map(i64::from).collect()
}

#[cfg(test)]
mod test {
    use ec_core::test_results::{self, TestResults};

    use super::count_ones;

    #[test]
    fn non_empty() {
        let input = [false, true, true, true, false, true];
        let output: TestResults<test_results::Score<i64>> =
            [0, 1, 1, 1, 0, 1].into_iter().collect();
        assert_eq!(output, count_ones(&input));
    }
}
