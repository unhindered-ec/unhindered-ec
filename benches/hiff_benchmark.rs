use criterion::{criterion_group, criterion_main, Criterion};
use rust_ga::bitstring::hiff;
use rust_ga::individual::Individual;

const NUM_BITS: usize = 128;

fn make_vector(c: &mut Criterion) {
    c.bench_function("XYZ: Construct a vector of `false`", |b| {
        b.iter(|| {
            let bits = [false; NUM_BITS].to_vec();
            assert_eq!(bits.len(), NUM_BITS);
        })
    });
}

fn compute_hiff(c: &mut Criterion) {
    c.bench_function("XYZ: Compute hiff on all false", |b| {
        b.iter(|| {
            let bits = [false; NUM_BITS].to_vec();
            let scores = hiff(&bits);
            assert!(scores.len() >= 2 * NUM_BITS - 1);
        })
    });
}

fn construct_hiff_individual(c: &mut Criterion) {
    c.bench_function("XYZ: Construct a HIFF individual on a random vector", |b| {
        b.iter(|| {
            let ind = Individual::new_bitstring(NUM_BITS, hiff, &mut rand::thread_rng());
            assert!(ind.genome().len() == NUM_BITS);
        })
    });
}

criterion_group!(
    hiff_benches,
    make_vector,
    compute_hiff,
    construct_hiff_individual
);
criterion_main!(hiff_benches);
