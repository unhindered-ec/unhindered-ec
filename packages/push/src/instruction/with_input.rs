use std::fmt::Display;

use miette::Diagnostic;

use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    instruction::{Instruction, PushInstruction, instruction_error::PushInstructionError},
    push_vm::variables::{HasInputs, VariableName},
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

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Diagnostic)]
#[error("Attempt to lookup unknown variable \"{0}\"")]
#[diagnostic(help = "Make sure you assign a value to \"{0}\" when you set up your state")]
pub struct UnknownVariableError(pub VariableName);

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum WithInputInstructionError<T> {
    #[error(transparent)]
    Instruction(T),
    #[error(transparent)]
    UnknownVariable(#[from] UnknownVariableError),
}

impl<T> Diagnostic for WithInputInstructionError<T>
where
    T: Diagnostic,
{
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::Instruction(e) => e.code(),
            Self::UnknownVariable(e) => e.code(),
        }
    }

    fn severity(&self) -> Option<miette::Severity> {
        match self {
            Self::Instruction(e) => e.severity(),
            Self::UnknownVariable(e) => e.severity(),
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::Instruction(e) => e.help(),
            Self::UnknownVariable(e) => e.help(),
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        match self {
            Self::Instruction(e) => e.url(),
            Self::UnknownVariable(e) => e.url(),
        }
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        match self {
            Self::Instruction(e) => e.source_code(),
            Self::UnknownVariable(e) => e.source_code(),
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self {
            Self::Instruction(e) => e.labels(),
            Self::UnknownVariable(e) => e.labels(),
        }
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        match self {
            Self::Instruction(e) => e.related(),
            Self::UnknownVariable(e) => e.related(),
        }
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::Instruction(e) => e.diagnostic_source(),
            Self::UnknownVariable(e) => e.diagnostic_source(),
        }
    }
}

impl<T> From<WithInputInstructionError<T>> for PushInstructionError
where
    T: Into<Self>,
{
    fn from(err: WithInputInstructionError<T>) -> Self {
        match err {
            WithInputInstructionError::Instruction(i) => i.into(),
            WithInputInstructionError::UnknownVariable(v) => v.into(),
        }
    }
}

impl<S> Instruction<S> for WithInputInstruction
where
    S: HasInputs,
{
    type Error = WithInputInstructionError<<S::InputInstruction as Instruction<S>>::Error>;

    /// # Errors
    ///
    /// This returns an error if performing the input instruction associated
    /// with this variable returns an error.
    ///
    /// # Panics
    ///
    /// This panics if there is no instruction associated with `var_name`, i.e.,
    /// we have not yet added that variable name to the map of names to
    /// instructions.
    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let Some(instruction) = state.get_input_instruction(&self.0) else {
            return Err(Error::fatal(state, UnknownVariableError(self.0.clone())));
        };

        instruction
            .perform(state)
            .map_inner_err(WithInputInstructionError::Instruction)
    }
}
