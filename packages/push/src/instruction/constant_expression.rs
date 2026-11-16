use ordered_float::OrderedFloat;

use crate::{
    instruction::{Instruction, Perform, common::PushValue},
    push_vm::{HasStack, stack::StackError},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstantExpression {
    Int(PushValue<i64>),
    Float(PushValue<OrderedFloat<f64>>),
    Bool(PushValue<bool>),
    String(PushValue<String>),
}

impl Instruction for ConstantExpression {}

impl<S> Perform<S> for ConstantExpression
where
    S: Clone + HasStack<i64> + HasStack<OrderedFloat<f64>> + HasStack<bool> + HasStack<String>,
{
    type Error = StackError;

    fn perform(&self, state: S) -> crate::error::InstructionResult<S, Self::Error> {
        match self {
            Self::Int(push_value) => push_value.perform(state),
            Self::Float(push_value) => push_value.perform(state),
            Self::Bool(push_value) => push_value.perform(state),
            Self::String(push_value) => push_value.perform(state),
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

impl From<PushValue<String>> for ConstantExpression {
    fn from(value: PushValue<String>) -> Self {
        Self::String(value)
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantExpression;
    use crate::{
        instruction::{Perform, common::PushValue},
        push_vm::{HasStack, push_state::PushState},
    };

    #[test]
    fn string_expression_pushes_onto_string_stack() {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let expression = ConstantExpression::from(PushValue::new("a".to_string()));
        let result = expression.perform(state).unwrap();
        assert_eq!(result.stack::<String>().top(), Ok(&"a".to_string()));
    }
}
