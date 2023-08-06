use super::{Error, Instruction, InstructionResult, PushInstruction, PushInstructionError};
use crate::push_vm::push_state::{HasStack, Stack};
use std::ops::Not;
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
#[allow(clippy::module_name_repetitions)]
pub enum BoolInstruction {
    Push(bool),
    BoolNot,
    BoolOr,
    BoolAnd,
    BoolXor,
    BoolImplies,
    // Do we really want either of these? Do they get used?
    // BooleanInvertFirstThenAnd,
    // BoolInvertSecondThenAnd,
    BoolFromInt,
    // BoolFromFloat,
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum BoolInstructionError {}

impl<S> Instruction<S> for BoolInstruction
where
    S: HasStack<bool> + HasStack<i64>,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        let bool_stack: &mut Stack<bool> = state.stack_mut();
        let result = match self {
            Self::Push(b) => Ok(*b),
            Self::BoolNot => bool_stack.pop().map(Not::not),
            Self::BoolAnd => bool_stack.pop2().map(|(x, y)| x && y),
            Self::BoolOr => bool_stack.pop2().map(|(x, y)| x || y),
            Self::BoolXor => bool_stack.pop2().map(|(x, y)| x != y),
            Self::BoolImplies => bool_stack.pop2().map(|(x, y)| !x || y),
            Self::BoolFromInt => {
                let int_stack: &mut Stack<i64> = state.stack_mut();
                int_stack.pop().map(|i| i != 0)
            }
        };
        let b = result.map_err(|error| Error::recoverable_error(state, error))?;
        bool_stack
            .push(b)
            .map_err(|error| Error::fatal_error(state, error))?;
        Ok(state)
    }
}

impl From<BoolInstruction> for PushInstruction {
    fn from(instr: BoolInstruction) -> Self {
        Self::BoolInstruction(instr)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod property_tests {
    use crate::{
        instruction::{BoolInstruction, Instruction},
        push_vm::push_state::{HasStack, PushState},
    };
    use proptest::{prop_assert_eq, proptest};
    use strum::IntoEnumIterator;

    fn all_instructions() -> Vec<BoolInstruction> {
        BoolInstruction::iter().collect()
    }

    proptest! {
        #[test]
        fn ops_do_not_crash(instr in proptest::sample::select(all_instructions()),
                x in proptest::bool::ANY, y in proptest::bool::ANY, i in proptest::num::i64::ANY) {
            let mut state = PushState::builder([])
                .with_bool_values(vec![x, y])
                .with_int_values(vec![i])
                .build();
            instr.perform(state);
        }

        #[test]
        fn and_is_correct(x in proptest::bool::ANY, y in proptest::bool::ANY) {
            let mut state = PushState::builder([])
                .with_bool_values(vec![x, y])
                .build();
            BoolInstruction::BoolAnd.perform(state);
            #[allow(clippy::unwrap_used)]
            let result: &bool = state.stack_mut().top().unwrap();
            prop_assert_eq!(*result, x && y);
        }

        #[test]
        fn implies_is_correct(x in proptest::bool::ANY, y in proptest::bool::ANY) {
            let mut state = PushState::builder([])
                .with_bool_values(vec![x, y])
                .build();
            BoolInstruction::BoolImplies.perform(state);
            #[allow(clippy::unwrap_used)]
            let result: &bool = state.stack_mut().top().unwrap();
            prop_assert_eq!(*result, !x || y);
        }
    }
}
