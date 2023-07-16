use super::Instruction;
use crate::push_vm::push_state::PushState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PushInstruction {
    InputVar(usize),
    BoolInstruction(BoolInstruction),
    IntInstruction(IntInstruction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolInstruction {
    Push(bool),
    BoolOr,
    BoolAnd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntInstruction {
    Push(i64),
    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
    IsEven,
}

// When this code was suggested (by MizardX@Twitch) they included the
// `inline(always)` annotation. Clippy is then fussy about this, because
// it's often overused by people who haven't done the testing
// necessary to figure out if it's actually needed. My guess is
// that it is actually a Good Thing, and that we should bring
// it back (with an `allow` annotation to make Clippy happy),
// but it would be good to have the testing to back it up.
// #[inline(always)]
fn pop2<T>(stack: &mut Vec<T>) -> Option<(T, T)> {
    if stack.len() >= 2 {
        let x = stack.pop()?;
        let y = stack.pop()?;
        Some((x, y))
    } else {
        None
    }
}

impl PushInstruction {
    #[must_use]
    pub fn push_bool(b: bool) -> Self {
        BoolInstruction::Push(b).into()
    }

    #[must_use]
    pub fn push_int(i: i64) -> Self {
        IntInstruction::Push(i).into()
    }
}

impl Instruction<PushState> for PushInstruction {
    fn perform(&self, state: &mut PushState) {
        match self {
            Self::InputVar(var_index) => state.push_input(*var_index),
            Self::BoolInstruction(i) => i.perform(state),
            Self::IntInstruction(i) => i.perform(state),
        }
    }
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
