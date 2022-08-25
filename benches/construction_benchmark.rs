use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_ga::{population::Population, bitstring::{count_ones, hiff}};

fn benchmark_construction_count_ones(c: &mut Criterion) {
    c.bench_function(
        "Construct population count_ones", 
        |b| b.iter(|| Population::new_bitstring_population(
            black_box(1000), 
            black_box(128),
            count_ones
        ).best_score().score
    ));
}

fn benchmark_construction_hiff(c: &mut Criterion) {
    c.bench_function(
        "Construct population HIFF", 
        |b| b.iter(|| Population::new_bitstring_population(
            black_box(1000), 
            black_box(128),
            hiff
        ).best_score().score
    ));
}

criterion_group!(benches, benchmark_construction_count_ones, benchmark_construction_hiff);
criterion_main!(benches);


    // #[bench]
    // fn bench_count_ones(b: &mut Bencher) {
    //     let mut rng = StdRng::seed_from_u64(0);
    //     b.iter(|| {
    //         let bits = vec![rng.gen_bool(0.5); 128];
    //         count_ones(&bits)
    //     });
    // }

    // #[bench]
    // fn bench_individual_new(b: &mut Bencher) {
    //     let mut rng = StdRng::seed_from_u64(0);
    //     b.iter(|| Individual::new(128, &mut rng));
    // }

    // #[bench]
    // fn bench_population_new(b: &mut Bencher) {
    //     let mut rng = StdRng::seed_from_u64(0);
    //     b.iter(|| Population::new(100, 128, &mut rng));
    // }

    // #[bench]
    // fn bench_population_iter(b: &mut Bencher) {
    //     let population = Population::new(100, 128, &mut rand::thread_rng());
    //     b.iter(|| {
    //         for ind in population.individuals.iter() {
    //             ind.fitness
    //         }
    //     });
    // }