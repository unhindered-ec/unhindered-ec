#![cfg(test)]

//! Pins the current shape of the typed instruction enums before the stack
//! instructions move into a shared type (issue #530).

use std::fmt::{Debug, Display};

use push::instruction::{BoolInstruction, FloatInstruction, IntInstruction};
use strum::IntoEnumIterator;

// One assert on the first nine items reports every missing or reordered
// stack instruction at once.
fn assert_iter_starts_with<I>(expected: [I; 9])
where
    I: IntoEnumIterator + PartialEq + Debug,
{
    assert_eq!(I::iter().take(9).collect::<Vec<_>>(), expected);
}

fn iter_names<I>() -> Vec<String>
where
    I: IntoEnumIterator + Display,
{
    I::iter().map(|i| i.to_string()).collect()
}

#[test]
fn bool_iter_starts_with_stack_instructions() {
    assert_iter_starts_with([
        BoolInstruction::pop(),
        BoolInstruction::push(false),
        BoolInstruction::dup(),
        BoolInstruction::swap(),
        BoolInstruction::is_empty(),
        BoolInstruction::stack_depth(),
        BoolInstruction::flush(),
        BoolInstruction::print(),
        BoolInstruction::println(),
    ]);
}

#[test]
fn int_iter_starts_with_stack_instructions() {
    assert_iter_starts_with([
        IntInstruction::pop(),
        IntInstruction::push(0),
        IntInstruction::dup(),
        IntInstruction::swap(),
        IntInstruction::is_empty(),
        IntInstruction::stack_depth(),
        IntInstruction::flush(),
        IntInstruction::print(),
        IntInstruction::print_ln(),
    ]);
}

#[test]
fn float_iter_starts_with_stack_instructions() {
    assert_iter_starts_with([
        FloatInstruction::pop(),
        FloatInstruction::push(0.0),
        FloatInstruction::dup(),
        FloatInstruction::swap(),
        FloatInstruction::is_empty(),
        FloatInstruction::stack_depth(),
        FloatInstruction::flush(),
        FloatInstruction::print(),
        FloatInstruction::print_ln(),
    ]);
}

// These lists pin the count, the order, and the `Display` text of `iter()`.
#[test]
fn bool_iter_names() {
    assert_eq!(
        iter_names::<BoolInstruction>(),
        [
            "Pop",
            "Push(false)",
            "Dup",
            "Swap",
            "IsEmpty",
            "StackDepth",
            "Flush",
            "Print",
            "Println",
            "Not",
            "Or",
            "And",
            "Xor",
            "Implies",
            "FromInt",
        ]
    );
}

#[test]
fn int_iter_names() {
    assert_eq!(
        iter_names::<IntInstruction>(),
        [
            "Pop",
            "Push(0)",
            "Dup",
            "Swap",
            "IsEmpty",
            "StackDepth",
            "Flush",
            "Print",
            "PrintLn",
            "Negate",
            "Abs",
            "Min",
            "Max",
            "Clamp",
            "Inc",
            "Dec",
            "Add",
            "Subtract",
            "Multiply",
            "ProtectedDivide",
            "Mod",
            "Power",
            "Square",
            "IsZero",
            "IsPositive",
            "IsNegative",
            "IsEven",
            "IsOdd",
            "Equal",
            "NotEqual",
            "LessThan",
            "LessThanEqual",
            "GreaterThan",
            "GreaterThanEqual",
            "FromBoolean",
            "FromFloatApprox",
        ]
    );
}

#[test]
fn float_iter_names() {
    assert_eq!(
        iter_names::<FloatInstruction>(),
        [
            "Pop",
            "Push(0)",
            "Dup",
            "Swap",
            "IsEmpty",
            "StackDepth",
            "Flush",
            "Print",
            "PrintLn",
            "Add",
            "Subtract",
            "Multiply",
            "ProtectedDivide",
            "Equal",
            "NotEqual",
            "GreaterThan",
            "LessThan",
            "GreaterThanOrEqual",
            "LessThanOrEqual",
            "FromIntApprox",
        ]
    );
}

#[test]
fn push_displays_its_value() {
    assert_eq!(BoolInstruction::push(true).to_string(), "Push(true)");
    assert_eq!(IntInstruction::push(5).to_string(), "Push(5)");
    assert_eq!(FloatInstruction::push(1.5).to_string(), "Push(1.5)");
}
