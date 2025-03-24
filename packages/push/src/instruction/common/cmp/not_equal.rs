use std::{any::TypeId, marker::PhantomData};

use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::{Instruction, instruction_error::PushInstructionError},
    push_vm::{HasStack, stack::StackError},
};

/// An instruction that compares two stack values for inequality.
///
/// This instruction works similarly to Rust's [`PartialEq`]. `NotEqual<T>`
/// (which is just a shorthand for `NotEqual<T,T>`) compares the second value on
/// the T stack with the first (top) value from the T
/// stack, pushing the result onto the boolean stack.
///
/// If we instead use two different types, like in `NotEqual<T,U>`, this
/// can compare across stacks as long as `T: PartialEq<U>`, comparing T == U
/// using the top values of the respective stacks. Again the result is pushed
/// onto the boolean stack.
///
/// # Inputs
///
/// ## `NotEqual<T, T>`
///
/// The `NotEqual<T, T>` instruction takes the following inputs:
///    - `T` stack
///      - Two values
///
/// ## `NotEqual<T, U>`
///
/// The `NotEqual<T, U>` instruction takes the following inputs:
///    - `T` stack
///       - One value
///    - `U` stack
///       - One value
///
/// # Behavior
///
/// The `NotEqual` instruction takes top two values from the `T`
/// stack, or one from the `T` stack and one from the `U` stack,
/// compares those values, and pushes the result onto the boolean
/// stack (`true` if those values are _not_ equal, and `false` if they _are_
/// equal).
///
/// ## Action Table
///
/// ### `NotEqual<T, T>`
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
/// | exists | exists | X ≠ Y | ✅ | The value of X≠Y is pushed onto the boolean stack |
/// | missing | irrelevant | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | present | missing | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// ### `NotEqual<T, U>`, where T ≠ U
///
///
/// The table below indicates the behavior in each of the different
/// cases where the two values being compared are being taken from _different_
/// stacks.
///
///    - The "Stack<T>" column indicates "X", the top of the `T` stack.
///    - The "Stack<U>" column indicates "Y", the top of the `U` stack.
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
/// | exists | exists | X ≠ Y | ✅ | The value of X≠Y is pushed onto the boolean stack |
/// | missing | irrelevant | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | present | missing | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// ## `NotEqual<T, T>`
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `T` stack contains fewer than two items.
///
/// ## `NotEqual<T, U>` where T ≠ U
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when either the `T` stack or the `U` stack is empty.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct NotEqual<T, U = T> {
    _p: PhantomData<(T, U)>,
}

impl<S, First, Second> Instruction<S> for NotEqual<First, Second>
where
    S: Clone + HasStack<First> + HasStack<Second> + HasStack<bool>,
    First: PartialEq<Second> + 'static,
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
            && ![TypeId::of::<First>(), TypeId::of::<Second>()].contains(&TypeId::of::<bool>())
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
        let first_stack_required = if TypeId::of::<First>() == TypeId::of::<Second>() {
            2
        } else {
            1
        };

        {
            // Create a scope so we can't accidentally use `first_stack_size` after
            // we might have modified the `First` stack.
            let first_stack_size = state.stack::<First>().size();
            // If the `First` stack doesn't have enough items, we want to return
            // a recoverable error right away before we modify any stacks.
            if first_stack_size < first_stack_required {
                return Err(Error::recoverable(
                    state,
                    StackError::Underflow {
                        num_requested: first_stack_required,
                        num_present: first_stack_size,
                    },
                ));
            }
        }

        let second = match state.stack_mut::<Second>().pop() {
            Ok(value) => value,
            Err(error) => return Err(Error::recoverable(state, error)),
        };
        let first = match state.stack_mut::<First>().pop() {
            Ok(value) => value,
            // Because of the size checks above, this should actually never fail, so we'll return a
            // `Fatal` error if for some reason it does.
            Err(error) => return Err(Error::fatal(state, error)),
        };

        state.with_push(first != second).map_err_into()
    }
}

#[cfg(test)]
mod test {
    use ordered_float::OrderedFloat;
    use proptest::prelude::*;
    use test_strategy::proptest;

    use super::NotEqual;
    use crate::{
        instruction::{Instruction, instruction_error::PushInstructionError},
        push_vm::{HasStack, push_state::PushState, stack::StackError},
    };

    #[test]
    fn not_equal_same_stack_equal() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([5, 5])
            .unwrap()
            .with_no_program()
            .build();
        let result = NotEqual::<i64>::default().perform(state).unwrap();
        assert!(result.stack::<i64>().is_empty());
        assert_eq!(result.stack::<bool>(), &[false]);
    }

    #[test]
    fn not_equal_same_stack_not_equal() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([5, 10])
            .unwrap()
            .with_no_program()
            .build();
        let result = NotEqual::<i64>::default().perform(state).unwrap();
        assert!(result.stack::<i64>().is_empty());
        assert_eq!(result.stack::<bool>(), &[true]);
    }

    // #[test]
    // fn not_equal_different_stacks_equal() {
    //     let state = PushState::builder()
    //         .with_max_stack_size(1)
    //         .with_int_values([5])
    //         .unwrap()
    //         .with_float_values([OrderedFloat(5.0)])
    //         .unwrap()
    //         .with_no_program()
    //         .build();
    //     let result = NotEqual::<i64, OrderedFloat<f64>>::default()
    //         .perform(state)
    //         .unwrap();
    //     assert_eq!(result.stack::<bool>().top().unwrap(), &false);
    //     assert_eq!(result.stack::<bool>().size(), 1);
    // }

    // #[test]
    // fn not_equal_different_stacks_not_equal() {
    //     let state = PushState::builder()
    //         .with_max_stack_size(1)
    //         .with_int_values([5])
    //         .unwrap()
    //         .with_float_values([OrderedFloat(10.0)])
    //         .unwrap()
    //         .with_no_program()
    //         .build();
    //     let result = NotEqual::<i64, OrderedFloat<f64>>::default()
    //         .perform(state)
    //         .unwrap();
    //     assert_eq!(result.stack::<bool>().top().unwrap(), &true);
    //     assert_eq!(result.stack::<bool>().size(), 1);
    // }

    #[test]
    fn not_equal_same_stack_underflow() {
        let state = PushState::builder()
            // This has to be 1 to ensure that there's room on the bool stack for the result.
            .with_max_stack_size(1)
            .with_no_program()
            .build();
        let result = NotEqual::<i64>::default().perform(state).unwrap_err();
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
    fn not_equal_same_stack_underflow_one() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values([5])
            .unwrap()
            .with_no_program()
            .build();
        let result = NotEqual::<i64>::default().perform(state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 2,
                num_present: 1
            })
        );
    }

    // #[test]
    // fn not_equal_different_stacks_underflow_first() {
    //     let state = PushState::builder()
    //         .with_max_stack_size(0)
    //         .with_float_values([OrderedFloat(10.0)])
    //         .unwrap()
    //         .with_no_program()
    //         .build();
    //     let result = NotEqual::<i64, OrderedFloat<f64>>::default()
    //         .perform(state)
    //         .unwrap_err();
    //     assert!(result.is_recoverable());
    //     assert_eq!(
    //         result.error(),
    //         &PushInstructionError::StackError(StackError::Underflow {
    //             num_requested: 1,
    //             num_present: 0
    //         })
    //     );
    // }

    // #[test]
    // fn not_equal_different_stacks_underflow_second() {
    //     let state = PushState::builder()
    //         .with_max_stack_size(1)
    //         .with_int_values([5])
    //         .unwrap()
    //         .with_max_stack_size(0)
    //         .with_no_program()
    //         .build();
    //     let result = NotEqual::<i64, OrderedFloat<f64>>::default()
    //         .perform(state)
    //         .unwrap_err();
    //     assert!(result.is_recoverable());
    //     assert_eq!(
    //         result.error(),
    //         &PushInstructionError::StackError(StackError::Underflow {
    //             num_requested: 1,
    //             num_present: 0
    //         })
    //     );
    // }

    #[test]
    fn not_equal_bool_stack_overflow() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([5, 5])
            .unwrap()
            // Trying to push the result will overflow the boolean stack
            .with_bool_values([false, false])
            .unwrap()
            .with_no_program()
            .build();
        let result = NotEqual::<i64>::default().perform(state).unwrap_err();
        assert!(!result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Overflow { stack_type: "bool" })
        );
    }

    #[proptest]
    fn not_equal_proptest_same_stack(a: i64, b: i64) {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([a, b])
            .unwrap()
            .with_no_program()
            .build();
        let result = NotEqual::<i64>::default().perform(state).unwrap();
        prop_assert_eq!(result.stack::<bool>().top().unwrap(), &(a != b));
        prop_assert_eq!(result.stack::<bool>().size(), 1);
    }

    // #[proptest]
    // fn not_equal_proptest_different_stacks(a: i64, b: f64) {
    //     let state = PushState::builder()
    //         .with_max_stack_size(1)
    //         .with_int_values([a])
    //         .unwrap()
    //         .with_float_values([OrderedFloat(b)])
    //         .unwrap()
    //         .with_no_program()
    //         .build();
    //     let result = NotEqual::<i64, OrderedFloat<f64>>::default()
    //         .perform(state)
    //         .unwrap();
    //     prop_assert_eq!(result.stack::<bool>().top().unwrap(), &(a as f64 !=
    // b));     prop_assert_eq!(result.stack::<bool>().size(), 1);
    // }
}
