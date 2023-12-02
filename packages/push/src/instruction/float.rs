use super::{Instruction, PushInstruction, PushInstructionError};
use crate::{
    error::{Error, InstructionResult},
    instruction::MapInstructionError,
    push_vm::{
        stack::{Stack, StackDiscard, StackError, StackPush},
        HasStack,
    },
};
use ordered_float::OrderedFloat;
use strum_macros::EnumIter;

#[derive(Debug, strum_macros::Display, Copy, Clone, EnumIter, Eq, PartialEq)]
#[non_exhaustive]
pub enum FloatInstruction {
    Push(OrderedFloat<f64>),
    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
    Equal,
    NotEqual,
}

impl From<FloatInstruction> for PushInstruction {
    fn from(instr: FloatInstruction) -> Self {
        Self::FloatInstruction(instr)
    }
}

impl<S> Instruction<S> for FloatInstruction
where
    S: Clone + HasStack<OrderedFloat<f64>> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Push(_) | Self::Add | Self::Subtract | Self::Multiply | Self::ProtectedDivide => {
                // All these instructions pop at least one value from the float stack, so we're
                // guaranteed that there will be space for the result. So we don't have to check that
                // any stacks are full before we start.
                let float_stack = state.stack_mut::<OrderedFloat<f64>>();
                match self {
                    Self::Push(f) => state.with_push(*f).map_err_into(),

                    Self::Add => float_stack
                        .top2()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(&x, &y)| x + y)
                        .with_stack_replace(2, state),

                    Self::Subtract => float_stack
                        .top2()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(&x, &y)| x - y)
                        .with_stack_replace(2, state),

                    Self::Multiply => float_stack
                        .top2()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(x, y)| (*x) * (*y))
                        .with_stack_replace(2, state),

                    Self::ProtectedDivide => float_stack
                        .top2()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(&x, &y)| if y == 0.0 { OrderedFloat(1.0) } else { x / y })
                        .with_stack_replace(2, state),

                    _ => {
                        unreachable!(
                            "We failed to handle an arithmetic float instruction: {self:?}"
                        )
                    }
                }
            }

            Self::Equal | Self::NotEqual => {
                // None of these instructions pop anything off the boolean stack, but
                // they will push a result onto that stack. Thus before we start performing
                // the instruction, we need to check for the case that the boolean stack is
                // already full, and return an `Overflow` error if it is.
                if state.stack::<bool>().is_full() {
                    return Err(Error::fatal(
                        state,
                        StackError::Overflow { stack_type: "bool" },
                    ));
                }
                let float_stack: &mut Stack<OrderedFloat<f64>> =
                    state.stack_mut::<OrderedFloat<f64>>();

                match self {
                    Self::Equal => float_stack
                        .top2()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(&x, &y)| x == y)
                        .with_stack_push(state)
                        .with_stack_discard::<OrderedFloat<f64>>(1),
                    Self::NotEqual => float_stack
                        .top2()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(&x, &y)| x != y)
                        .with_stack_push(state)
                        .with_stack_discard::<OrderedFloat<f64>>(1),
                    _ => unreachable!(
                        "We failed to implement a boolean-valued operation on floats: {self:?}"
                    ),
                }
            }
        }
    }
}
