use std::{any::TypeId, marker::PhantomData};

use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::{Instruction, instruction_error::PushInstructionError},
    push_vm::{HasStack, stack::StackError},
};

/// An instruction that check if one value is greater than another.
///
/// This instruction works similarly to Rust's [`PartialOrd`].
///
/// `GreaterThan<T>` (which
/// is just a shorthand for `GreaterThan<T,T>`) compares the top
/// two values of `Stack<T>`, pushing the result onto the boolean
/// stack. The comparison will be `x > y`, where `x` is the top value
/// on the stack, and `y` is the second value on the stack.
///
/// If we instead use two different types, like in `GreaterThan<T,U>`, this
/// can compare across stacks as long as `T: PartialOrd<U>`, comparing T > U
/// using the top values of the respective stacks. Again the result is pushed
/// onto the boolean stack.
///
/// # Inputs
///
/// ## `GreaterThan<T, T>`
///
/// The `GreaterThan<T, T>` instruction takes the following inputs:
///    - `T` stack
///      - Two values
///
/// ## `GreaterThan<T, U>`
///
/// The `GreaterThan<T, U>` instruction takes the following inputs:
///    - `T` stack
///       - One value
///    - `U` stack
///       - One value
///
/// # Behavior
///
/// The `GreaterThan` instruction takes top two values from the `T`
/// stack (`x` from the top and `y` below it), or one from the `T` stack (`x`)
/// and one from the `U` stack (`y`),
/// compares those values, and pushes the result onto the boolean
/// stack (`true` if `x > y`, and `false` otherwise).
///
/// ## Action Table
///
/// ### `GreaterThan<T, T>`
///
/// The table below indicates the behavior in each of the different
/// cases where the two values being compared are being taken from the same
/// stack.
///
///    - The "X" column indicates the top of the `T` stack.
///    - The "Y" column indicates the second value on the `T` stack.
///    - The "Result" column indicates the value left on the top of the boolean
///      stack.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | X  | Y | Result | Success | Note |
/// | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- |
/// | exists | exists | X = Y | ✅ | The value of X>Y is pushed onto the boolean stack |
/// | missing | irrelevant | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | present | missing | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// ### `GreaterThan<T, U>`, where T ≠ U
///
///
/// The table below indicates the behavior in each of the different
/// cases where the two values being compared are being taken from _different_
/// stacks.
///
///    - The `Stack<T>` column indicates "X", the top of the `T` stack.
///    - The `Stack<U>` column indicates "Y", the top of the `U` stack.
///    - The "Result" column indicates the value left on the top of the boolean
///      stack.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | "X": `Stack<T>`  | "Y": `Stack<U>` | Result | Success | Note |
/// | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- |
/// | exists | exists | X = Y | ✅ | The value of X>Y is pushed onto the boolean stack |
/// | missing | irrelevant | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | present | missing | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// ## `GreaterThan<T, T>`
///
/// Returns a
/// [`StackError::Underflow`]
/// error when the `T` stack contains fewer than two items.
///
/// ## `GreaterThan<T, U>` where T ≠ U
///
/// Returns a
/// [`StackError::Underflow`]
/// error when either the `T` stack or the `U` stack is empty.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct GreaterThan<T, U = T> {
    _p: PhantomData<(T, U)>,
}

impl<S, First, Second> Instruction<S> for GreaterThan<First, Second>
where
    S: Clone + HasStack<Second> + HasStack<First> + HasStack<bool>,
    First: PartialOrd<Second> + 'static,
    Second: 'static,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        // If the bool stack is full, then this would overflow that
        // stack and we return a fatal error. If either of the stacks being
        // compared, however, is the boolean stack, then that will free up at
        // least the one space needed for the result.
        let bool_stack = state.stack::<bool>();
        if bool_stack.is_full()
            && ![TypeId::of::<Second>(), TypeId::of::<First>()].contains(&TypeId::of::<bool>())
        {
            return Err(Error::fatal(
                state,
                StackError::Overflow { stack_type: "bool" },
            ));
        }

        // If First=Second (i.e., we're comparing items from the same stack), then we
        // will need to pop two values from that stack. If they are different we
        // will need to pop one value from First. So we'll just check the size
        // of First here, which will include the size of Second if First=Second.
        let second_stack_required = if TypeId::of::<Second>() == TypeId::of::<First>() {
            2
        } else {
            1
        };

        {
            // Create a scope so we can't accidentally use `first_stack_size` after
            // we might have modified the `First` stack.
            let second_stack_size = state.stack::<Second>().size();
            // If the `First` stack doesn't have enough items, we want to return
            // a recoverable error right away before we modify any stacks.
            if second_stack_size < second_stack_required {
                return Err(Error::recoverable(
                    state,
                    StackError::Underflow {
                        num_requested: second_stack_required,
                        num_present: second_stack_size,
                    },
                ));
            }
        }

        let first = match state.stack_mut::<First>().pop() {
            Ok(value) => value,
            Err(error) => return Err(Error::recoverable(state, error)),
        };
        let second = match state.stack_mut::<Second>().pop() {
            Ok(value) => value,
            // Because of the size checks above, this should actually never fail, so we'll return a
            // `Fatal` error if for some reason it does.
            Err(error) => return Err(Error::fatal(state, error)),
        };

        state.with_push(first > second).map_err_into()
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use test_strategy::proptest;

    use super::GreaterThan;
    use crate::{
        instruction::{Instruction, instruction_error::PushInstructionError},
        push_vm::{HasStack, push_state::PushState, stack::StackError},
    };

    #[test]
    fn equal() {
        let state = PushState::builder()
            .with_instruction_step_limit(1)
            .with_max_stack_size(2)
            .with_int_values([5, 5])
            .unwrap()
            .with_no_program()
            .build();
        let result = GreaterThan::<i64>::default().perform(state).unwrap();
        assert!(result.stack::<i64>().is_empty());
        assert_eq!(result.stack::<bool>(), &[false]);
    }

    #[test]
    fn less_than() {
        let state = PushState::builder()
            .with_instruction_step_limit(1)
            .with_max_stack_size(2)
            .with_int_values([5, 10])
            .unwrap()
            .with_no_program()
            .build();
        let result = GreaterThan::<i64>::default().perform(state).unwrap();
        assert!(result.stack::<i64>().is_empty());
        assert_eq!(result.stack::<bool>(), &[false]);
    }

    #[test]
    fn greater_than() {
        let state = PushState::builder()
            .with_instruction_step_limit(1)
            .with_max_stack_size(2)
            .with_int_values([10, 5])
            .unwrap()
            .with_no_program()
            .build();
        let result = GreaterThan::<i64>::default().perform(state).unwrap();
        assert!(result.stack::<i64>().is_empty());
        assert_eq!(result.stack::<bool>(), &[true]);
    }

    #[test]
    fn empty_stack() {
        let state = PushState::builder()
            .with_instruction_step_limit(1)
            // This has to be 1 to ensure that there's room on the bool stack for the result.
            .with_max_stack_size(1)
            .with_no_program()
            .build();
        let result = GreaterThan::<i64>::default().perform(state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 2,
                num_present: 0
            })
        );
    }

    #[test]
    fn singleton_stack() {
        let state = PushState::builder()
            .with_instruction_step_limit(1)
            .with_max_stack_size(1)
            .with_int_values([5])
            .unwrap()
            .with_no_program()
            .build();
        let result = GreaterThan::<i64>::default().perform(state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 2,
                num_present: 1
            })
        );
    }

    #[test]
    fn bool_stack_overflow() {
        let state = PushState::builder()
            .with_instruction_step_limit(1)
            .with_max_stack_size(2)
            .with_int_values([5, 5])
            .unwrap()
            // Trying to push the result will overflow the boolean stack
            .with_bool_values([false, false])
            .unwrap()
            .with_no_program()
            .build();
        let result = GreaterThan::<i64>::default().perform(state).unwrap_err();
        assert!(!result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Overflow { stack_type: "bool" })
        );
    }

    #[proptest]
    fn proptest_same_stack(a: i64, b: i64) {
        let state = PushState::builder()
            .with_instruction_step_limit(1)
            .with_max_stack_size(2)
            .with_int_values([a, b])
            .unwrap()
            .with_no_program()
            .build();
        let result = GreaterThan::<i64>::default().perform(state).unwrap();
        prop_assert_eq!(result.stack::<bool>().top().unwrap(), &(a > b));
        prop_assert_eq!(result.stack::<bool>().size(), 1);
    }
}
