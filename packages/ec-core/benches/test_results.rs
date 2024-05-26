use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ec_core::test_results::TestResults;

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

criterion_group!(benches, from_iterator);
criterion_main!(benches);
