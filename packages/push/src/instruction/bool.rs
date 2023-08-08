use super::{Error, Instruction, InstructionResult, PushInstruction, PushInstructionError};
use crate::push_vm::stack::{HasStack, Stack, StackError};
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
    S: Clone + HasStack<bool> + HasStack<i64>,
{
    type Error = PushInstructionError;

    // TODO: This only "works" because all the stack operations are "transactional",
    //   i.e., things like `pop2()` either completely succeed or return an error without
    //   modifying the (mutable) state. (This is done by checking that the size of the
    //   relevant stack is big enough before removing any elements.) If any stack operations
    //   were _not_ "transactional" then we could end up passing an inconsistent state
    //   to the call to `Error::recoverable_error()`, which would be bad. Because the `pop`
    //   and `push` calls aren't together, we can still have inconsistent states in the
    //   call to `Error::fatal_error()`. For example, if the boolean is full and the
    //   instruction is `BoolFromInt`, we could pop off an integer before we realize there's
    //   no room to push on the new boolean. We can special case that, but the burden lies
    //   on the programmer, with no help from the type system.
    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        // let mut original_state = state.clone();
        let bool_stack: &mut Stack<bool> = state.stack_mut();
        let result = match self {
            Self::Push(b) => Ok(*b),
            Self::BoolNot => bool_stack.pop().map(Not::not),
            Self::BoolAnd => bool_stack.pop2().map(|(x, y)| x && y),
            Self::BoolOr => bool_stack.pop2().map(|(x, y)| x || y),
            Self::BoolXor => bool_stack.pop2().map(|(x, y)| x != y),
            Self::BoolImplies => bool_stack.pop2().map(|(x, y)| !x || y),
            Self::BoolFromInt => {
                if HasStack::<bool>::stack(&state).is_full() {
                    return Err(Error::fatal_error(
                        state,
                        StackError::Overflow { stack_type: "bool" },
                    ));
                }
                let int_stack: &mut Stack<i64> = state.stack_mut();
                int_stack.pop().map(|i| i != 0)
            }
        };
        match result {
            Err(error) => Err(Error::recoverable_error(state, error)),
            Ok(b) => {
                let bool_stack: &mut Stack<bool> = state.stack_mut();
                let push_result = bool_stack.push(b);
                match push_result {
                    Err(error) => Err(Error::fatal_error(state, error)),
                    Ok(_) => Ok(state),
                }
            }
        }
        // let b = result.map_err(|error| Error::recoverable_error(original_state, error))?;
        // let bool_stack: &mut Stack<bool> = state.stack_mut();
        // bool_stack
        //     .push(b)
        //     .map_err(|error| Error::fatal_error(original_state, error))?;
        // Ok(state)
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
        push_vm::{push_state::PushState, stack::HasStack},
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
            let state = PushState::builder([])
                .with_bool_values(vec![x, y])
                .with_int_values(vec![i])
                .build();
            let _ = instr.perform(state).unwrap();
        }

        #[test]
        fn and_is_correct(x in proptest::bool::ANY, y in proptest::bool::ANY) {
            let state = PushState::builder([])
                .with_bool_values(vec![x, y])
                .build();
            let result_state = BoolInstruction::BoolAnd.perform(state).unwrap();
            prop_assert_eq!(result_state.bool.size(), 1);
            prop_assert_eq!(*result_state.bool.top().unwrap(), x && y);
        }

        #[test]
        fn implies_is_correct(x in proptest::bool::ANY, y in proptest::bool::ANY) {
            let state = PushState::builder([])
                .with_bool_values(vec![x, y])
                .build();
            let result_state = BoolInstruction::BoolImplies.perform(state).unwrap();
            prop_assert_eq!(result_state.bool.size(), 1);
            prop_assert_eq!(*result_state.bool.top().unwrap(), !x || y);
        }
    }
}
