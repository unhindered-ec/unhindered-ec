use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_ga::{
    population::Population, 
    bitstring::{count_ones, hiff}, 
    do_main, 
    args::{TargetProblem, Args, RunModel}
};

const DEFAULT_ARGS: Args = Args {
    run_model: RunModel::Parallel,
    target_problem: TargetProblem::Hiff,
    population_size: 1000,
    bit_length: 128,
    num_generations: 100,
};

fn benchmark_construction_count_ones(c: &mut Criterion) {
    c.bench_function(
        "Construct population count_ones", 
        |b| b.iter(|| Population::new_bitstring_population(
            black_box(1000), 
            black_box(128),
            count_ones
        ).best_score().unwrap().total_score
    ));
}

fn benchmark_construction_hiff(c: &mut Criterion) {
    c.bench_function(
        "Construct population HIFF", 
        |b| b.iter(|| Population::new_bitstring_population(
            black_box(1000), 
            black_box(128),
            hiff
        ).best_score().unwrap().total_score
    ));
}

fn benchmark_run_count_ones_serial(c: &mut Criterion) {
    let args = Args {
        run_model: RunModel::Serial,
        target_problem: TargetProblem::CountOnes,
        ..DEFAULT_ARGS
    };
    c.bench_function(
        "Run main() serially on Count Ones", 
        |b| b.iter(|| {
            do_main(black_box(args))
        })
    );
}

fn benchmark_run_count_ones_parallel(c: &mut Criterion) {
    let args = Args {
        run_model: RunModel::Parallel,
        target_problem: TargetProblem::CountOnes,
        ..DEFAULT_ARGS
    };
    c.bench_function(
        "Run main() in parallel on Count Ones", 
        |b| b.iter(|| {
            do_main(black_box(args))
        })
    );
}

fn benchmark_run_hiff_serial(c: &mut Criterion) {
    let args = Args {
        run_model: RunModel::Serial,
        target_problem: TargetProblem::Hiff,
        ..DEFAULT_ARGS
    };
    c.bench_function(
        "Run main() serially on HIFF", 
        |b| b.iter(|| {
            do_main(black_box(args))
        })
    );
}

fn benchmark_run_hiff_parallel(c: &mut Criterion) {
    let args = Args {
        run_model: RunModel::Parallel,
        target_problem: TargetProblem::Hiff,
        ..DEFAULT_ARGS
    };
    c.bench_function(
        "Run main() in parallel on HIFF", 
        |b| b.iter(|| {
            do_main(black_box(args))
        })
    );
}

criterion_group!(construction_benches, benchmark_construction_count_ones, benchmark_construction_hiff);
criterion_group!(main_benches, benchmark_run_count_ones_serial, benchmark_run_count_ones_parallel, benchmark_run_hiff_serial, benchmark_run_hiff_parallel);
criterion_main!(main_benches);
