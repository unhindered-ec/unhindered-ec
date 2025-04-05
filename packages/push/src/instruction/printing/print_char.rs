use std::io::Write;

use super::super::{Instruction, instruction_error::PushInstructionError};
use crate::{error::InstructionResult, push_vm::push_io::HasStdout};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintChar<const CHAR: char>;

impl<const CHAR: char> PrintChar<CHAR> {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<State, const CHAR: char> Instruction<State> for PrintChar<CHAR>
where
    State: HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        // We need to remove this `unwrap()`.
        writeln!(state.stdout(), "{CHAR}").unwrap();
        Ok(state)
    }
}

pub type PrintSpace = PrintChar<' '>;

pub type PrintNewline = PrintChar<'n'>;
