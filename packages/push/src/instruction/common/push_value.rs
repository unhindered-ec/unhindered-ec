use std::fmt::Display;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{
    error::{InstructionResult, MapInstructionError},
    push_vm::HasStack,
};

/// An instruction that pushes a value type `T` on the
/// top a stack of type `T` in the given state.
///
/// # Inputs
///
/// The `Push` instruction takes no inputs from the stacks.
///
/// # Behavior
///
/// The `Push` instruction clones its internal value and pushes on the stack of
/// type `T`.
///
/// If the stack is full before this instruction is performed, then
/// a fatal [`StackError::Overflow`](crate::push_vm::stack::StackError::Overflow) is returned,
/// terminating the running of the program.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`Stack<T>` full" column indicates whether the stack of type `T` is
///      full before this instruction is performed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<T>` full |  Success | Note |
/// | ------------- | ------------- | ------------- |
/// | false | ✅ | The internal value is pushed onto the stack |
/// | true | [‼️..](crate::push_vm::stack::StackError::Overflow) | Program is terminated |
///
/// # Errors
///
/// If the stack is full
/// then this returns a [`Error::Fatal`](crate::error::Error::Fatal)
/// [`StackError::Overflow`](crate::push_vm::stack::StackError::Overflow)
/// error.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct PushValue<T>(pub T);

impl<T> PushValue<T> {
    /// Create an instance of `PushValue` with the given `value`.
    pub const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<S, T> Instruction<S> for PushValue<T>
where
    S: Clone + HasStack<T>,
    T: Clone,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        state.with_push(self.0.clone()).map_err_into()
    }
}

impl<T> Display for PushValue<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Push({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use test_strategy::proptest;

    use crate::{
        instruction::{
            Instruction, common::push_value::PushValue, instruction_error::PushInstructionError,
        },
        push_vm::{HasStack, push_state::PushState, stack::StackError},
    };

    #[proptest]
    fn push_variety_of_values_on_empty(value: i64) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .build();
        let push = PushValue(value);
        let result = push.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 1);
        assert_eq!(result.stack::<i64>().top().unwrap(), &value);
    }

    #[proptest]
    fn push_variety_of_values_on_nonempty(value: i64) {
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_int_values([0, 1])
            .unwrap()
            .with_no_program()
            .build();
        let push = PushValue(value);
        let result = push.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 3);
        assert_eq!(result.stack::<i64>(), &[1, 0, value]);
    }

    #[test]
    fn push_full_stack() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .build();
        let push = PushValue(0);
        let result = push.perform(state).unwrap_err();
        assert!(result.is_fatal());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Overflow { stack_type: "i64" })
        );
    }
}
