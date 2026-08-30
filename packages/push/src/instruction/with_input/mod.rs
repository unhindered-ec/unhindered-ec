mod error;

use std::fmt::Display;

pub use error::WithInputInstructionError;

use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::{Instruction, PushInstruction},
    push_vm::variables::{HasInputs, UnknownVariableError, VariableName},
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct WithInputInstruction(pub VariableName);

impl Display for WithInputInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<VariableName> for WithInputInstruction {
    fn from(var_name: VariableName) -> Self {
        Self(var_name)
    }
}

impl From<&str> for WithInputInstruction {
    fn from(var_name: &str) -> Self {
        Self::from(VariableName::from(var_name))
    }
}

impl From<WithInputInstruction> for PushInstruction {
    fn from(value: WithInputInstruction) -> Self {
        Self::WithInput(value)
    }
}

impl<S> Instruction<S> for WithInputInstruction
where
    S: HasInputs,
{
    type Error = WithInputInstructionError<<S::InputInstruction as Instruction<S>>::Error>;

    /// # Errors
    ///
    /// - Returns an error if performing the input instruction associated with
    ///   this variable returns an error.
    /// - Returns a fatal [`UnknownVariableError`] if there is an attempt to
    ///   lookup the value of a variable that isn't registered with the state.
    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let Some(instruction) = state.get_input_instruction(&self.0) else {
            return Err(Error::fatal(state, UnknownVariableError(self.0.clone())));
        };

        instruction
            .perform(state)
            .map_inner_err(WithInputInstructionError::Instruction)
    }
}
