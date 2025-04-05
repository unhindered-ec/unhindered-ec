use std::{fmt::Display, io::Write, marker::PhantomData};

use super::super::instruction_error::PushInstructionError;
use crate::{
    error::{Error, InstructionResult},
    instruction::Instruction,
    push_vm::{HasStack, push_io::HasStdout},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Print<T> {
    pub(crate) _p: PhantomData<T>,
}

impl<State, T> Instruction<State> for Print<T>
where
    T: Display,
    State: HasStack<T> + HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        let value = match state.stack_mut::<T>().pop() {
            Ok(value) => value,
            Err(error) => return Err(Error::recoverable(state, error)),
        };
        let stdout = state.stdout();
        // We need to remove this `unwrap()`.
        write!(stdout, "{value}").unwrap();
        Ok(state)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PrintLn<T> {
    pub(crate) _p: PhantomData<T>,
}

impl<State, T> Instruction<State> for PrintLn<T>
where
    T: Display,
    State: HasStack<T> + HasStdout,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: State) -> InstructionResult<State, Self::Error> {
        let value = match state.stack_mut::<T>().pop() {
            Ok(value) => value,
            Err(error) => return Err(Error::recoverable(state, error)),
        };
        let stdout = state.stdout();
        // We need to remove this `unwrap()`.
        writeln!(stdout, "{value}").unwrap();
        Ok(state)
    }
}
