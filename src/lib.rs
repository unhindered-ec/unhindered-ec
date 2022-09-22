use rand::rngs::ThreadRng;

use crate::{bitstring::{hiff, Bitstring, LinearCrossover, LinearMutation}, population::{Population, Selector}, generation::Generation, individual::Individual};

pub mod individual;
pub mod population;
pub mod generation;
pub mod bitstring;

pub fn do_main() {
    let scorer = hiff;

    let binary_tournament = Population::<Bitstring>::make_tournament_selector(2);
    let decimal_tournament = Population::<Bitstring>::make_tournament_selector(10);
    let selectors: Vec<&Selector<Bitstring>> 
        = vec![&Population::best_score,
               &Population::random, 
               &binary_tournament,
               &binary_tournament,
               &decimal_tournament];

    let population
        = Population::new_bitstring_population(
            1000, 
            128, 
            scorer);
    assert!(!population.is_empty());

    let make_child = move |rng: &mut ThreadRng, generation: &Generation<Bitstring>| {
            // These two `unwrap()`s are OK because we've asserted that the set of selectors
            // isn't empty.
            #[allow(clippy::unwrap_used)]
            let first_parent = generation.get_parent(rng); // parent_selector.get(rng).unwrap();
            #[allow(clippy::unwrap_used)]
            let second_parent = generation.get_parent(rng); // parent_selector.get(rng).unwrap();

            let genome
                = first_parent.genome
                .two_point_xo(&second_parent.genome, rng)
                .mutate_one_over_length(rng);
            let score = scorer(&genome);
            Individual { genome, score }
            // todo!()
    };

    let mut generation = Generation::new(
        population,
        &selectors,
        &make_child
    );

    assert!(!generation.population.is_empty());
    let best = generation.best_individual();
    println!("{:?}", best);
    println!("Pop size = {}", generation.population.size());
    println!("Bit length = {}", best.genome.len());

    (0..100).for_each(|generation_number| {
        generation = generation.par_next();
        let best = generation.best_individual();
        println!("Generation {} best is {:?}", generation_number, best);
    });
}
