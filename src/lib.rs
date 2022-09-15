use crate::{bitstring::{hiff, Bitstring}, population::{Population, Selector}};

pub mod individual;
pub mod population;
pub mod bitstring;

pub fn do_main() {
    let scorer = hiff;
    let mut population
        = Population::new_bitstring_population(
            1000, 
            128, 
            scorer);
    assert!(!population.individuals.is_empty());
    #[allow(clippy::unwrap_used)]
    let best = population.best_individual();
    println!("{:?}", best);
    println!("Pop size = {}", population.individuals.len());
    println!("Bit length = {}", best.genome.len());

    let binary_tournament = Population::<Bitstring>::make_tournament_selector(2);
    let decimal_tournament = Population::<Bitstring>::make_tournament_selector(10);

    let selectors: Vec<&Selector<Bitstring>> 
        = vec![&Population::best_score,
               &Population::random, 
               &binary_tournament,
               &binary_tournament,
               &decimal_tournament];

    (0..100).for_each(|generation| {
        population = population.next_generation_with_selectors(&selectors, scorer);
        let best = population.best_individual();
        println!("Generation {} best is {:?}", generation, best);
        println!("Pop size = {}", population.individuals.len());
        println!("Bit length = {}", best.genome.len());
    });
}
