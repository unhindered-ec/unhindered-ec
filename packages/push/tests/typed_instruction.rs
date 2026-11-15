#![cfg(test)]

//! Describes the shared stack instruction type and the generic wrapper
//! type for issue #530.
//!
//! `bool_iter_names` in `stack_instructions.rs` pins the order of
//! `BoolInstruction::iter()`, so this file does not repeat it.

use push::{
    instruction::{BoolInstruction, BoolOp, Perform, TypedInstruction, common::StackInstruction},
    push_vm::{HasStack, push_state::PushState},
};
use strum::IntoEnumIterator;

fn bool_state<const N: usize>(values: [bool; N]) -> PushState {
    PushState::builder()
        .with_max_stack_size(10)
        .with_no_program()
        .with_bool_values(values)
        .unwrap()
        .with_instruction_step_limit(1000)
        .build()
}

#[test]
fn stack_instruction_iter_gives_each_stack_instruction() {
    assert_eq!(
        StackInstruction::<bool>::iter().collect::<Vec<_>>(),
        [
            StackInstruction::pop(),
            StackInstruction::push(false),
            StackInstruction::dup(),
            StackInstruction::swap(),
            StackInstruction::is_empty(),
            StackInstruction::stack_depth(),
            StackInstruction::flush(),
            StackInstruction::print(),
            StackInstruction::print_ln(),
        ]
    );
}

#[test]
fn stack_instruction_dup_copies_top() {
    let result = StackInstruction::<bool>::dup()
        .perform(bool_state([true]))
        .unwrap();
    assert_eq!(*result.stack::<bool>(), [true, true]);
}

#[test]
fn stack_instruction_push_displays_its_value() {
    assert_eq!(StackInstruction::push(true).to_string(), "Push(true)");
}

#[test]
fn bool_instruction_wraps_bool_op() {
    assert_eq!(BoolInstruction::not(), TypedInstruction::Op(BoolOp::Not));
}

// The two tests below check only the wiring through the wrapper. The unit
// tests next to `Dup` and `BoolOp` check the instruction logic.
#[test]
fn bool_instruction_performs_stack_instruction() {
    let result = BoolInstruction::dup().perform(bool_state([true])).unwrap();
    assert_eq!(*result.stack::<bool>(), [true, true]);
}

#[test]
fn bool_instruction_performs_op() {
    let result = BoolInstruction::not().perform(bool_state([true])).unwrap();
    assert_eq!(*result.stack::<bool>(), [false]);
}
