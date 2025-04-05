use std::marker::PhantomData;

use crate::{
    error::{InstructionResult, MapInstructionError},
    instruction::{Instruction, NumOpens, instruction_error::PushInstructionError},
    push_vm::HasStack,
};

/// An instruction that pushes a boolean onto the bool stack
/// indicating if the stack of type `T` is empty or not.
///
/// # Inputs
///
/// The `IsEmpty` instruction doesn't take any inputs from
/// the stacks, but does check the stack type `T` for whether it
/// is empty or not.
///
/// # Behavior
///
/// The `IsEmpty` instruction pushes a boolean onto the boolean
/// stack indicating if the stack of type `T` is empty or not.
///
/// If the type `T` is `bool`, this will push `true` or `false` onto that
/// stack depending on whether the `bool` stack was empty before the
/// instruction was performed. Otherwise the stack of type `T` is unchanged.
///
/// If the boolean stack is full before this instruction is performed, then
/// a fatal [`StackError::Overflow`](crate::push_vm::stack::StackError::Overflow) is returned,
/// terminating the running of the program.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "`Stack<T>`" column indicates the is empty or not.
///    - The "`Stack<bool>` full" column indicates whether the stack of type
///      `bool` is full before this instruction is performed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | `Stack<T>`  | `Stack<bool>` full |  Success | Note |
/// | ------------- | ------------- | ------------- | ------------- |
/// | empty  | false | ✅ | `true` is pushed onto the `bool` stack |
/// | not empty  | false | ✅ | `false` is pushed onto the `bool` stack |
/// | irrelevant  | true | [‼️..](crate::push_vm::stack::StackError::Overflow) | Program is terminated |
///
/// # Errors
///
/// If the stack access returns any error other than a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// then this returns that as a [`Error::Fatal`](crate::error::Error::Fatal)
/// error.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct IsEmpty<T> {
    _p: PhantomData<T>,
}

impl<T> IsEmpty<T> {
    pub const fn new() -> Self {
        Self { _p: PhantomData }
    }
}

impl<T> NumOpens for IsEmpty<T> {
    fn num_opens(&self) -> usize {
        0
    }
}

impl<S, T> Instruction<S> for IsEmpty<T>
where
    S: HasStack<T> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let is_empty = state.stack::<T>().is_empty();
        state.with_push(is_empty).map_err_into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruction::{
            Instruction, common::is_empty::IsEmpty, instruction_error::PushInstructionError,
        },
        push_vm::{HasStack, push_state::PushState, stack::StackError},
    };

    #[test]
    fn i64_stack_empty() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let is_empty = IsEmpty::<i64>::default();
        let result = is_empty.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 0);
        assert_eq!(result.stack::<bool>().size(), 1);
        assert_eq!(result.stack::<bool>().top().unwrap(), &true);
    }

    #[test]
    fn i64_stack_not_empty() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values([7])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let is_empty = IsEmpty::<i64>::default();
        let result = is_empty.perform(state).unwrap();
        assert_eq!(result.stack::<i64>().size(), 1);
        assert_eq!(result.stack::<i64>().top().unwrap(), &7);
        assert_eq!(result.stack::<bool>().size(), 1);
        assert_eq!(result.stack::<bool>().top().unwrap(), &false);
    }

    #[test]
    fn bool_stack_empty() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let is_empty = IsEmpty::<bool>::default();
        let result = is_empty.perform(state).unwrap();
        assert_eq!(result.stack::<bool>().size(), 1);
        assert_eq!(result.stack::<bool>().top().unwrap(), &true);
    }

    #[test]
    fn bool_stack_not_empty() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_bool_values([true])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let is_empty = IsEmpty::<bool>::default();
        let result = is_empty.perform(state).unwrap();
        assert_eq!(result.stack::<bool>().size(), 2);
        assert_eq!(result.stack::<bool>().top2().unwrap(), (&false, &true));
    }

    #[test]
    fn is_empty_full_bool_stack() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_bool_values([true])
            .unwrap()
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let is_empty = IsEmpty::<i64>::default();
        let result = is_empty.perform(state).unwrap_err();
        assert!(result.is_fatal());
        assert_eq!(
            result.error(),
            &PushInstructionError::StackError(StackError::Overflow { stack_type: "bool" })
        );
    }
}
