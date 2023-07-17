use strum_macros::EnumIter;

use super::{Instruction, PushInstruction};
use crate::{push_vm::push_state::PushState, util::pop2};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
#[allow(clippy::module_name_repetitions)]
pub enum IntInstruction {
    Push(i64),
    Negate,
    Abs,
    Inc,
    Dec,
    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
    Mod,
    Power,
    Square,
    // We can't easily convert from i64 to f64, etc.,
    // so we might need to do the log(n) integer
    // implementation of sqrt.
    // Sqrt,
    IsEven,
    IsOdd,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    FromBoolean,
    Min,
    Max,
}

impl Instruction<PushState> for IntInstruction {
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    fn perform(&self, state: &mut PushState) {
        match self {
            Self::Push(i) => state.int.push(*i),
            Self::Negate => {
                if let Some(i) = state.int.pop() {
                    state.int.push(-i);
                }
            }
            Self::Abs => {
                if let Some(i) = state.int.pop() {
                    state.int.push(i.abs());
                }
            }
            Self::Inc => {
                if let Some(x) = state.int.pop() {
                    if let Some(result) = x.checked_add(1) {
                        state.int.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        state.int.push(x);
                    }
                }
            }
            Self::Dec => {
                if let Some(x) = state.int.pop() {
                    if let Some(result) = x.checked_sub(1) {
                        state.int.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        state.int.push(x);
                    }
                }
            }
            Self::Square => {
                if let Some(x) = state.int.pop() {
                    if let Some(result) = x.checked_mul(x) {
                        state.int.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        state.int.push(x);
                    }
                }
            }
            Self::Add => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = pop2(&mut state.int) {
                    if let Some(result) = x.checked_add(y) {
                        state.int.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        state.int.push(y);
                        state.int.push(x);
                    }
                }
            }
            Self::Subtract => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = pop2(&mut state.int) {
                    if let Some(result) = x.checked_sub(y) {
                        state.int.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        state.int.push(y);
                        state.int.push(x);
                    }
                }
            }
            Self::Multiply => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = pop2(&mut state.int) {
                    if let Some(result) = x.checked_mul(y) {
                        state.int.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        state.int.push(y);
                        state.int.push(x);
                    }
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
            Self::Mod => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    if y == 0 {
                        // Do nothing, i.e., put the values back
                        state.int.push(y);
                        state.int.push(x);
                    } else {
                        state.int.push(x % y);
                    }
                }
            }
            // TODO: I'm not convinced that Clojush handles negative y correctly.
            // TODO: I assume that this blows up for large values of y and I'm not sure
            //   what actually happens. There's a `checked_pow` function that might be
            //   the preferable choice?
            Self::Power => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    if let Ok(y) = u32::try_from(y) {
                        state.int.push(x.pow(y));
                    } else {
                        // Do nothing, i.e., put the values back
                        state.int.push(y);
                        state.int.push(x);
                    }
                }
            }
            Self::IsEven => {
                if let Some(i) = state.int.pop() {
                    state.bool.push(i % 2 == 0);
                }
            }
            Self::IsOdd => {
                if let Some(i) = state.int.pop() {
                    state.bool.push(i % 2 != 0);
                }
            }
            Self::LessThan => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.bool.push(x < y);
                }
            }
            Self::LessThanEqual => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.bool.push(x <= y);
                }
            }
            Self::GreaterThan => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.bool.push(x > y);
                }
            }
            Self::GreaterThanEqual => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.bool.push(x >= y);
                }
            }
            Self::FromBoolean => {
                if let Some(b) = state.bool.pop() {
                    if b {
                        state.int.push(1);
                    } else {
                        state.int.push(0);
                    }
                }
            }
            Self::Min => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.int.push(x.min(y));
                }
            }
            Self::Max => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    state.int.push(x.max(y));
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
