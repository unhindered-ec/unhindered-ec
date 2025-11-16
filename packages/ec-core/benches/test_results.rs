use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use ec_core::{distributions::collection::ConvertToCollectionGenerator, test_results::TestResults};
use rand::{
    distr::{Distribution, StandardUniform},
    rng,
};

// Benchmark the time required to construct instances of `TestResults` from a
// collection of individual error values.
pub fn from_iterator(c: &mut Criterion) {
    const VALUE: i64 = 1_000_000;
    const NUM_VALUES: usize = 1_000;
    let values = [VALUE; NUM_VALUES];
    c.bench_function("TestResults construction from values", |b| {
        b.iter(|| {
            _ = TestResults::<i64>::from_iter(black_box(values));
        });
    });
}

// Benchmark the time required to find the smallest from a set of
// previously constructed `TestResults`.
pub fn find_smallest(c: &mut Criterion) {
    const NUM_VALUES: usize = 100;
    const NUM_RESULTS: usize = 1_000;
    // We need the random values to be smaller (`i32`) than the type used
    // in `TestResults` (`i64`) so that the sum of the values in a vector
    // won't overflow the largest possible value in the `TestResults` type.
    let test_results: Vec<TestResults<i64>> = Distribution::<Vec<i32>>::sample_iter(
        StandardUniform.into_collection_generator(NUM_VALUES),
        &mut rng(),
    )
    .map(TestResults::from_iter)
    .take(NUM_RESULTS)
    .collect::<Vec<_>>();
    c.bench_function("Find smallest TestResults", |b| {
        b.iter(|| {
            _ = black_box(&test_results).iter().min();
        });
    });
}

criterion_group!(benches, from_iterator, find_smallest);
criterion_main!(benches);
