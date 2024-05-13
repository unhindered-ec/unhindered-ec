use crate::instruction::{instruction_error::PushInstructionError, Instruction, NumOpens};

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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
