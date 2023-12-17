use super::IntInstructionError;
use crate::push_vm::stack::StackError;

/// An error that can occur when performing a `PushInstruction`.
#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum PushInstructionError {
    // Stack errors can be things like stack over- or underflows.
    #[error(transparent)]
    StackError(#[from] StackError),
    #[error("Exceeded the maximum step limit {step_limit}")]
    StepLimitExceeded { step_limit: usize },
    // Int errors can be things like integer overflows.
    #[error(transparent)]
    Int(#[from] IntInstructionError),
}
