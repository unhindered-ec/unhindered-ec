use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_ga::{
    bitstring::{count_ones, hiff},
    individual::Individual,
    population::VecPop,
};

fn benchmark_construction_count_ones(c: &mut Criterion) {
    c.bench_function("Construct population count_ones", |b| {
        b.iter(|| {
            VecPop::new_bitstring_population(black_box(1000), black_box(128), count_ones)
                .best_individual()
                .test_results()
                .clone()
        })
    });
}

fn benchmark_construction_hiff(c: &mut Criterion) {
    c.bench_function("Construct population HIFF", |b| {
        b.iter(|| {
            VecPop::new_bitstring_population(black_box(1000), black_box(128), hiff)
                .best_individual()
                .test_results()
                .clone()
        })
    });
}

criterion_group!(
    construction_benches,
    benchmark_construction_count_ones,
    benchmark_construction_hiff
);
criterion_main!(construction_benches);
