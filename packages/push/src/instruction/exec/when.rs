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
pub struct When;

impl<S> Instruction<S> for When
where
    S: Clone + HasStack<PushProgram> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let condition = state.stack::<bool>().top();
        let block = state.stack::<PushProgram>().top();
        match (condition, block) {
            // If there is a boolean that is true and a block, discard the boolean
            // and leave the block so that it may be
            // performed next.
            (Ok(true), Ok(_)) => Ok(state).with_stack_discard::<bool>(1),
            // If there is a boolean that is false and a block, discard both the
            // boolean and the block since we don't
            // want to perform that block.
            (Ok(false), Ok(_)) => Ok(state)
                .with_stack_discard::<bool>(1)
                .with_stack_discard::<PushProgram>(1),
            // If there is no boolean but there is a block, discard the block since
            // we only want to perform it if there is a
            // boolean that is true.
            (Err(StackError::Underflow { .. }), Ok(_)) => {
                Ok(state).with_stack_discard::<PushProgram>(1)
            }
            // If there is no block, then we just skip this instruction regardless
            // of whether there's a boolean or not.
            (Ok(_) | Err(StackError::Underflow { .. }), Err(StackError::Underflow { .. })) => {
                Ok(state)
            }
            (Err(e), _) | (_, Err(e)) => Err(Error::recoverable(state, e)),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::{
        instruction::{exec::when::When, ExecInstruction, Instruction},
        push_vm::push_state::PushState,
    };

    #[test]
    fn when_is_correct_with_all_empty_stacks() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_no_program()
            .with_bool_values([])
            .unwrap()
            .build();
        let result_state = When.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }

    #[test]
    fn when_is_correct_with_empty_exec() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_no_program()
            .with_bool_values([true])
            .unwrap()
            .build();
        let result_state = When.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }

    #[test]
    fn when_is_correct_with_empty_bool() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_program([ExecInstruction::Noop])
            .unwrap()
            .build();
        let result_state = When.perform(state).unwrap();
        assert!(result_state.exec.is_empty());
    }

    #[test]
    fn when_is_correct_with_true() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_program([ExecInstruction::Noop])
            .unwrap()
            .with_bool_values([true])
            .unwrap()
            .build();
        let result_state = When.perform(state.clone()).unwrap();
        assert!(result_state.bool.is_empty());
        assert_eq!(result_state.exec, state.exec);
    }

    #[test]
    fn when_is_correct_with_false() {
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_program([ExecInstruction::Noop])
            .unwrap()
            .with_bool_values([false])
            .unwrap()
            .build();
        let result_state = When.perform(state).unwrap();
        assert!(result_state.bool.is_empty());
        assert!(result_state.exec.is_empty());
    }
}
