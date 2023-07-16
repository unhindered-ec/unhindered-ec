use super::{Instruction, PushInstruction};
use crate::{push_vm::push_state::PushState, util::pop2};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum IntInstruction {
    Push(i64),
    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
    IsEven,
}

impl Instruction<PushState> for IntInstruction {
    fn perform(&self, state: &mut PushState) {
        match self {
            Self::Push(i) => state.int.push(*i),
            Self::Add => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.int.push(x + y);
                }
            }
            Self::Subtract => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.int.push(x - y);
                }
            }
            Self::Multiply => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.int.push(x * y);
                }
            }
            Self::ProtectedDivide => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = pop2(&mut state.int) {
                    if y == 0 {
                        state.int.push(1);
                    } else {
                        state.int.push(x / y);
                    }
                }
            }
            Self::IsEven => {
                if let Some(i) = state.int.pop() {
                    state.bool.push(i % 2 == 0);
                }
            }
        }
    }
}

impl From<IntInstruction> for PushInstruction {
    fn from(instr: IntInstruction) -> Self {
        Self::IntInstruction(instr)
    }
}
