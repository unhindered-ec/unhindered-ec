use strum_macros::EnumIter;

use super::{Instruction, PushInstruction};
use crate::{
    push_vm::push_state::{PushStack, PushState, Stack},
    util::pop2,
};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
#[allow(clippy::module_name_repetitions)]
pub enum BoolInstruction {
    Push(bool),
    BoolNot,
    BoolOr,
    BoolAnd,
    BoolXor,
    // Do we really want either of these? Do they get used?
    // BooleanInvertFirstThenAnd,
    // BoolInvertSecondThenAnd,
    BoolFromInt,
    // BoolFromFloat,
}

// TODO: It would be nice to have a macro to generate these.
//   Something like:
//
//   #[derive(PushInstruction)]
//   fn bool_and(x: bool, y: bool) -> bool {
//      x && y
//   }
//
//   The argument types would tell us which stacks values
//   come from, and the return type tells us which stack
//   the result goes on.

impl<S> Instruction<S> for BoolInstruction
where
    S: PushStack<bool> + PushStack<i64>,
{
    fn perform(&self, state: &mut S) {
        let bool_stack: &mut Stack<bool> = state.stack_mut();
        match self {
            // let state = state as PushStack<bool>;
            Self::Push(b) => bool_stack.push(*b),
            Self::BoolNot => {
                if let Some::<bool>(x) = bool_stack.pop() {
                    bool_stack.push(!x);
                }
            }
            Self::BoolAnd => {
                if let Some::<(bool, bool)>((x, y)) = bool_stack.pop2() {
                    bool_stack.push(x && y);
                }
            }
            Self::BoolOr => {
                // if let Some((x, y)) = pop2(&mut state.bool) {
                //     state.bool.push(x || y);
                // }
                todo!()
            }
            Self::BoolXor => {
                // if let Some((x, y)) = pop2(&mut state.bool) {
                //     state.bool.push(x != y);
                // }
                todo!()
            }
            Self::BoolFromInt => {
                let int_stack: &mut Stack<i64> = state.stack_mut();
                if let Some::<i64>(x) = int_stack.pop() {
                    let bool_stack: &mut Stack<bool> = state.stack_mut();
                    bool_stack.push(x != 0);
                }
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
mod property_tests {
    use crate::{
        instruction::{BoolInstruction, Instruction},
        push_vm::push_state::{Inputs, PushState},
    };
    use proptest::proptest;
    use strum::IntoEnumIterator;

    fn all_instructions() -> Vec<BoolInstruction> {
        BoolInstruction::iter().collect()
    }

    // proptest! {
    //     #[test]
    //     fn bool_ops_do_not_crash(instr in proptest::sample::select(all_instructions()), x in proptest::bool::ANY, y in proptest::bool::ANY, i in proptest::num::i64::ANY) {
    //         let mut state = PushState::builder([], &Inputs::default()).build();
    //         state.bool.push(y);
    //         state.bool.push(x);
    //         state.int.push(i);
    //         instr.perform(&mut state);
    //     }
    // }
}
