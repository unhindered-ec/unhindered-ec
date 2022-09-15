#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use rust_ga::{population::Population, bitstring::{hiff, count_ones}};
use rust_ga::do_main;

pub fn main() {
    do_main()
}

#[cfg(test)]
mod tests {
    use rust_ga::population::Population;
    use rust_ga::individual::Individual;
    use rust_ga::bitstring::count_ones;
    use rust_ga::bitstring::hiff;
    // use rand::rngs::StdRng;
    // use rand::SeedableRng;
    // use std::time::Instant;
    // use test::Bencher;

    #[test]
    fn test_count_ones() {
        let bits = vec![true, false, true, false, true, false, true, false];
        assert_eq!(count_ones(&bits), 4);
    }

    #[test]
    fn test_hiff() {
        let bits = vec![true, false, false, false, true, true, true, true];
        assert_eq!(hiff(&bits), 18);
    }

    #[test]
    fn test_individual_new() {
        let mut rng = rand::thread_rng();
        let ind = Individual::new_bitstring(128, count_ones, &mut rng);
        assert_eq!(ind.genome.len(), 128);
        assert_eq!(ind.score, count_ones(&ind.genome));
    }

    #[test]
    fn test_population_new_count_ones() {
        let pop = Population::new_bitstring_population(100, 128, count_ones);
        assert_eq!(pop.individuals.len(), 100);
        assert_eq!(pop.individuals[0].genome.len(), 128);
        assert_eq!(pop.individuals[0].score, count_ones(&pop.individuals[0].genome));
    }

    #[test]
    fn test_population_new_hiff() {
        let pop = Population::new_bitstring_population(100, 128, hiff);
        assert_eq!(pop.individuals.len(), 100);
        assert_eq!(pop.individuals[0].genome.len(), 128);
        assert_eq!(pop.individuals[0].score, hiff(&pop.individuals[0].genome));
    }
}
