use miette::Diagnostic;

use crate::push_vm::variables::VariableName;

#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Diagnostic)]
#[error("Attempt to look up unknown variable \"{0}\"")]
#[diagnostic(help = "Make sure you assign a value to \"{0}\" when you set up your state")]
pub struct UnknownVariableError(pub VariableName);
