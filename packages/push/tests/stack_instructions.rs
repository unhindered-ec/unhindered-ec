#![cfg(test)]

//! Checks that `iter()` on each typed instruction enum gives the stack
//! instructions and the type-specific instructions (issue #530).
//!
//! The enums are `#[non_exhaustive]`, so these tests check only that
//! `iter()` contains at least the listed instructions. They do not check
//! the order, the count, or the `Display` text.

use std::fmt::Debug;

use push::instruction::{BoolInstruction, FloatInstruction, IntInstruction};
use strum::IntoEnumIterator;

fn assert_iter_contains<I>(expected: impl IntoIterator<Item = I>)
where
    I: IntoEnumIterator + PartialEq + Debug,
{
    let all: Vec<I> = I::iter().collect();
    let missing: Vec<I> = expected.into_iter().filter(|i| !all.contains(i)).collect();
    assert_eq!(missing, []);
}

#[test]
fn bool_iter_contains_stack_and_bool_instructions() {
    assert_iter_contains([
        BoolInstruction::pop(),
        BoolInstruction::push(false),
        BoolInstruction::dup(),
        BoolInstruction::swap(),
        BoolInstruction::is_empty(),
        BoolInstruction::stack_depth(),
        BoolInstruction::flush(),
        BoolInstruction::print(),
        BoolInstruction::println(),
        BoolInstruction::not(),
    ]);
}

#[test]
fn int_iter_contains_stack_and_int_instructions() {
    assert_iter_contains([
        IntInstruction::pop(),
        IntInstruction::push(0),
        IntInstruction::dup(),
        IntInstruction::swap(),
        IntInstruction::is_empty(),
        IntInstruction::stack_depth(),
        IntInstruction::flush(),
        IntInstruction::print(),
        IntInstruction::print_ln(),
        IntInstruction::add(),
    ]);
}

#[test]
fn float_iter_contains_stack_and_float_instructions() {
    assert_iter_contains([
        FloatInstruction::pop(),
        FloatInstruction::push(0.0),
        FloatInstruction::dup(),
        FloatInstruction::swap(),
        FloatInstruction::is_empty(),
        FloatInstruction::stack_depth(),
        FloatInstruction::flush(),
        FloatInstruction::print(),
        FloatInstruction::print_ln(),
        FloatInstruction::add(),
    ]);
}
