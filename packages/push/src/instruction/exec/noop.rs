use crate::instruction::{Instruction, NumOpens, instruction_error::PushInstructionError};

/// A "no-op" instruction that does nothing, i.e., always
/// runs successfully and makes no changes to the stacks.
///
/// # Inputs
///
/// This has no inputs and ignores the contents of all the stacks.
///
/// # Behavior
///
/// This always succeeds and makes no changes to any of the stacks.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Noop;

impl NumOpens for Noop {
    fn num_opens(&self) -> usize {
        0
    }
}

impl<S> Instruction<S> for Noop {
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> crate::error::InstructionResult<S, Self::Error> {
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::Noop;
    use crate::{
        instruction::{ExecInstruction, Instruction},
        push_vm::push_state::PushState,
    };

    #[test]
    fn noop_is_correct() {
        let state = PushState::builder()
            .with_max_stack_size(2)
            .with_program([ExecInstruction::noop(), ExecInstruction::noop()])
            .unwrap()
            .with_instruction_step_limit(1000)
            .build();
        let result_state = Noop.perform(state.clone()).unwrap();
        assert_eq!(result_state, state);
    }
}
