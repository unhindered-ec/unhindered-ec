use miette::Diagnostic;

use super::IntInstructionError;
use crate::push_vm::stack::StackError;

/// An error that can occur when performing a `PushInstruction`.
#[derive(thiserror::Error, Debug, Eq, PartialEq, Diagnostic)]
pub enum PushInstructionError {
    /// Stack errors can be things like stack over- or underflows.
    #[error(transparent)]
    StackError(
        #[from]
        #[diagnostic_source]
        StackError,
    ),
    #[error("Exceeded the maximum step limit {step_limit}")]
    #[diagnostic(help = "You might want to increase your step limit.")]
    StepLimitExceeded { step_limit: usize },
    /// Int errors can be things like integer overflows.
    #[error(transparent)]
    Int(
        #[from]
        #[diagnostic_source]
        IntInstructionError,
    ),
}
