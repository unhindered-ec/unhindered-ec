#![allow(
    clippy::unwrap_used,
    reason = "Panics are acceptable in benchmarking code"
)]

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench_group(sample_size = 1_000, max_time = 1)]
mod n_point_xo {
    use divan::Bencher;
    use ec_core::operator::recombinator::Recombinator;
    use ec_linear::recombinator::{
        n_point_xo::{NPointXoChunks, NPointXoMultiSwap, NPointXoPrimitive, NPointXoWindows},
        two_point_xo::TwoPointXo,
    };

    const NUM_CROSSOVERS: [usize; 6] = [1, 2, 4, 8, 16, 32];
    const GENOME_SIZE: usize = 10_000;

    #[divan::bench(consts = NUM_CROSSOVERS)]
    fn chunks<const N: usize>(bencher: Bencher) {
        bencher
            .with_inputs(|| {
                let first_parent = vec![0; GENOME_SIZE];
                let second_parent = vec![1; GENOME_SIZE];

                let crossover_operator = NPointXoChunks::<N>::new().unwrap();

                let rng = rand::rng();

                ([first_parent, second_parent], crossover_operator, rng)
            })
            .bench_values(|(parents, crossover_operator, mut rng)| {
                crossover_operator
                    .recombine(divan::black_box(parents), &mut rng)
                    .unwrap()
            });
    }

    #[divan::bench(consts = NUM_CROSSOVERS)]
    fn multi_swap<const N: usize>(bencher: Bencher) {
        bencher
            .with_inputs(|| {
                let first_parent = vec![0; GENOME_SIZE];
                let second_parent = vec![1; GENOME_SIZE];

                let crossover_operator = NPointXoMultiSwap::<N>::new().unwrap();

                let rng = rand::rng();

                ([first_parent, second_parent], crossover_operator, rng)
            })
            .bench_values(|(parents, crossover_operator, mut rng)| {
                crossover_operator
                    .recombine(divan::black_box(parents), &mut rng)
                    .unwrap()
            });
    }

    #[divan::bench(consts = NUM_CROSSOVERS)]
    fn windows<const N: usize>(bencher: Bencher) {
        bencher
            .with_inputs(|| {
                let first_parent = vec![0; GENOME_SIZE];
                let second_parent = vec![1; GENOME_SIZE];

                let crossover_operator = NPointXoWindows::<N>::new().unwrap();

                let rng = rand::rng();

                ([first_parent, second_parent], crossover_operator, rng)
            })
            .bench_values(|(parents, crossover_operator, mut rng)| {
                crossover_operator
                    .recombine(divan::black_box(parents), &mut rng)
                    .unwrap()
            });
    }

    #[divan::bench(consts = NUM_CROSSOVERS)]
    fn primitive<const N: usize>(bencher: Bencher) {
        bencher
            .with_inputs(|| {
                let first_parent = vec![0; GENOME_SIZE];
                let second_parent = vec![1; GENOME_SIZE];

                let crossover_operator = NPointXoPrimitive::<N>::new().unwrap();

                let rng = rand::rng();

                ([first_parent, second_parent], crossover_operator, rng)
            })
            .bench_values(|(parents, crossover_operator, mut rng)| {
                crossover_operator
                    .recombine(divan::black_box(parents), &mut rng)
                    .unwrap()
            });
    }

    #[divan::bench]
    fn two_point(bencher: Bencher) {
        bencher
            .with_inputs(|| {
                let first_parent = vec![0; GENOME_SIZE];
                let second_parent = vec![1; GENOME_SIZE];

                let crossover_operator = TwoPointXo;

                let rng = rand::rng();

                ([first_parent, second_parent], crossover_operator, rng)
            })
            .bench_values(|(parents, crossover_operator, mut rng)| {
                crossover_operator
                    .recombine(divan::black_box(parents), &mut rng)
                    .unwrap()
            });
    }
}
