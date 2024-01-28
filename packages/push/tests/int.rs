#![allow(clippy::unwrap_used)]
#![allow(clippy::tuple_array_conversions)]

use proptest::{arbitrary::any, prop_assert_eq, proptest};
use push::{
    instruction::{
        instruction_error::PushInstructionError, Instruction, IntInstruction, IntInstructionError,
    },
    push_vm::{push_state::PushState, HasStack},
};
use strum::IntoEnumIterator;

#[test]
fn add() {
    let x = 409;
    let y = 512;
    let state = PushState::builder()
        .with_max_stack_size(100)
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
        .with_max_stack_size(100)
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
        .with_max_stack_size(100)
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
        .with_max_stack_size(100)
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

proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(1_000))]

    #[test]
    fn negate(x in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();
        let result = IntInstruction::Negate.perform(state).unwrap();
        prop_assert_eq!(result.stack::<i64>().size(), 1);
        prop_assert_eq!(*result.stack::<i64>().top().unwrap(), -x);
    }

    #[test]
    fn abs(x in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();
        let result = IntInstruction::Abs.perform(state).unwrap();
        prop_assert_eq!(result.stack::<i64>().size(), 1);
        prop_assert_eq!(*result.stack::<i64>().top().unwrap(), x.abs());
    }

    #[test]
    fn sqr(x in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
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
                }.into()
            );
            assert!(result.is_recoverable());
            let top_int = result.state().stack::<i64>().top().unwrap();
            prop_assert_eq!(*top_int, x);
        }
    }

    #[test]
    fn add_does_not_crash(x in any::<i64>(), y in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
            .with_int_values([x,y])
            .unwrap()
            .with_no_program()
            .build();
        let _ = IntInstruction::Add.perform(state);
    }

    #[test]
    fn add_adds_or_does_nothing(x in any::<i64>(), y in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
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

    #[test]
    fn subtract_subs_or_does_nothing(x in any::<i64>(), y in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
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

    #[test]
    fn multiply_works_or_does_nothing(x in any::<i64>(), y in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
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

    #[test]
    fn protected_divide_zero_denominator(x in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
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

    #[test]
    fn protected_divide_works_or_does_nothing(
        x in any::<i64>(),
        y in any::<i64>()
    ) {
        let state = PushState::builder()
            .with_max_stack_size(100)
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

    #[test]
    fn mod_zero_denominator(x in any::<i64>()) {
        let state =PushState::builder()
            .with_max_stack_size(100)
            .with_int_values([0,x])
            .unwrap()
            .with_no_program()
            .build();
        let result = IntInstruction::Mod.perform(state);
        #[allow(clippy::unwrap_used)]
        let output = result.unwrap().stack_mut::<i64>().pop().unwrap();
        // Modding by zero should always return 0 since x % x = 0 for all x != 0.
        prop_assert_eq!(output, 0);
    }

    #[test]
    fn mod_rems_or_does_nothing(x in any::<i64>(), y in any::<i64>()) {
        let state =PushState::builder()
            .with_max_stack_size(100)
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

    #[test]
    fn inc_does_not_crash(x in any::<i64>()) {
        let state = PushState::builder()
            .with_max_stack_size(100)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();
        let _ = IntInstruction::Inc.perform(state);
    }

    #[test]
    fn int_ops_do_not_crash(
            instr in proptest::sample::select(all_instructions()),
            x in any::<i64>(),
            y in any::<i64>(),
            b in proptest::bool::ANY) {
        let state = PushState::builder()
            .with_max_stack_size(100)
            .with_int_values([x, y])
            .unwrap()
            .with_bool_values(std::iter::once(b))
            .unwrap()
            .with_no_program()
            .build();
        let _ = instr.perform(state);
    }
}
