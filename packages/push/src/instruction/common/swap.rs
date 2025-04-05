use std::marker::PhantomData;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::NumOpens,
    push_vm::HasStack,
};

/// An instruction that swaps the top two values of a stack of type `T`.
///
/// # Inputs
///
/// The `Swap` instruction takes the following inputs:
///    - Stack of type `T`
///      - Two values
///
/// # Behavior
///
/// The `Swap` instruction swaps the top two values on the stack of type `T`.
///
/// If the stack has fewer than two items, it will fail,
/// and the state is returned unchanged.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`Stack<T>`" column indicates the top two values of the stack of
///      type `T`, or whether they exist.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<T>` top | `Stack<T>` next-to-top | Success | Note |
/// | ------------- | ------------- | ------------- |
/// | exists | exists  | ✅ | The top two values are swapped |
/// | exists | missing | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | missing | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// This returns a [`Error::Recoverable`]
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// error if `Stack<T>` doesn't have at least two values.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Swap<T> {
    _p: PhantomData<T>,
}

impl<T> Swap<T> {
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<T> NumOpens for Swap<T> {
    fn num_opens(&self) -> usize {
        0
    }
}

impl<S, T> Instruction<S> for Swap<T>
where
    S: Clone + HasStack<T>,
    T: Clone,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        match state.stack_mut::<T>().pop2() {
            Ok((x, y)) => state
                .with_push(x)
                .map_err_into()?
                .with_push(y)
                .map_err_into(),
            Err(error) => Err(Error::recoverable(state, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::{collection::vec, prelude::any};
    use test_strategy::proptest;

    use super::*;
    use crate::{
        instruction::Instruction,
        push_vm::{push_state::PushState, stack::StackError},
    };

    #[test]
    fn swap_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Swap::<i64>::new().perform(state).unwrap_err();
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
    fn swap_singleton_stack() {
        // Check that calling `Swap` on a stack with one element returns a
        // `StackError::Underflow` error.
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values([1])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result = Swap::<i64>::new().perform(state).unwrap_err();
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 2,
                num_present: 1
            })
        );
    }

    #[test]
    fn swap_non_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_int_values([1, 2])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        assert_eq!(state.stack::<i64>().top().unwrap(), &1);
        let result = Swap::<i64>::new().perform(state).unwrap();
        assert_eq!(result.stack::<i64>().top2().unwrap(), (&2, &1));
    }

    #[proptest]
    fn swap_on_stack_with_multiple_items(
        // We want at least two items so the swap can actually happen.
        #[strategy(vec(any::<i64>(), 2..100))] mut values: Vec<i64>,
    ) {
        let state = PushState::builder()
            .with_max_stack_size(100)
            .with_int_values(values.clone().into_iter())
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let swap = Swap::<i64>::default();
        let result = swap.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), values.len());
        // We need to swap the first two elements of the set of values
        // to mimic the swap, and then reverse the set of values since
        // they are essentially reversed when they are initially pushed
        // onto the stack.
        values.swap(0, 1);
        values.reverse();
        // Now the set of values of the stack should be the same as `values`.
        assert_eq!(result.stack::<i64>(), &values);
    }
}
