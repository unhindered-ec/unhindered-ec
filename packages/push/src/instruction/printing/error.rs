use miette::Diagnostic;

use crate::push_vm::stack::StackError;

// Error that can occur when trying to print a value from the string stack
#[derive(thiserror::Error, Debug, PartialEq, Eq, Diagnostic)]
pub enum PrintingError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    StackError(
        #[from]
        #[diagnostic_source]
        StackError,
    ),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Writing(#[from] AppendStdoutError),
}

#[derive(thiserror::Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(help = "Check that your stdout stream is writable and big enough")]
pub struct AppendStdoutError(#[from] std::io::Error);

// TODO: This is kind of hacky, we should really just remove the partialeq
// requirement for errors
impl PartialEq for AppendStdoutError {
    fn eq(&self, other: &Self) -> bool {
        self.0.kind() == other.0.kind() && self.0.to_string() == other.0.to_string()
    }
}

impl Eq for AppendStdoutError {}
