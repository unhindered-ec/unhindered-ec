use std::{fmt::Display, io::Write, marker::PhantomData};

use super::{Instruction, instruction_error::PushInstructionError};
use crate::{
    error::{Error, InstructionResult},
    push_vm::{HasStack, push_io::HasStdout},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Print<T> {
    _p: PhantomData<T>,
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
    _p: PhantomData<T>,
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

pub const PrintSpace: PrintChar<' '> = PrintChar::new();
pub const PrintNewline: PrintChar<'n'> = PrintChar::new();

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PrintString(pub String);

impl PrintString {
    pub const fn new(s: String) -> Self {
        Self(s)
    }
}

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
