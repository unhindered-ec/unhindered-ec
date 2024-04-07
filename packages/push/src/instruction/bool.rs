use std::ops::Not;

use strum_macros::EnumIter;

use super::{Instruction, PushInstruction, PushInstructionError};
use crate::{
    error::{InstructionResult, MapInstructionError},
    push_vm::stack::{HasStack, PushOnto},
};

#[derive(Debug, strum_macros::Display, Clone, PartialEq, Eq, EnumIter)]
#[non_exhaustive]
pub enum BoolInstruction {
    Push(bool),
    Not,
    Or,
    And,
    Xor,
    Implies,
    // Do we really want either of these? Do they get used?
    // BooleanInvertFirstThenAnd,
    // BoolInvertSecondThenAnd,
    FromInt,
    // BoolFromFloat,
}

impl<S> Instruction<S> for BoolInstruction
where
    S: Clone + HasStack<bool> + HasStack<i64>,
{
    type Error = PushInstructionError;

    // TODO: This only "works" because all the stack operations are "transactional",
    // i.e., things like `pop2()` either completely succeed or return an error
    // without modifying the (mutable) state. (This is done by checking that
    // the size of the relevant stack is big enough before removing any
    // elements.) If any stack operations were _not_ "transactional" then we
    // could end up passing an inconsistent state to the call to
    // `Error::recoverable_error()`, which would be bad. Because the `pop` and
    // `push` calls aren't together, we can still have inconsistent states in the
    // call to `Error::fatal_error()`. For example, if the boolean is full and the
    // instruction is `BoolFromInt`, we could pop off an integer before we realize
    // there's no room to push on the new boolean. We can special case that,
    // but the burden lies on the programmer, with no help from the type
    // system.

    /*
    // Get the nth character from a string and push it on the char stack.
    //   - n comes from the int stack
    //   - string comes from the string stack
    //   - result goes on the char stack
    let transaction: Transaction<PushState> = state.transaction();

    // This version has the transaction be mutable.
    let string = transaction.try_pop<String>()?;
    // This version returns a "new" transaction.
    let (string, transaction) = transaction.try_pop<String>()?;

    let (index, transaction) = transaction.try_pop<i64>()?;
    let c = string.chars.nth(index)?;
    let transaction = transaction.try_push<char>(c)?;
    let new_state = transaction.close()?; // Can closing actually fail?
     */

    // [pop string] then [pop integer] contains a closure with a tuple of (string,
    // int)

    // state.transaction().pop::<String>().with_min_length(1)
    //     .and_pop::<Integer>().then_push::<Char>(|(s, i)| s.chars.nth(i))
    // state.transaction().pop::<String>().with_min_length(1)
    //     .and_pop::<Integer>().charAt().then_push::<Char>()
    // state.transaction().pop::<String>().with_min_length(1)
    //     .and_pop::<Integer>().map::<Char>(|(s, i)|
    // s.chars.nth(i)).then_push::<Char>() Then you wouldn't be able to chain on
    // that and query what you would push onto the stack so maybe not ideal.

    // Options:
    //   - Make operations reversible (undo/redo)
    //   - Hold operations in some kind of queue and apply the at the end when we
    //     know they'll all work

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        let bool_stack = state.stack_mut::<bool>();
        match self {
            Self::Push(b) => state.with_push(*b).map_err_into(),
            Self::Not => bool_stack.pop().map(Not::not).push_onto(state),
            Self::And => bool_stack.pop2().map(|(x, y)| x && y).push_onto(state),
            Self::Or => bool_stack.pop2().map(|(x, y)| x || y).push_onto(state),
            Self::Xor => bool_stack.pop2().map(|(x, y)| x != y).push_onto(state),
            Self::Implies => bool_stack.pop2().map(|(x, y)| !x || y).push_onto(state),
            Self::FromInt => {
                let mut state = state.not_full::<bool>().map_err_into()?;
                state
                    .stack_mut::<i64>()
                    .pop()
                    .map(|i| i != 0)
                    .push_onto(state)
            }
        }
    }
}

impl From<BoolInstruction> for PushInstruction {
    fn from(instr: BoolInstruction) -> Self {
        Self::BoolInstruction(instr)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::ignored_unit_patterns)]
mod property_tests {
    use proptest::{arbitrary::any, prop_assert_eq, proptest};
    use strum::IntoEnumIterator;

    use crate::{
        instruction::{BoolInstruction, Instruction},
        push_vm::push_state::PushState,
    };

    fn all_instructions() -> Vec<BoolInstruction> {
        BoolInstruction::iter().collect()
    }

    proptest! {
        #[test]
        fn ops_do_not_crash(instr in proptest::sample::select(all_instructions()),
                x in any::<bool>(), y in any::<bool>(), i in any::<i64>()) {
            let state = PushState::builder()
                .with_max_stack_size(1000)
                .with_no_program()
                .with_bool_values([x, y])
                .unwrap()
                .with_int_values([i])
                .unwrap()
                .build();
            instr.perform(state).unwrap();
        }

        #[test]
        fn and_is_correct(x in any::<bool>(), y in any::<bool>()) {
            let state = PushState::builder()
                .with_max_stack_size(1000)
                .with_no_program()
                .with_bool_values([x, y])
                .unwrap()
                .build();
            let result_state = BoolInstruction::And.perform(state).unwrap();
            prop_assert_eq!(result_state.bool.size(), 1);
            prop_assert_eq!(*result_state.bool.top().unwrap(), x && y);
        }

        #[test]
        fn implies_is_correct(x in any::<bool>(), y in any::<bool>()) {
            let state = PushState::builder()
                .with_max_stack_size(1000)
                .with_no_program()
                .with_bool_values([x, y])
                .unwrap()
                .build();
            let result_state = BoolInstruction::Implies.perform(state).unwrap();
            prop_assert_eq!(result_state.bool.size(), 1);
            prop_assert_eq!(*result_state.bool.top().unwrap(), !x || y);
        }
    }
}
