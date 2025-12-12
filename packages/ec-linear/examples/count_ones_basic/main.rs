use ec_core::{
    distributions::collection::ConvertToCollectionDistribution,
    generation::Generation,
    individual::{ec::WithScorer, scorer::FnScorer},
    operator::{
        Composable,
        genome_extractor::GenomeExtractor,
        genome_scorer::GenomeScorer,
        mutator::Mutate,
        recombinator::Recombine,
        selector::{Select, Selector, best::Best, tournament::Tournament},
    },
    test_results::Score,
};
use ec_linear::{
    genome::bitstring::Bitstring, mutator::with_one_over_length::WithOneOverLength,
    recombinator::two_point_xo::TwoPointXo,
};
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
    // To create a starting population we generate a Distribution of Populations
    // from which we can then sample a single initial, random population:
    //
    // - start with the `StandardUniform` distribution which, among other things,
    //   allows sampling of bools
    // - turn that into a (uniform) Distribution of bitstring genomes
    // - score each of these genomes to get a distribution of individuals (scored
    //   genomes)
    // - turn that into a distribution of collections of individuals, i.e. a
    //   (uniform) Distribution of Populations
    //
    // Finally we sample a single initial population from that
    // distribution.
    let initial_population: Vec<_> = StandardUniform // impl Distribution<bool>
        .to_collection(bit_length) // impl Distribution<Bitstring>
        .with_scorer(scorer) // impl Distribution<Individual>
        .into_collection(population_size) // impl Distribution<Population>
        .sample(&mut rng); // a specific Population

    // Create a pipeline that takes a population and generates a new individual.
    //
    // This is used below to create new individuals for the next generation from the
    // previous generation:
    //
    // - Start with the selector that will be used to select parent individuals from
    //   the previous generation
    // - Apply that twice to generate a pair of parents
    // - Map the `GenomeExtractor` to get a pair of genomes from those parents
    // - Recombine those genomes using two-point crossover, generating a new genome
    // - Mutate the new genome using `WithOneOverLength`, which flips bits with a
    //   probability of 1/N, where N is the length of the genome
    // - Score the mutated genome to create an individual
    let make_new_individual = Select::new(selector)
        .apply_twice()
        .map(GenomeExtractor)
        .then(Recombine::new(TwoPointXo))
        .then(Mutate::new(WithOneOverLength))
        .wrap::<GenomeScorer<_, _>>(scorer);

    // Create the initial generation from the operator pipeline and the initial
    // population
    let mut generation = Generation::new(make_new_individual, initial_population);

    // Run evolution for `max_generations`.
    for generation_number in 0..max_generations {
        // Update the `generation` in place, parallelized for each individual.
        generation.par_next()?;

        // Select a "best" individual, i.e., an individual with the highest score.
        let best = Best.select(generation.population(), &mut rng)?;
        // Print that best individual.
        println!("Generation {generation_number:3} best is {best}");
    }

    Ok(())
}

// The scoring function.
//
// For Count Ones, the scoring function just counts the number of `true` values
// in the given bitstring.
#[must_use]
pub fn count_ones(bits: &[bool]) -> Score<i64> {
    bits.iter().copied().map(i64::from).sum()
}
