use crate::{
    error::InstructionResult,
    push_vm::{push_state::PushState, stack::StackError, PushInteger},
};
use std::{fmt::Debug, fmt::Display, sync::Arc};

pub use self::{bool::BoolInstruction, int::IntInstruction};
use self::{bool::BoolInstructionError, int::IntInstructionError};

mod bool;
mod int;

/*
 * exec_if requires a boolean and two (additional) values on the exec stack.
 * If the bool is true, we remove the second of the two exec stack values,
 * and if it's false, we remove the first.
 */

/*
 * exec_while requires a boolean and one additional value on the exec stack.
 * If the bool is true, then you push a copy of the "body" onto the exec, followed
 * by another copy of exec_while.
 */

/*
 * Instructions that are generic over stacks:
 *
 * - push
 * - pop
 * - dup (int_dup, exec_dup, bool_dup, ...)
 */

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum PushInstructionError {
    #[error(transparent)]
    StackError(#[from] StackError),
    #[error("Exceeded the maximum step limit {step_limit}")]
    StepLimitExceeded { step_limit: usize },
    #[error(transparent)]
    Int(#[from] IntInstructionError),
    #[error(transparent)]
    Bool(#[from] BoolInstructionError),
}

/// Maps a (presumably error) type into an `InstructionResult`.
/// This is in fact used to convert `InstructionResult<S, E1>`
/// into `InstructionResult<S, E2>`, i.e. do `map_err()` on
/// the inner error types of an `InstructionResult`, preserving
/// the other fields in `Error`.
pub trait MapInstructionError<S, E> {
    ///  
    /// # Errors
    ///
    /// This always returns an error type.
    fn map_err_into(self) -> InstructionResult<S, E>;
}

// MizardX@Twitch's initial suggestion here had `E2` as a generic on the
// _function_ `map_err_into()` instead of at the `impl` level. That provided
// some additional flexibility, although it wasn't that we would use it.
// The current approach (suggested by esitsu@Twitch) simplified the
// `MapInstructionError` trait in a nice way, so I went with that.
impl<S, E1, E2> MapInstructionError<S, E2> for InstructionResult<S, E1>
where
    E1: Into<E2>,
{
    fn map_err_into(self) -> InstructionResult<S, E2> {
        self.map_err(|e| e.map_inner_err(Into::into))
    }
}

pub trait Instruction<S> {
    type Error;

    /// # Errors
    ///
    /// This returns an error if the instruction being performed
    /// returns some kind of error. This could include things like
    /// stack over- or underflows, or numeric errors like integer overflow.
    fn perform(&self, state: S) -> InstructionResult<S, Self::Error>;
}

impl<S, E> Instruction<S> for Box<dyn Instruction<S, Error = E>> {
    type Error = E;

    fn perform(&self, state: S) -> InstructionResult<S, E> {
        self.as_ref().perform(state)
    }
}

// impl<F> Instruction for F
// where
//     F: Fn(dyn State) -> dyn State
// {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableName(Arc<str>);

impl From<&str> for VariableName {
    fn from(s: &str) -> Self {
        Self(Arc::from(s))
    }
}

impl Display for VariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod variable_name_test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn variable_name() {
        let x = VariableName::from("x");
        let x2 = VariableName::from("x");
        assert_eq!(x, x2);
        let y = VariableName::from("y");
        assert_ne!(x, y);

        let mut map = HashMap::new();
        map.insert(x.clone(), 5);
        map.insert(y.clone(), 7);

        assert_eq!(map.get(&x).unwrap(), &5);
        assert_eq!(map.get(&y).unwrap(), &7);
        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&x2).unwrap(), &5);

        let z = VariableName::from("z");
        assert_eq!(map.get(&z), None);
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum PushInstruction {
    InputVar(VariableName),
    BoolInstruction(BoolInstruction),
    IntInstruction(IntInstruction),
}

impl Debug for PushInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputVar(arg0) => write!(f, "{arg0}"),
            Self::BoolInstruction(arg0) => write!(f, "Bool-{arg0}"),
            Self::IntInstruction(arg0) => write!(f, "Int-{arg0}"),
        }
    }
}

impl PushInstruction {
    #[must_use]
    pub fn push_bool(b: bool) -> Self {
        BoolInstruction::Push(b).into()
    }

    #[must_use]
    pub fn push_int(i: PushInteger) -> Self {
        IntInstruction::Push(i).into()
    }
}

impl Instruction<PushState> for PushInstruction {
    type Error = PushInstructionError;

    fn perform(&self, state: PushState) -> InstructionResult<PushState, Self::Error> {
        match self {
            Self::InputVar(var_name) => {
                // TODO: Should `push_input` return the new state?
                //   Or add a `with_input` that returns the new state and keep `push_input`?
                state.with_input(var_name)
            }
            Self::BoolInstruction(i) => i.perform(state),
            Self::IntInstruction(i) => i.perform(state),
        }
    }
}
