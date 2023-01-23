use criterion::{criterion_group, criterion_main, Criterion};
use rust_ga::{child_maker::{two_point_xo_mutate::{self, TwoPointXoMutate}, ChildMaker}, bitstring::{count_ones, new_bitstring_population}, selector::best::Best};



fn child_maker(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let two_point_xo_mutate = TwoPointXoMutate { 
        scorer: &count_ones
    };

    c.bench_function("Two-point Xo + mutate", |b| {
        b.iter(|| {
            let population = new_bitstring_population(1, 128, count_ones);
            let child = two_point_xo_mutate.make_child(&mut rng, &population, &Best);
            // let bits = [false; NUM_BITS].to_vec();
            // assert_eq!(bits.len(), NUM_BITS);
        })
    });
}

criterion_group!(child_maker_benches, child_maker);
criterion_main!(child_maker_benches);
