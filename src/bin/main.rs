#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use clap::Parser;
use rust_ga::args::Args;
use rust_ga::do_main;

fn main() {
    let args = Args::parse();

    do_main(args);
}

#[cfg(test)]
mod tests {
    use rust_ga::bitstring::count_ones;
    use rust_ga::bitstring::fitness_vec_to_test_results;
    use rust_ga::bitstring::hiff;
    use rust_ga::individual::ec::EcIndividual;
    use rust_ga::population::VecPop;

    #[test]
    fn test_count_ones() {
        let bits = vec![true, false, true, false, true, false, true, false];
        assert_eq!(count_ones(&bits), vec![1, 0, 1, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn test_hiff() {
        let bits = vec![true, false, false, false, true, true, true, true];
        assert_eq!(
            hiff(&bits),
            vec![1, 1, 0, 1, 1, 2, 0, 1, 1, 2, 1, 1, 2, 4, 0]
        );
    }

    #[test]
    fn test_individual_new() {
        let mut rng = rand::thread_rng();
        let ind = EcIndividual::new_bitstring(
            128,
            |bits| fitness_vec_to_test_results(count_ones(bits)),
            &mut rng,
        );
        assert_eq!(ind.genome().len(), 128);
        assert_eq!(ind.test_results().results, count_ones(&ind.genome()));
        assert_eq!(
            ind.test_results().total_result,
            count_ones(&ind.genome()).iter().sum()
        );
    }

    #[test]
    fn test_population_new_count_ones() {
        let pop = VecPop::new_bitstring_population(100, 128, |bits| {
            fitness_vec_to_test_results(count_ones(bits))
        });
        assert_eq!(pop.size(), 100);
        let first_individual = pop.iter().next().unwrap();
        assert_eq!(first_individual.genome().len(), 128);
        assert_eq!(
            first_individual.test_results().results,
            count_ones(first_individual.genome())
        );
        assert_eq!(
            first_individual.test_results().total_result,
            count_ones(first_individual.genome()).iter().sum()
        );
    }

    #[test]
    fn test_population_new_hiff() {
        let pop = VecPop::new_bitstring_population(100, 128, |bits| {
            fitness_vec_to_test_results(hiff(bits))
        });
        assert_eq!(pop.size(), 100);
        let first_individual = pop.iter().next().unwrap();
        assert_eq!(first_individual.genome().len(), 128);
        assert_eq!(
            first_individual.test_results().results,
            hiff(&first_individual.genome())
        );
        assert_eq!(
            first_individual.test_results().total_result,
            hiff(&first_individual.genome()).iter().sum()
        );
    }
}
