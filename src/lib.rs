use crate::{bitstring::hiff, population::Population};

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
    // println!("{:?}", best);
    // println!("Pop size = {}", population.individuals.len());
    // println!("Bit length = {}", best.genome.len());

    (0..100).for_each(|generation| {
        population = population.next_generation(scorer);
        let best = population.best_individual();
        // println!("Generation {} best is {:?}", generation, best);
        // println!("Pop size = {}", population.individuals.len());
        // println!("Bit length = {}", best.genome.len());
    });
}
