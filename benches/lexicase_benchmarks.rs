use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_ga::{bitstring::{Bitstring, hiff}, population::Population};

fn make_benchmark_populations() -> Vec<(Population<Bitstring>, String)> {
    vec![
        (Population::new_bitstring_population(1000, 128, hiff), "Random".to_string())
    ]
    // todo!()
}

fn bench_lexicase(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lexicase");
    for (population, pop_name) in make_benchmark_populations().iter() {
        group.bench_with_input(BenchmarkId::new("simple_lexicase", pop_name), population, 
            |b, p| b.iter(|| p.simple_lexicase()));
        group.bench_with_input(BenchmarkId::new("lexicase_with_dup_removal", pop_name), population, 
            |b, p| b.iter(|| p.lexicase_with_dup_removal()));
    }
    group.finish();
}

criterion_group!(lexicase_benches, bench_lexicase);
criterion_main!(lexicase_benches);