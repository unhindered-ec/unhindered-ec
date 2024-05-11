#![cfg(test)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::tuple_array_conversions)]

use proptest::prop_assert_eq;
use push::{
    instruction::{
        instruction_error::PushInstructionError, Instruction, IntInstruction, IntInstructionError,
    },
    push_vm::{push_state::PushState, HasStack},
};
use strum::IntoEnumIterator;
use test_strategy::proptest;

#[test]
fn add() {
    let x = 409;
    let y = 512;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Add.perform(state).unwrap();
    assert_eq!(result.stack::<i64>().size(), 1);
    assert_eq!(*result.stack::<i64>().top().unwrap(), x + y);
}

#[test]
fn add_overflows() {
    let x = 4_098_586_571_925_584_936;
    let y = 5_124_785_464_929_190_872;
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_no_program()
        .build();

    let result = IntInstruction::Add.perform(state).unwrap_err();
    assert_eq!(result.state().stack::<i64>().size(), 2);
    assert_eq!(
        result.error(),
        &PushInstructionError::from(IntInstructionError::Overflow {
            op: IntInstruction::Add
        })
    );
    assert!(result.is_recoverable());
}

#[test]
fn inc_overflows() {
    let x = i64::MAX;
    let state = PushState::builder()
        .with_max_stack_size(1)
        .with_int_values(std::iter::once(x))
        .unwrap()
        .with_no_program()
        .build();

    let result = IntInstruction::Inc.perform(state).unwrap_err();
    assert_eq!(result.state().stack::<i64>().size(), 1);
    assert_eq!(result.state().stack::<i64>().top().unwrap(), &i64::MAX);
    assert_eq!(
        result.error(),
        &IntInstructionError::Overflow {
            op: IntInstruction::Inc
        }
        .into()
    );
    assert!(result.is_recoverable());
}

#[test]
fn dec_overflows() {
    let x = i64::MIN;
    let state = PushState::builder()
        .with_max_stack_size(1)
        .with_int_values(std::iter::once(x))
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Dec.perform(state).unwrap_err();
    assert_eq!(result.state().stack::<i64>().size(), 1);
    assert_eq!(result.state().stack::<i64>().top().unwrap(), &i64::MIN);
    assert_eq!(
        result.error(),
        &IntInstructionError::Overflow {
            op: IntInstruction::Dec
        }
        .into()
    );
    assert!(result.is_recoverable());
}

fn all_instructions() -> Vec<IntInstruction> {
    IntInstruction::iter().collect()
}

#[proptest]
fn negate(#[any] x: i64) {
    let state = PushState::builder()
        .with_max_stack_size(1)
        .with_int_values(std::iter::once(x))
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Negate.perform(state).unwrap();
    prop_assert_eq!(result.stack::<i64>().size(), 1);
    prop_assert_eq!(*result.stack::<i64>().top().unwrap(), -x);
}

#[proptest]
fn abs(#[any] x: i64) {
    let state = PushState::builder()
        .with_max_stack_size(1)
        .with_int_values(std::iter::once(x))
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Abs.perform(state).unwrap();
    prop_assert_eq!(result.stack::<i64>().size(), 1);
    prop_assert_eq!(*result.stack::<i64>().top().unwrap(), x.abs());
}

#[proptest]
fn sqr(#[any] x: i64) {
    let state = PushState::builder()
        .with_max_stack_size(1)
        .with_int_values(std::iter::once(x))
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Square.perform(state);
    if let Some(x_squared) = x.checked_mul(x) {
        let result = result.unwrap();
        prop_assert_eq!(result.stack::<i64>().size(), 1);
        let output = *result.stack::<i64>().top().unwrap();
        prop_assert_eq!(output, x_squared);
    } else {
        let result = result.unwrap_err();
        assert_eq!(
            result.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::Square
            }
            .into()
        );
        assert!(result.is_recoverable());
        let top_int = result.state().stack::<i64>().top().unwrap();
        prop_assert_eq!(*top_int, x);
    }
}

#[proptest]
fn add_does_not_crash(#[any] x: i64, #[any] y: i64) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let _ = IntInstruction::Add.perform(state);
}

#[proptest]
fn add_adds_or_does_nothing(#[any] x: i64, #[any] y: i64) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Add.perform(state);
    #[allow(clippy::unwrap_used)]
    if let Some(expected_result) = x.checked_add(y) {
        let output = result.unwrap().stack_mut::<i64>().pop().unwrap();
        prop_assert_eq!(output, expected_result);
    } else {
        // This only checks that `x` is still on the top of the stack.
        // We arguably want to confirm that the entire state of the system
        // is unchanged, except that the `Add` instruction has been
        // removed from the `exec` stack.
        let result = result.unwrap_err();
        assert_eq!(
            result.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::Add
            }
            .into()
        );
        assert!(result.is_recoverable());
        let top_int = result.state().stack::<i64>().top().unwrap();
        prop_assert_eq!(*top_int, x);
    }
}

#[proptest]
fn subtract_subs_or_does_nothing(#[any] x: i64, #[any] y: i64) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Subtract.perform(state);
    #[allow(clippy::unwrap_used)]
    if let Some(expected_result) = x.checked_sub(y) {
        let output = result.unwrap().stack_mut::<i64>().pop().unwrap();
        prop_assert_eq!(output, expected_result);
    } else {
        // This only checks that `x` is still on the top of the stack.
        // We arguably want to confirm that the entire state of the system
        // is unchanged, except that the `Add` instruction has been
        // removed from the `exec` stack.
        let result = result.unwrap_err();
        assert_eq!(
            result.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::Subtract
            }
            .into()
        );
        assert!(result.is_recoverable());
        let top_int = result.state().stack::<i64>().top().unwrap();
        prop_assert_eq!(*top_int, x);
    }
}

#[proptest]
fn multiply_works_or_does_nothing(#[any] x: i64, #[any] y: i64) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Multiply.perform(state);
    #[allow(clippy::unwrap_used)]
    if let Some(expected_result) = x.checked_mul(y) {
        let output = result.unwrap().stack_mut::<i64>().pop().unwrap();
        prop_assert_eq!(output, expected_result);
    } else {
        // This only checks that `x` is still on the top of the stack.
        // We arguably want to confirm that the entire state of the system
        // is unchanged, except that the `Add` instruction has been
        // removed from the `exec` stack.
        let result = result.unwrap_err();
        assert_eq!(
            result.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::Multiply
            }
            .into()
        );
        assert!(result.is_recoverable());
        let top_int = result.state().stack::<i64>().top().unwrap();
        prop_assert_eq!(*top_int, x);
    }
}

#[proptest]
fn protected_divide_zero_denominator(#[any] x: i64) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, 0])
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::ProtectedDivide.perform(state);
    #[allow(clippy::unwrap_used)]
    let output = result.unwrap().stack_mut::<i64>().pop().unwrap();
    // Dividing by zero should always return 1.
    prop_assert_eq!(output, 1);
}

#[proptest]
fn protected_divide_works_or_does_nothing(#[any] x: i64, #[any] y: i64) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::ProtectedDivide.perform(state);
    #[allow(clippy::unwrap_used)]
    if let Some(expected_result) = x.checked_div(y) {
        let output = result.unwrap().stack_mut::<i64>().pop().unwrap();
        prop_assert_eq!(output, expected_result);
    } else {
        // This only checks that `x` is still on the top of the stack.
        // We arguably want to confirm that the entire state of the system
        // is unchanged, except that the `Add` instruction has been
        // removed from the `exec` stack.
        let result = result.unwrap_err();
        assert_eq!(
            result.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::ProtectedDivide
            }
            .into()
        );
        assert!(result.is_recoverable());
        let top_int = result.state().stack::<i64>().top().unwrap();
        prop_assert_eq!(*top_int, x);
    }
}

#[proptest]
fn mod_zero_denominator(#[any] x: i64) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([0, x])
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Mod.perform(state);
    #[allow(clippy::unwrap_used)]
    let output = result.unwrap().stack_mut::<i64>().pop().unwrap();
    // Modding by zero should always return 0 since x % x = 0 for all x != 0.
    prop_assert_eq!(output, 0);
}

#[proptest]
fn mod_rems_or_does_nothing(#[any] x: i64, #[any] y: i64) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_no_program()
        .build();
    let result = IntInstruction::Mod.perform(state);
    #[allow(clippy::unwrap_used)]
    if let Some(expected_result) = x.checked_rem(y) {
        let output = result.unwrap().stack_mut::<i64>().pop().unwrap();
        prop_assert_eq!(output, expected_result);
    } else if y == 0 {
        let output: i64 = *result.unwrap().stack_mut::<i64>().top().unwrap();
        // Modding by zero should always return 0 since x % x == 0 for all x != 0.
        prop_assert_eq!(output, 0);
    } else {
        // This only checks that `x` is still on the top of the stack.
        // We arguably want to confirm that the entire state of the system
        // is unchanged, except that the `Add` instruction has been
        // removed from the `exec` stack.
        let result = result.unwrap_err();
        assert_eq!(
            result.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::Mod
            }
            .into()
        );
        assert!(result.is_recoverable());
        let top_int = result.state().stack::<i64>().top().unwrap();
        prop_assert_eq!(*top_int, x);
    }
}

#[proptest]
fn inc_does_not_crash(#[any] x: i64) {
    let state = PushState::builder()
        .with_max_stack_size(1)
        .with_int_values(std::iter::once(x))
        .unwrap()
        .with_no_program()
        .build();
    let _ = IntInstruction::Inc.perform(state);
}

#[proptest]
fn int_ops_do_not_crash(
    #[strategy(proptest::sample::select(all_instructions()))] instr: IntInstruction,
    #[any] x: i64,
    #[any] y: i64,
    #[any] b: bool,
) {
    let state = PushState::builder()
        .with_max_stack_size(2)
        .with_int_values([x, y])
        .unwrap()
        .with_bool_values(std::iter::once(b))
        .unwrap()
        .with_no_program()
        .build();
    let _ = instr.perform(state);
}
