use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_ga::{selector::{tournament::Tournament, Selector}, population::VecPop, bitstring::Bitstring};

fn tournaments(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let population = VecPop::new_bitstring_population(1000, 128, |_: &Bitstring| 0);

    let mut group = c.benchmark_group("Tournament selection");
    for tournament_size in [2, 10, 100] {
        let tournament_selector = Tournament::new(tournament_size);
        group.bench_with_input(BenchmarkId::new("tournament size", tournament_size), &tournament_selector,
            |b, t| b.iter(|| t.select(&mut rng, &population)));
    }
    group.finish();
}

criterion_group!(tournament_benches, tournaments);
criterion_main!(tournament_benches);
