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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {

    use proptest::prop_assert_eq;
    use test_strategy::proptest;

    use crate::{
        instruction::{Instruction, IntInstruction},
        push_vm::{push_state::PushState, HasStack},
    };

    #[proptest]
    fn negate(#[any] x: i64) {
        let state = PushState::builder()
            .with_max_stack_size(1)
            .with_int_values(std::iter::once(x))
            .unwrap()
            .with_no_program()
            .build();
        let result = IntInstruction::negate().perform(state).unwrap();
        prop_assert_eq!(result.stack::<i64>().size(), 1);
        prop_assert_eq!(*result.stack::<i64>().top().unwrap(), -x);
    }
}
