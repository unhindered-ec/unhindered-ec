use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ordered_float::OrderedFloat;
use push::{
    genome::plushy::{Plushy, PushGene},
    instruction::{FloatInstruction, PushInstruction},
    push_vm::{program::PushProgram, push_state::PushState, HasStack, State},
    vec_into,
};

/// An evolved Plushy genome whose associated Push program evaluates the
/// polynomial (x^3+1)^3 + 1.
#[must_use]
pub fn sample_genome() -> Plushy {
    let genome = vec_into![
        PushInstruction::InputVar("x".into()),
        FloatInstruction::Dup,
        FloatInstruction::ProtectedDivide,
        FloatInstruction::Dup,
        FloatInstruction::Multiply,
        PushGene::Close,
        FloatInstruction::Add,
        FloatInstruction::Add,
        PushInstruction::InputVar("x".into()),
        FloatInstruction::Dup,
        FloatInstruction::Multiply,
        PushInstruction::InputVar("x".into()),
        PushGene::Close,
        FloatInstruction::Multiply,
        FloatInstruction::Push(OrderedFloat(1.0)),
        FloatInstruction::Add,
        FloatInstruction::Dup,
        PushGene::Close,
        FloatInstruction::Dup,
        FloatInstruction::Multiply,
        FloatInstruction::Multiply,
        FloatInstruction::Add,
    ];
    Plushy::new(genome)
}

#[must_use]
pub fn sample_program() -> Vec<PushProgram> {
    sample_genome().into()
}

/// The target polynomial, (x^3+1)^3 + 1.
#[must_use]
pub fn expected(x: OrderedFloat<f64>) -> OrderedFloat<f64> {
    let term = x * x * x + 1.0;
    term * term * term + 1.0
}

/// The input we'll use for the benchmarking. The particular value here probably
/// doesn't actually matter.
const INPUT_VALUE: OrderedFloat<f64> = OrderedFloat(0.25);

/// Set up the initial state for evaluating this Push program.
///
/// # Panics
///
/// Panics if for some reason we can't push our program onto the
/// `exec` stack.
#[must_use]
pub fn build_state(program: Vec<PushProgram>) -> PushState {
    const MAX_STACK_SIZE: usize = 100;
    #[allow(clippy::unwrap_used)]
    PushState::builder()
        .with_max_stack_size(MAX_STACK_SIZE)
        .with_program(program)
        .unwrap()
        .with_float_input("x", INPUT_VALUE)
        .build()
}

/// Run the program and confirm that the result is correct.
///
/// # Panics
///
/// Panics if
/// * It fails to successfully run the sample program
/// * There's no "return" value on the top of the `OrderedFloat<f64>` stack.
pub fn evaluate_regression(c: &mut Criterion) {
    let state = build_state(sample_program());
    let expected_result = expected(INPUT_VALUE);
    c.bench_function("Run symbolic regression function", |b| {
        #[allow(clippy::unwrap_used)]
        b.iter(|| {
            let final_state = &black_box(&state).clone().run_to_completion().unwrap();
            let answer = final_state.stack::<OrderedFloat<f64>>().top().unwrap();
            assert_eq!(answer, &expected_result);
        });
    });
}

criterion_group!(benches, evaluate_regression);
criterion_main!(benches);
