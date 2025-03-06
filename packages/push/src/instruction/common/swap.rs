use std::marker::PhantomData;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{
    error::{Error, InstructionResult, MapInstructionError},
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
///    - The "Stack<T>" column indicates the top two values of the stack of type
///      `T`, or whether they exist.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | Stack<T>  | Success | Note |
/// | ------------- | ------------- | ------------- |
/// | exists, exists  | ✅ | The top two values are swapped |
/// | exists, missing | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | missing, irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// If the stack access returns any error other than a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// then this returns that as a [`Error::Fatal`](crate::error::Error::Fatal)
/// error.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Swap<T> {
    _p: PhantomData<T>,
}

impl<T> Swap<T> {
    pub const fn new() -> Self {
        Self { _p: PhantomData }
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
            .build();
        assert_eq!(state.stack::<i64>().top().unwrap(), &1);
        let result = Swap::<i64>::new().perform(state).unwrap();
        assert_eq!(result.stack::<i64>().top2().unwrap(), (&2, &1));
    }
}
