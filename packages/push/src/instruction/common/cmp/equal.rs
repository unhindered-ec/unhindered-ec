use std::{any::TypeId, marker::PhantomData};

use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::{Instruction, instruction_error::PushInstructionError},
    push_vm::{HasStack, stack::StackError},
};

/// An instruction that compares two stack values for equality.
///
/// This instruction works similarly to Rust's [`PartialEq`]. `Equal<T>` (which
/// is just a shorthand for `Equal<T,T>`) compares the second value on the T
/// stack with the first (top) value from the T
/// stack, pushing the result onto the boolean stack.
///
/// If we instead use two different types, like in `Equal<T,U>`, this
/// can compare across stacks as long as `T: PartialEq<U>`, comparing T == U
/// using the top values of the respective stacks. Again the result is pushed
/// onto the boolean stack.
///
/// # Inputs
///
/// ## `Equal<T, T>`
///
/// The `Equal<T, T>` instruction takes the following inputs:
///    - `T` stack
///      - Two values
///
/// ## `Equal<T, U>`
///
/// The `Equal<T, U>` instruction takes the following inputs:
///    - `T` stack
///       - One value
///    - `U` stack
///       - One value
///
/// # Behavior
///
/// The `Equal` instruction takes top two values from the `T`
/// value of the `i64` stack. The one exception (as described above) is when
/// the value is `i64::MIN`, where `Abs` removes it and pushes
/// on `i64::MAX` in its place.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`i64` stack" column indicates the value of the top of the `i64`
///      stack, or whether it exists.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `i64` stack  |  Success | Note |
/// | ------------- | ------------- | ------------- |
/// | `i64::MIN`    | ✅ | `i64::MIN` is replaced with `i64::MAX` |
/// | exists, not `i64::MIN` | ✅ | Takes the absolute value of the top value of the `i64` stack |
/// | missing | [❗..](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// Returns a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error when the `i64` stack is empty.
///
/// # Differences
/// Implementations of integer negation instructions in Clojure (e.g., Clojush
/// or Propeller) or Python (e.g. `PyshGP`) won't have the wrapping issue
/// because they act on arbitrary precision integers.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Equal<T, U = T> {
    _p: PhantomData<(T, U)>,
}

impl<S, First, Second> Instruction<S> for Equal<First, Second>
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

        state.with_push(first == second).map_err_into()
    }
}

#[cfg(test)]
mod test {
    use ordered_float::OrderedFloat;
    use proptest::prelude::*;
    use test_strategy::proptest;

    use super::Equal;
    use crate::{
        instruction::{Instruction, instruction_error::PushInstructionError},
        push_vm::{HasStack, push_state::PushState, stack::StackError},
    };

    #[test]
    fn equal_same_stack_equal() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([5, 5])
            .unwrap()
            .with_no_program()
            .build();
        let result = Equal::<i64>::default().perform(state).unwrap();
        assert!(result.stack::<i64>().is_empty());
        assert_eq!(result.stack::<bool>(), &[true]);
    }

    #[test]
    fn equal_same_stack_not_equal() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([5, 10])
            .unwrap()
            .with_no_program()
            .build();
        let result = Equal::<i64>::default().perform(state).unwrap();
        assert!(result.stack::<i64>().is_empty());
        assert_eq!(result.stack::<bool>(), &[false]);
    }

    // #[test]
    // fn equal_different_stacks_equal() {
    //     let state = PushState::builder()
    //         .with_max_stack_size(1)
    //         .with_int_values([5])
    //         .unwrap()
    //         .with_float_values([OrderedFloat(5.0)])
    //         .unwrap()
    //         .with_no_program()
    //         .build();
    //     let result = Equal::<i64, f64>::default().perform(state).unwrap();
    //     assert_eq!(result.stack::<bool>().top().unwrap(), &true);
    //     assert_eq!(result.stack::<bool>().size(), 1);
    // }

    // #[test]
    // fn equal_different_stacks_not_equal() {
    //     let state = PushState::builder()
    //         .with_max_stack_size(1)
    //         .with_int_values([5])
    //         .unwrap()
    //         .with_float_values([OrderedFloat(10.0)])
    //         .unwrap()
    //         .with_no_program()
    //         .build();
    //     let result = Equal::<i64, f64>::default().perform(state).unwrap();
    //     assert_eq!(result.stack::<bool>().top().unwrap(), &false);
    //     assert_eq!(result.stack::<bool>().size(), 1);
    // }

    #[test]
    fn equal_same_stack_underflow() {
        let state = PushState::builder()
            // This has to be 1 to ensure that there's room on the bool stack for the result.
            .with_max_stack_size(1)
            .with_no_program()
            .build();
        let result = Equal::<i64>::default().perform(state).unwrap_err();
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
    fn equal_same_stack_underflow_one() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values([5])
            .unwrap()
            .with_no_program()
            .build();
        let result = Equal::<i64>::default().perform(state).unwrap_err();
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
    // fn equal_different_stacks_underflow_first() {
    //     let state = PushState::builder()
    //         .with_max_stack_size(0)
    //         .with_float_values([OrderedFloat(10.0)])
    //         .unwrap()
    //         .with_no_program()
    //         .build();
    //     let result = Equal::<i64, f64>::default().perform(state).unwrap_err();
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
    // fn equal_different_stacks_underflow_second() {
    //     let state = PushState::builder()
    //         .with_max_stack_size(1)
    //         .with_int_values([5])
    //         .unwrap()
    //         .with_max_stack_size(0)
    //         .with_no_program()
    //         .build();
    //     let result = Equal::<i64, f64>::default().perform(state).unwrap_err();
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
    fn equal_bool_stack_overflow() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([5, 5])
            .unwrap()
            // Trying to push the result will overflow the boolean stack
            .with_bool_values([false, false])
            .unwrap()
            .with_no_program()
            .build();
        let result = Equal::<i64>::default().perform(state).unwrap_err();
        assert!(!result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Overflow { stack_type: "bool" })
        );
    }

    #[proptest]
    fn equal_proptest_same_stack(a: i64, b: i64) {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([a, b])
            .unwrap()
            .with_no_program()
            .build();
        let result = Equal::<i64>::default().perform(state).unwrap();
        prop_assert_eq!(result.stack::<bool>().top().unwrap(), &(a == b));
        prop_assert_eq!(result.stack::<bool>().size(), 1);
    }

    // #[proptest]
    // fn equal_proptest_different_stacks(a: i64, b: f64) {
    //     let state = PushState::builder()
    //         .with_max_stack_size(1)
    //         .with_int_values([a])
    //         .unwrap()
    //         .with_float_values([b])
    //         .unwrap()
    //         .with_no_program()
    //         .build();
    //     let result = Equal::<i64, f64>::default().perform(state).unwrap();
    //     prop_assert_eq!(result.stack::<bool>().top().unwrap(), &(a as f64 ==
    // b));     prop_assert_eq!(result.stack::<bool>().size(), 1);
    // }
}
