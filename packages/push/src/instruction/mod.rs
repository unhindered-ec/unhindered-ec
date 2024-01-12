use ordered_float::OrderedFloat;

use crate::{push::instruction::instruction_error::PushInstructionError,error::InstructionResult, push_vm::push_state::PushState};

pub use self::int::IntInstructionError;
use self::variable_name::VariableName;
pub use self::{
    bool::BoolInstruction, exec::ExecInstruction, float::FloatInstruction, int::IntInstruction,
};

mod bool;
mod exec;
mod float;
pub mod instruction_error;
mod int;
pub mod variable_name;

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

#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PushInstruction {
    InputVar(VariableName),
    Exec(ExecInstruction),
    BoolInstruction(BoolInstruction),
    IntInstruction(IntInstruction),
    FloatInstruction(FloatInstruction),
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
            Self::Exec(_) => todo!(),
            Self::BoolInstruction(i) => i.perform(state),
            Self::IntInstruction(i) => i.perform(state),
            Self::FloatInstruction(i) => i.perform(state),
        }
    }
}

pub trait NumOpens {
    fn num_opens(&self) -> usize {
        0
    }
}

impl NumOpens for PushInstruction {
    fn num_opens(&self) -> usize {
        match self {
            Self::Exec(i) => i.num_opens(),
            _ => 0,
        }
    }
}

impl std::fmt::Debug for PushInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputVar(instruction) => write!(f, "{instruction}"),
            Self::Exec(instruction) => write!(f, "Exec-{instruction:?}"),
            Self::BoolInstruction(instruction) => write!(f, "Bool-{instruction}"),
            Self::IntInstruction(instruction) => write!(f, "Int-{instruction:?}"),
            Self::FloatInstruction(instruction) => write!(f, "Float-{instruction:?}"),
        }
    }
}
