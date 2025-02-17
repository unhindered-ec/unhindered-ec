use std::marker::PhantomData;

use super::{Instruction, instruction_error::PushInstructionError};
use crate::{
    error::InstructionResult,
    push_vm::{HasStack, stack::PushOnto},
};

pub struct Dup<T> {
    _p: PhantomData<T>,
}

impl<S, T> Instruction<S> for Dup<T>
where
    S: Clone + HasStack<T>,
    T: Clone,
{
    type Error = PushInstructionError;

    fn perform(&self, state: S) -> InstructionResult<S, Self::Error> {
        state.stack::<T>().top().cloned().push_onto(state)
    }
}

// TODO: Write some tests
//  - Multiple tops
//  - Empty stack
//  - Stacks with multiple items
//  - Bring tests over from `dup_block.rs`

// TODO: Get rid of `DupBlock`
