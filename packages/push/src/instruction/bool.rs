use super::{Instruction, PushInstruction};
use crate::{push_vm::push_state::PushState, util::pop2};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum BoolInstruction {
    Push(bool),
    BoolOr,
    BoolAnd,
}

impl Instruction<PushState> for BoolInstruction {
    fn perform(&self, state: &mut PushState) {
        match self {
            Self::Push(b) => state.bool.push(*b),
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
        }
    }
}

impl From<BoolInstruction> for PushInstruction {
    fn from(instr: BoolInstruction) -> Self {
        Self::BoolInstruction(instr)
    }
}
