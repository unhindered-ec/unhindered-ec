use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ec_core::{
    distributions::collection::ConvertToCollectionGenerator, performance::test_results::TestResults,
};
use rand::{
    distributions::{Distribution, Standard},
    thread_rng,
};

// Benchmark the time required to construct instances of `TestResults` from a
// collection of individual error values.
pub fn from_iterator(c: &mut Criterion) {
    const VALUE: i64 = 1_000_000;
    const NUM_VALUES: usize = 1_000;
    let values = [VALUE; NUM_VALUES];
    c.bench_function("TestResults construction from values", |b| {
        b.iter(|| {
            _ = TestResults::<i64>::from(black_box(values));
        });
    });
}

// Benchmark the time required to find the smallest from a set of
// previously constructed `TestResults`.
pub fn find_smallest(c: &mut Criterion) {
    const NUM_VALUES: usize = 100;
    const NUM_RESULTS: usize = 1_000;
    let test_results: Vec<TestResults<i64>> = Distribution::<Vec<i64>>::sample_iter(
        Standard.into_collection_generator(NUM_VALUES),
        &mut thread_rng(),
    )
    .map(Into::into)
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
