use strum_macros::EnumIter;

use super::{Instruction, PushInstruction};
use crate::{push_vm::push_state::PushState, util::pop2};

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

impl Instruction<PushState> for BoolInstruction {
    fn perform(&self, state: &mut PushState) {
        match self {
            Self::Push(b) => state.bool.push(*b),
            Self::BoolNot => {
                if let Some(x) = state.bool.pop() {
                    state.bool.push(!x);
                }
            }
            Self::BoolAnd => {
                if let Some((x, y)) = pop2(&mut state.bool) {
                    state.bool.push(x && y);
                }
            }
            Self::BoolOr => {
                if let Some((x, y)) = pop2(&mut state.bool) {
                    state.bool.push(x || y);
                }
            }
            Self::BoolXor => {
                if let Some((x, y)) = pop2(&mut state.bool) {
                    state.bool.push(x != y);
                }
            }
            Self::BoolFromInt => {
                if let Some(x) = state.int.pop() {
                    state.bool.push(x != 0);
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

    proptest! {
        #[test]
        fn bool_ops_do_not_crash(instr in proptest::sample::select(all_instructions()), x in proptest::bool::ANY, y in proptest::bool::ANY) {
            let mut state = PushState::builder([], &Inputs::default()).build();
            state.bool.push(y);
            state.bool.push(x);
            instr.perform(&mut state);
        }
    }
}
