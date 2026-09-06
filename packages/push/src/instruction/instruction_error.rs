use miette::Diagnostic;

use super::{
    IntInstructionError,
    printing::{AppendStdoutError, PrintingError},
};
use crate::push_vm::{stack::StackError, variables::UnknownVariableError};

/// An error that can occur when performing a `PushInstruction`.
#[derive(thiserror::Error, Debug, Eq, PartialEq, Diagnostic)]
#[error("There was a error evaluating a `PushInstruction`")]
pub enum PushInstructionError {
    /// Stack errors can be things like stack over- or underflows.
    // #[error("transparent")]
    #[diagnostic(transparent)]
    StackError(#[from] StackError),
    #[error("Exceeded the maximum step limit {step_limit}")]
    // The `StepLimitExceeded` variant is usually not seen by the user as it is
    // typically processed by the interpreter, and a value from the appropriate
    // stack is returned.
    #[diagnostic(
        help = "You might want to increase your step limit if {step_limit} seems too low, or else \
                an infinite (or very large) loop may have occurred."
    )]
    StepLimitExceeded { step_limit: usize },

    /// Int errors can be things like integer overflows.
    // #[error(transparent)]
    #[diagnostic(transparent)]
    Int(#[from] IntInstructionError),

    // #[error(transparent)]
    #[diagnostic(transparent)]
    Printing(#[from] PrintingError),

    // #[error(transparent)]
    #[diagnostic(transparent)]
    UnknownVariable(#[from] UnknownVariableError),
}

impl From<AppendStdoutError> for PushInstructionError {
    fn from(value: AppendStdoutError) -> Self {
        Self::Printing(value.into())
    }
}
