use crate::{
    error::InstructionResult,
    instruction::{Instruction, NumOpens, instruction_error::PushInstructionError},
    push_vm::{HasStack, program::PushProgram, stack::PushOnto},
};

/// An instruction that duplicates (clones) the top
/// _block_ of the `Exec` stack, adding an implied "open"
/// to crate a block from here to the corresponding `Close`.
///
/// # Inputs
///
/// The `Exec::DupBlock` instruction takes the following inputs:
///    - `Exec` stack
///      - One code block
///
/// # Behavior
///
/// The `Exec::DupBlock` instruction clones the top block on the `Exec` stack
/// leaving both the original and the copy on the `Exec` stack. It has an
/// implied "open" to create a block that runs from this instruction to the
/// first `Close` in a Plushy genome.
///
/// If the `Exec` stack (after this instruction) is empty, this is a no-op,
/// and the state is returned unchanged.
///
/// If the `Exec` stack is full before this instruction is performed, then
/// a fatal [`StackError::Overflow`](crate::push_vm::stack::StackError::Overflow)
/// returned, terminating the running of the program.
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
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct DupBlock;

impl NumOpens for DupBlock {
    fn num_opens(&self) -> usize {
        1
    }
}

impl<S> Instruction<S> for DupBlock
where
    S: Clone + HasStack<PushProgram>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        state.stack::<PushProgram>().top().cloned().push_onto(state)
    }
}

#[cfg(test)]
mod tests {
    use super::DupBlock;
    use crate::{
        instruction::{ExecInstruction, Instruction, PushInstructionError},
        list_into::arr_into,
        push_vm::{program::PushProgram, push_state::PushState, stack::StackError},
    };

    #[test]
    fn exec_present_not_full() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_program([ExecInstruction::noop()])
            .unwrap()
            .with_instruction_step_limit(1000)
            .build();
        let result_state = DupBlock.perform(state).unwrap();
        assert_eq!(
            result_state.exec,
            arr_into![<PushProgram> ExecInstruction::noop(), ExecInstruction::noop()]
        );
    }

    #[test]
    fn exec_empty() {
        let state = PushState::builder()
            .with_max_stack_size(0)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let result_error = DupBlock.perform(state).unwrap_err();
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
            .with_program([ExecInstruction::noop()])
            .unwrap()
            .with_instruction_step_limit(1000)
            .build();
        let result_error = DupBlock.perform(state).unwrap_err();
        assert!(result_error.is_fatal());
        assert!(matches!(
            result_error.error(),
            PushInstructionError::StackError(StackError::Overflow { .. }),
        ));
    }
}
