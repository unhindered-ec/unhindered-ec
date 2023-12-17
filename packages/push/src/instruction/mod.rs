use ordered_float::OrderedFloat;

use crate::{
    error::{Error, InstructionResult},
    push_vm::{push_state::PushState, stack::StackError, HasStack},
};
use std::{fmt::Debug, fmt::Display, sync::Arc};

pub use self::{bool::BoolInstruction, float::FloatInstruction, int::IntInstruction};
pub use self::{bool::BoolInstructionError, int::IntInstructionError};

mod bool;
mod float;
pub mod instruction_error;
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

#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PushInstruction {
    InputVar(VariableName),
    Block(Vec<PushInstruction>),
    BoolInstruction(BoolInstruction),
    IntInstruction(IntInstruction),
    FloatInstruction(FloatInstruction),
}

impl Debug for PushInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputVar(instruction) => write!(f, "{instruction}"),
            Self::Block(block) => write!(f, "{block:?}"),
            Self::BoolInstruction(instruction) => write!(f, "Bool-{instruction}"),
            Self::IntInstruction(instruction) => write!(f, "Int-{instruction:?}"),
            Self::FloatInstruction(instruction) => write!(f, "Float-{instruction:?}"),
        }
    }
}

impl PushInstruction {
    #[must_use]
    pub fn push_bool(b: bool) -> Self {
        BoolInstruction::Push(b).into()
    }

    #[must_use]
    pub fn push_int(i: i64) -> Self {
        IntInstruction::Push(i).into()
    }

    #[must_use]
    pub fn push_float(f: OrderedFloat<f64>) -> Self {
        FloatInstruction::Push(f).into()
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
            Self::Block(block) => block.perform(state),
            Self::BoolInstruction(i) => i.perform(state),
            Self::IntInstruction(i) => i.perform(state),
            Self::FloatInstruction(i) => i.perform(state),
        }
    }
}

// This is for "performing" an instruction that is in
// fact a block of instructions. To perform this instruction
// we need to push all the instructions in the block onto
// the stack in the correct order, i.e., the first instruction
// in the block should be the top instruction on the exec
// stack after all the pushing is done.
impl<S, I> Instruction<S> for Vec<I>
where
    S: HasStack<I>,
    I: Instruction<S> + Clone,
    I::Error: From<StackError>,
{
    type Error = I::Error;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        // If the size of the block + the size of the exec stack exceed the max stack size
        // then we generate a fatal error.
        if let Err(err) = state.stack_mut::<I>().try_extend(self.iter().cloned()) {
            return Err(Error::fatal(state, err));
        }
        Ok(state)
    }
}
