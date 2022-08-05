use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_ga::population::Population;
use rust_ga::bitstring::make_bitstring;
use rust_ga::bitstring::count_ones;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "Construct population", 
        |b| b.iter(|| Population::new(
            black_box(1000), 
            |rng| make_bitstring(black_box(128), rng),
            count_ones)));
}

criterion_group!(benches, criterion_benchmark);
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