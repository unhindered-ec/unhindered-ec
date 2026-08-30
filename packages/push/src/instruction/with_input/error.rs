use std::fmt::Display;

use miette::Diagnostic;

use crate::{
    instruction::instruction_error::PushInstructionError, push_vm::variables::UnknownVariableError,
};

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
