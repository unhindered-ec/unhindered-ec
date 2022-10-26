use std::iter;

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_ga::{bitstring::{Bitstring, hiff}, population::Population, individual::Individual};

fn make_uniform_pop() -> Population<Bitstring> {
    let ind = Individual::new_bitstring(128, hiff, &mut rand::thread_rng());
    let individuals = iter::repeat(ind).take(1000).collect();
    Population { individuals }
}

fn make_num_distinct_pop(num_distinct_groups: usize) -> Population<Bitstring> {
    let group_size = 1000 / num_distinct_groups;
    let mut individuals = Vec::new();
    for _ in 0..num_distinct_groups {
        let ind = Individual::new_bitstring(128, hiff, &mut rand::thread_rng());
        for _ in 0..group_size {
            individuals.push(ind.clone());
        }
    }
    Population { individuals }
}

fn make_benchmark_populations() -> Vec<(Population<Bitstring>, String)> {
    vec![
        (Population::new_bitstring_population(1000, 128, hiff), "Random".to_string()),
        (make_uniform_pop(), "Uniform".to_string())
    ]
}

fn bench_lexicase(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lexicase");
    for (population, pop_name) in make_benchmark_populations().iter() {
        group.bench_with_input(BenchmarkId::new("simple_lexicase", pop_name), population, 
            |b, p| b.iter(|| p.simple_lexicase()));
        group.bench_with_input(BenchmarkId::new("lexicase_with_dup_removal", pop_name), population, 
            |b, p| b.iter(|| p.lexicase_with_dup_removal()));
    }
    for num_groups in [10, 20, 25, 40, 50, 100, 200, 500, 1000] {
        let population = make_num_distinct_pop(num_groups);
        group.bench_with_input(BenchmarkId::new("simple_lexicase", num_groups), &population, 
            |b, p| b.iter(|| p.simple_lexicase()));
        group.bench_with_input(BenchmarkId::new("lexicase_with_dup_removal", num_groups), &population, 
            |b, p| b.iter(|| p.lexicase_with_dup_removal()));
    }
    group.finish();
}

criterion_group!(lexicase_benches, bench_lexicase);
criterion_main!(lexicase_benches);
