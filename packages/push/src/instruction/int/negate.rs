use std::ops::Neg;

use crate::{
    error::InstructionResult,
    instruction::{Instruction, PushInstructionError},
    push_vm::{stack::PushOnto, HasStack},
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Negate;

impl<S> Instruction<S> for Negate
where
    S: Clone + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        let int_stack = state.stack::<i64>();
        int_stack.top().map(Neg::neg).replace_on(1, state)
    }
}
