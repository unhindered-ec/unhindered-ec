use ordered_float::OrderedFloat;

use crate::{
    instruction::{Instruction, common::PushValue, instruction_error::PushInstructionError},
    push_vm::HasStack,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstantExpression {
    Int(PushValue<i64>),
    Float(PushValue<OrderedFloat<f64>>),
    Bool(PushValue<bool>),
}

impl<S> Instruction<S> for ConstantExpression
where
    S: Clone + HasStack<i64> + HasStack<OrderedFloat<f64>> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> crate::error::InstructionResult<S, Self::Error> {
        match self {
            Self::Int(push_value) => push_value.perform(state),
            Self::Float(push_value) => push_value.perform(state),
            Self::Bool(push_value) => push_value.perform(state),
        }
    }
}

impl From<PushValue<i64>> for ConstantExpression {
    fn from(value: PushValue<i64>) -> Self {
        Self::Int(value)
    }
}

impl From<PushValue<OrderedFloat<f64>>> for ConstantExpression {
    fn from(value: PushValue<OrderedFloat<f64>>) -> Self {
        Self::Float(value)
    }
}

impl From<PushValue<bool>> for ConstantExpression {
    fn from(value: PushValue<bool>) -> Self {
        Self::Bool(value)
    }
}
