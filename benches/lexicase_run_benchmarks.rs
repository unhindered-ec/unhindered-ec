// use std::iter;

// use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
// use rust_ga::{
//     bitstring::{Bitstring, count_ones}, 
//     population::Population, 
//     individual::Individual, 
//     args::{Args, RunModel, TargetProblem, LexicaseSelection}, do_main
// };

// const DEFAULT_ARGS: Args = Args {
//     run_model: RunModel::Parallel,
//     target_problem: TargetProblem::CountOnes,
//     population_size: 1000,
//     bit_length: 512,
//     num_generations: 50,
//     lexicase_selection: Some(LexicaseSelection::Simple),
// };

// fn bench_lexicase_runs(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Lexicase Runs");
//     for (problem, problem_label) in [
//         // (TargetProblem::CountOnes, "count-ones"),
//         (TargetProblem::Hiff, "HIFF")
//     ] {
//         for (selector, label) in [
//             // (LexicaseSelection::Simple, "simple"),
//             // (LexicaseSelection::RemoveDuplicates, "remove-dups"),
//             // (LexicaseSelection::OnePass, "one-pass"),
//             (LexicaseSelection::ReuseVector, "reuse-vector")
//         ] {
//             let args = Args {
//                 target_problem: problem,
//                 lexicase_selection: Some(selector),
//                 ..DEFAULT_ARGS
//             };
//             group.bench_with_input(BenchmarkId::new(problem_label, label), &args,
//             |b, args| b.iter(|| {
//                 do_main(*args);
//             }));
//         }
//     }
// }

// criterion_group!(lexicase_run_benches, bench_lexicase_runs);
// criterion_main!(lexicase_run_benches);
