use ordered_float::OrderedFloat;

use crate::push::instruction::instruction_error::PushInstructionError;
use crate::{error::InstructionResult, push_vm::push_state::PushState};

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
            Self::Exec(_) => todo!(),
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
