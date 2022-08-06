use rust_ga::{population::Population};

fn main() {
    let population = Population::new_bitstring(100, 128);
    let best = population.individuals.iter().max_by_key(
        |ind| ind.fitness
    ).unwrap();
    println!("{:?}", best);
}

#[cfg(test)]
mod tests {
    use rust_ga::population::Population;
    use rust_ga::individual::Individual;
    use rust_ga::bitstring::{count_ones, count_ones_vec};
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
    fn test_individual_new() {
        let mut rng = rand::thread_rng();
        let ind = Individual::new_bitstring(128, count_ones_vec, &mut rng);
        assert_eq!(ind.genome.len(), 128);
        assert_eq!(ind.fitness, count_ones(&ind.genome));
    }

    #[test]
    fn test_population_new() {
        let pop = Population::new_bitstring(100, 128);
        assert_eq!(pop.individuals.len(), 100);
        assert_eq!(pop.individuals[0].genome.len(), 128);
        assert_eq!(pop.individuals[0].fitness, count_ones(&pop.individuals[0].genome));
    }
}
