use super::{Instruction, PushInstruction, PushInstructionError};
use crate::{
    error::Error,
    error::InstructionResult,
    instruction::MapInstructionError,
    push_vm::{stack::StackPush, HasStack},
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
}

impl From<FloatInstruction> for PushInstruction {
    fn from(instr: FloatInstruction) -> Self {
        Self::FloatInstruction(instr)
    }
}

impl<S> Instruction<S> for FloatInstruction
where
    S: Clone + HasStack<OrderedFloat<f64>>,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
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
        }
    }
}
