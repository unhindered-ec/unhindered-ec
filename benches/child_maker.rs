use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rust_ga::{
    bitstring::new_bitstring_population,
    child_maker::{two_point_xo_mutate::TwoPointXoMutate, ChildMaker},
    individual::Individual,
    selector::best::Best,
    test_results::TestResults,
};

fn child_maker(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let trivial_scorer = |_: &[bool]| vec![0i64];

    let trivial_tester = |_: &Vec<bool>| TestResults {
        total_result: 0i64,
        results: vec![0i64],
    };

    let two_point_xo_mutate = TwoPointXoMutate {
        scorer: &trivial_scorer,
    };

    let population = new_bitstring_population(
        1,
        16,
        // TODO: I should really have a function somewhere that converts functions
        //   that return vectors of scores to `TestResults` structs.
        trivial_tester,
    );

    c.bench_function("Two-point Xo + mutate", |b| {
        b.iter(|| {
            let child 
                = two_point_xo_mutate.make_child(black_box(&mut rng), black_box(&population), black_box(&Best));
            assert_eq!(0, child.test_results().total_result);
        })
    });
}

criterion_group!(child_maker_benches, child_maker);
criterion_main!(child_maker_benches);
