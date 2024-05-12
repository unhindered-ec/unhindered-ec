use crate::{
    error::InstructionResult,
    instruction::{instruction_error::PushInstructionError, Instruction, NumOpens},
    push_vm::{program::PushProgram, stack::PushOnto, HasStack},
};

/// A `Dup` instruction for the `Exec` stack that duplicates (clones) the top
/// element of the `Exec` stack.
///
/// # Inputs
///
/// The `Exec::Dup` instruction takes the following inputs:
///    - `Exec` stack
///      - One code block
///
/// # Behavior
///
/// The `Exec::Dup` instruction clones the top block on the `Exec` stack
/// leaving both the original and the copy on the `Exec` stack.
///
/// If the `Exec` stack (after this instruction) is empty, this is a no-op,
/// and the state is returned unchanged.
///
/// If the `Exec` stack is full before this instruction is performed, then
/// a fatal `[StackError::StackOverflow]` returned, terminating the running
/// of the program.
///
/// ## Action Table
///
/// The table below indicates the behavior in each of the different
/// cases.
///
///    - The "Exec stack" column indicates the value of the top of the Exec
///      stack, or whether it exists.
///    - The "Exec stack full" column indicates whether the `Exec` stack is full
///      before this instruction is performed.
///    - The "Success" column indicates whether the instruction succeeds, and if
///      not what kind of error is returned:
///       - ✅: success (so two copies of the top block on the `Exec stack`)
///       - ❗: recoverable error, with links to the error kind
///       - ‼️: fatal error, with links to the error kind
///    - The "Note" column briefly summarizes the action state in that case
///
/// | Exec stack  | Exec stack full |  Success | Note |
/// | ------------- | ------------- | ------------- | ------------- |
/// | exists  | false | ✅ | The top block is cloned |
/// | missing | irrelevant | [❗…](crate::push_vm::stack::StackError::Underflow) | State is unchanged |
/// | exists  | true | [‼️..](crate::push_vm::stack::StackError::Overflow) | Program is terminated |
///
/// # Errors
///
/// If the stack access returns any error other than a
/// [`StackError::Underflow`](crate::push_vm::stack::StackError::Underflow)
/// then this returns that as a [`Error::Fatal`](crate::error::Error::Fatal)
/// error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Dup;

impl NumOpens for Dup {
    fn num_opens(&self) -> usize {
        1
    }
}

impl<S> Instruction<S> for Dup
where
    S: Clone + HasStack<PushProgram>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        state
            .stack::<PushProgram>()
            .top()
            .map_err(PushInstructionError::from)
            .cloned()
            .push_onto(state)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::{
        instruction::{ExecInstruction, Instruction},
        list_into::arr_into,
        push_vm::{push_state::PushState, stack::StackError},
    };

    #[test]
    fn exec_present_not_full() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_program([ExecInstruction::Noop])
            .unwrap()
            .build();
        let result_state = Dup.perform(state).unwrap();
        assert_eq!(
            result_state.exec,
            arr_into![<PushProgram> ExecInstruction::Noop, ExecInstruction::Noop]
        );
    }

    #[test]
    fn exec_empty() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .build();
        let result_error = Dup.perform(state).unwrap_err();
        assert!(result_error.is_recoverable());
        assert!(matches!(
            result_error.error(),
            PushInstructionError::StackError(StackError::Underflow { .. }),
        ));
    }

    #[test]
    fn exec_present_and_full() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_program([ExecInstruction::Noop])
            .unwrap()
            .build();
        let result_error = Dup.perform(state).unwrap_err();
        assert!(result_error.is_fatal());
        assert!(matches!(
            result_error.error(),
            PushInstructionError::StackError(StackError::Overflow { .. }),
        ));
    }
}
