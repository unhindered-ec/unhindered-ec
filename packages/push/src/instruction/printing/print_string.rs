use std::io::Write;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{error::InstructionResult, push_vm::push_io::HasStdout};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintString(pub String);

impl<State> Instruction<State> for PrintString
where
    State: HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        let stdout = state.stdout();
        // We need to remove this `unwrap()`.
        writeln!(stdout, "{}", self.0).unwrap();
        Ok(state)
    }
}
