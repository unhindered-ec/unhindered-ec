use crate::{
    error::{Error, InstructionResult},
    instruction::{instruction_error::PushInstructionError, Instruction},
    push_vm::{
        program::PushProgram,
        stack::{StackDiscard, StackError},
        HasStack,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Unless;

impl<S> Instruction<S> for Unless
where
    S: Clone + HasStack<PushProgram> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let condition = state.stack::<bool>().top();
        let block = state.stack::<PushProgram>().top();
        match (condition, block) {
            // If there is a boolean that is false and a block, discard the boolean
            // and leave the block so that it may be
            // performed next.
            (Ok(false), Ok(_)) => Ok(state).with_stack_discard::<bool>(1),
            // If there is a boolean that is true and a block, discard both the
            // boolean and the block since we don't
            // want to perform that block.
            (Ok(true), Ok(_)) => Ok(state)
                .with_stack_discard::<bool>(1)
                .with_stack_discard::<PushProgram>(1),
            // In all other cases, we return the state unchanged. If the was a boolean
            // value, but no block, we skip the instruction without consuming anything.
            // If there was no boolean, we execute the block if there is one.
            (
                Ok(_) | Err(StackError::Underflow { .. }),
                Ok(_) | Err(StackError::Underflow { .. }),
            ) => Ok(state),
            (Err(e), _) | (_, Err(e)) => Err(Error::recoverable(state, e)),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::{
        instruction::{ExecInstruction, Instruction},
        push_vm::push_state::PushState,
    };

    #[test]
    fn unless_is_correct_with_all_empty_stacks() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_no_program()
            .with_bool_values([])
            .unwrap()
            .build();
        let result_state = Unless.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }

    #[test]
    fn unless_is_correct_with_empty_exec() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_no_program()
            .with_bool_values([true])
            .unwrap()
            .build();
        let result_state = Unless.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }

    #[test]
    fn unless_is_correct_with_empty_bool() {
        // If there's no boolean, we leave the state unchanged and skip the instruction.
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_program([ExecInstruction::Noop])
            .unwrap()
            .build();
        let result_state = Unless.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }

    #[test]
    fn unless_is_correct_with_true() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_program([ExecInstruction::Noop])
            .unwrap()
            .with_bool_values([true])
            .unwrap()
            .build();
        let result_state = Unless.perform(state).unwrap();
        assert!(result_state.bool.is_empty());
        assert!(result_state.exec.is_empty());
    }

    #[test]
    fn unless_is_correct_with_false() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_program([ExecInstruction::Noop])
            .unwrap()
            .with_bool_values([false])
            .unwrap()
            .build();
        let result_state = Unless.perform(state.clone()).unwrap();
        assert!(result_state.bool.is_empty());
        assert_eq!(result_state.exec, state.exec);
    }
}
