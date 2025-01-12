#![cfg(test)]
#![expect(
    clippy::arithmetic_side_effects,
    reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing side \
              for test code."
)]

use ordered_float::OrderedFloat;
use proptest::prop_assert_eq;
use push::{
    instruction::{FloatInstruction, Instruction, PushInstruction},
    push_vm::{HasStack, push_state::PushState, stack::StackError},
};
use test_strategy::proptest;

#[test]
fn to_push_instruction() {
    let float_instruction = FloatInstruction::Add;
    let push_instruction: PushInstruction = float_instruction.into();
    matches!(push_instruction, PushInstruction::FloatInstruction(fi) if fi == float_instruction);
}

#[test]
fn push_float() {
    let x = OrderedFloat(589.632);
    let state = PushState::builder()
        .with_max_stack_size(1)
        .with_no_program()
        .build();
    let result = FloatInstruction::Push(x).perform(state).unwrap();
    assert_eq!(result.stack::<OrderedFloat<f64>>().size(), 1);
    assert_eq!(*result.stack::<OrderedFloat<f64>>().top().unwrap(), x);
}

#[test]
fn add() {
    let x = OrderedFloat(409.37);
    let y = OrderedFloat(512.825);
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::Add.perform(state).unwrap();
    assert_eq!(result.stack::<OrderedFloat<f64>>().size(), 1);
    assert_eq!(*result.stack::<OrderedFloat<f64>>().top().unwrap(), x + y);
}

#[test]
fn overflow_bool_stack() {
    let x = OrderedFloat(409.37);
    let state = PushState::builder()
        // Set the max stack size to 2 so we can cause it to overflow.
        .with_max_stack_size(2)
        // Push two copies of x so we can call `FloatInstruction::Equal`
        .with_float_values([x, x])
        .unwrap()
        // Now fill the boolean stack so that it overflows when we check the floats for equality
        .with_bool_values([false, false])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::Equal.perform(state).unwrap_err();
    assert_eq!(
        result.error(),
        &StackError::Overflow { stack_type: "bool" }.into()
    );
}

#[test]
fn dup() {
    let x = OrderedFloat(409.37);
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values(std::iter::once(x))
        .unwrap()
        .with_no_program()
        .build();
    let mut result = FloatInstruction::Dup.perform(state).unwrap();
    assert_eq!(result.stack::<OrderedFloat<f64>>().size(), 2);
    let float_stack = result.stack_mut::<OrderedFloat<f64>>();
    let (&a, &b) = float_stack.top2().unwrap();
    assert_eq!(a, x);
    assert_eq!(b, x);
}

#[proptest]
fn add_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x + y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::Add.perform(state).unwrap();
    let output = result.stack::<OrderedFloat<f64>>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn subtract_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x - y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::Subtract.perform(state).unwrap();
    let output = result.stack::<OrderedFloat<f64>>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn multiply_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x * y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::Multiply.perform(state).unwrap();
    let output = result.stack::<OrderedFloat<f64>>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn protected_divide_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = if y == 0.0 { OrderedFloat(1.0) } else { x / y };
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::ProtectedDivide.perform(state).unwrap();
    let output = result.stack::<OrderedFloat<f64>>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn equal_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x == y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::Equal.perform(state).unwrap();
    let output = result.stack::<bool>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn not_equal_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x != y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::NotEqual.perform(state).unwrap();
    let output = result.stack::<bool>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn greater_than_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x > y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::GreaterThan.perform(state).unwrap();
    let output = result.stack::<bool>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn less_than_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x < y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::LessThan.perform(state).unwrap();
    let output = result.stack::<bool>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn greater_than_or_equal_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x >= y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::GreaterThanOrEqual.perform(state).unwrap();
    let output = result.stack::<bool>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}

#[proptest]
fn less_than_or_equal_prop(#[any] x: OrderedFloat<f64>, #[any] y: OrderedFloat<f64>) {
    let expected_result = x <= y;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_float_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = FloatInstruction::LessThanOrEqual.perform(state).unwrap();
    let output = result.stack::<bool>().top().unwrap();
    prop_assert_eq!(*output, expected_result);
}
