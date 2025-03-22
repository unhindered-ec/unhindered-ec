use std::marker::PhantomData;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{
    error::{Error, InstructionResult},
    instruction::NumOpens,
    push_vm::HasStack,
};

/// An instruction that pops (removes) the top
/// value of a stack of type `T` in the given state.
///
/// # Inputs
///
/// The `Pop` instruction takes the following inputs:
///    - Stack of type `T`
///      - One value
///
/// # Behavior
///
/// The `Pop` instruction removes the top value on the stack of type `T`.
///
/// If stack is empty, this is a no-op,
/// and the state is returned unchanged.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`Stack<T>`" column indicates the value of the top of the stack of
///      type `T`, or whether it exists.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<T>`  | Success | Note |
/// | ------------- | ------------- | ------------- |
/// | exists  | ✅ | The top value is removed |
/// | missing | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
///
/// # Errors
///
/// The only possible error here is
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// and the instruction will then be ignored.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Pop<T> {
    _p: PhantomData<T>,
}

impl<T> Pop<T> {
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<T> NumOpens for Pop<T> {
    fn num_opens(&self) -> usize {
        0
    }
}

impl<S, T> Instruction<S> for Pop<T>
where
    S: Clone + HasStack<T>,
    T: Clone,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        match state.stack_mut::<T>().pop() {
            Ok(_) => Ok(state),
            // If `pop()` fails it's because the stack was empty and we
            // want to return a recoverable error so the `pop` instruction
            // can be ignored.
            Err(error) => Err(Error::recoverable(state, error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::sample::size_range;
    use test_strategy::proptest;

    use super::*;
    use crate::push_vm::{push_state::PushState, stack::StackError};

    #[test]
    fn pop_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .build();
        let pop = Pop::<i64>::default();
        let result = pop.perform(state).unwrap_err();
        assert!(result.is_recoverable());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Underflow {
                num_requested: 1,
                num_present: 0
            })
        );
    }

    #[test]
    fn pop_non_empty_stack() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values([17])
            .unwrap()
            .with_no_program()
            .build();
        let pop = Pop::<i64>::default();
        let result = pop.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 0);
    }

    // Test that `Pop` works on stacks of 1..100 elements of type `i64`.
    #[proptest]
    fn pop_stacks_with_multiple_items(#[any(size_range(1..100).lift())] values: Vec<i64>) {
        let state = PushState::builder()
            .with_max_stack_size(100)
            .with_int_values(values.clone().into_iter().rev())
            .unwrap()
            .with_no_program()
            .build();
        let pop = Pop::<i64>::default();
        let result = pop.perform(state).unwrap();
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "The length of `values` is at least 1, so subtracting 1 is guaranteed to be \
                      safe."
        )]
        let expected_num_values = values.len() - 1;
        assert_eq!(result.stack::<i64>().size(), expected_num_values);
        assert_eq!(
            result.stack::<i64>(),
            &values.as_slice()[0..expected_num_values]
        );
    }
}
