use strum_macros::EnumIter;

use super::{Instruction, PushInstruction};
use crate::push_vm::push_state::{HasStack, PushState, Stack};

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
        let int_stack: &mut Stack<i64> = state.stack_mut();
        match self {
            Self::Push(i) => int_stack.push(*i),
            Self::Negate => {
                if let Some(i) = int_stack.pop() {
                    int_stack.push(-i);
                }
            }
            Self::Abs => {
                if let Some(i) = int_stack.pop() {
                    int_stack.push(i.abs());
                }
            }
            Self::Inc => {
                if let Some(x) = int_stack.pop() {
                    if let Some(result) = x.checked_add(1) {
                        int_stack.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        int_stack.push(x);
                    }
                }
            }
            Self::Dec => {
                if let Some(x) = int_stack.pop() {
                    if let Some(result) = x.checked_sub(1) {
                        int_stack.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        int_stack.push(x);
                    }
                }
            }
            Self::Square => {
                if let Some(x) = int_stack.pop() {
                    if let Some(result) = x.checked_mul(x) {
                        int_stack.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        int_stack.push(x);
                    }
                }
            }
            Self::Add => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = int_stack.pop2() {
                    if let Some(result) = x.checked_add(y) {
                        int_stack.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        int_stack.push(y);
                        int_stack.push(x);
                    }
                }
            }
            Self::Subtract => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = int_stack.pop2() {
                    if let Some(result) = x.checked_sub(y) {
                        int_stack.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        int_stack.push(y);
                        int_stack.push(x);
                    }
                }
            }
            Self::Multiply => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = int_stack.pop2() {
                    if let Some(result) = x.checked_mul(y) {
                        int_stack.push(result);
                    } else {
                        // Do nothing, i.e., put the arguments back
                        int_stack.push(y);
                        int_stack.push(x);
                    }
                }
            }
            Self::ProtectedDivide => {
                // TODO: We should probably check that this addition succeeds and do something
                //   sensible if it doesn't. That requires having these return a `Result` or
                //   `Option`, however, which we don't yet do.
                if let Some((x, y)) = int_stack.pop2() {
                    if y == 0 {
                        int_stack.push(1);
                    } else {
                        int_stack.push(x / y);
                    }
                }
            }
            Self::Mod => {
                if let Some((x, y)) = int_stack.pop2() {
                    if y == 0 {
                        // Do nothing, i.e., put the values back
                        int_stack.push(y);
                        int_stack.push(x);
                    } else {
                        int_stack.push(x % y);
                    }
                }
            }
            // TODO: I'm not convinced that Clojush handles negative y correctly.
            // TODO: I assume that this blows up for large values of y and I'm not sure
            //   what actually happens. There's a `checked_pow` function that might be
            //   the preferable choice?
            Self::Power => {
                if let Some((x, y)) = int_stack.pop2() {
                    if let Ok(y) = u32::try_from(y) {
                        int_stack.push(x.pow(y));
                    } else {
                        // Do nothing, i.e., put the values back
                        int_stack.push(y);
                        int_stack.push(x);
                    }
                }
            }
            Self::IsEven => {
                if let Some(i) = int_stack.pop() {
                    state.bool.push(i % 2 == 0);
                }
            }
            Self::IsOdd => {
                if let Some(i) = int_stack.pop() {
                    state.bool.push(i % 2 != 0);
                }
            }
            Self::LessThan => {
                if let Some((x, y)) = int_stack.pop2() {
                    state.bool.push(x < y);
                }
            }
            Self::LessThanEqual => {
                if let Some((x, y)) = int_stack.pop2() {
                    state.bool.push(x <= y);
                }
            }
            Self::GreaterThan => {
                if let Some((x, y)) = int_stack.pop2() {
                    state.bool.push(x > y);
                }
            }
            Self::GreaterThanEqual => {
                if let Some((x, y)) = int_stack.pop2() {
                    state.bool.push(x >= y);
                }
            }
            Self::FromBoolean => {
                let bool_stack: &mut Stack<bool> = state.stack_mut();
                if let Some(b) = bool_stack.pop() {
                    let int_stack: &mut Stack<i64> = state.stack_mut();
                    if b {
                        int_stack.push(1);
                    } else {
                        int_stack.push(0);
                    }
                }
            }
            Self::Min => {
                if let Some((x, y)) = int_stack.pop2() {
                    int_stack.push(x.min(y));
                }
            }
            Self::Max => {
                if let Some((x, y)) = int_stack.pop2() {
                    int_stack.push(x.max(y));
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

#[cfg(test)]
mod test {
    use crate::push_vm::push_state::Inputs;

    use super::*;

    #[test]
    fn add_overflows() {
        let x = 4_098_586_571_925_584_936;
        let y = 5_124_785_464_929_190_872;
        let mut state = PushState::builder([], &Inputs::default()).build();
        state.int.push(y);
        state.int.push(x);
        IntInstruction::Add.perform(&mut state);
    }

    #[test]
    fn inc_overflows() {
        let x = i64::MAX;
        let mut state = PushState::builder([], &Inputs::default()).build();
        state.int.push(x);
        IntInstruction::Inc.perform(&mut state);
    }

    #[test]
    fn dec_overflows() {
        let x = i64::MIN;
        let mut state = PushState::builder([], &Inputs::default()).build();
        state.int.push(x);
        IntInstruction::Dec.perform(&mut state);
    }
}

#[cfg(test)]
mod property_tests {
    use crate::{
        instruction::{Instruction, IntInstruction},
        push_vm::push_state::{Inputs, PushState},
    };
    use proptest::{prop_assert_eq, proptest};
    use strum::IntoEnumIterator;

    fn all_instructions() -> Vec<IntInstruction> {
        IntInstruction::iter().collect()
    }

    proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(1_000))]

        #[test]
        fn add_doesnt_crash(x in proptest::num::i64::ANY, y in proptest::num::i64::ANY) {
            let mut state = PushState::builder([], &Inputs::default()).build();
            state.int.push(y);
            state.int.push(x);
            IntInstruction::Add.perform(&mut state);
        }

        #[test]
        fn add_adds_or_does_nothing(x in proptest::num::i64::ANY, y in proptest::num::i64::ANY) {
            let mut state = PushState::builder([], &Inputs::default()).build();
            state.int.push(y);
            state.int.push(x);
            IntInstruction::Add.perform(&mut state);
            #[allow(clippy::unwrap_used)]
            let output = state.int.pop().unwrap();
            if let Some(result) = x.checked_add(y) {
                prop_assert_eq!(output, result);
            } else {
                // This only checks that `x` is still on the top of the stack.
                // We arguably want to confirm that the entire state of the system
                // is unchanged, except that the `Add` instruction has been
                // removed from the `exec` stack.
                prop_assert_eq!(output, x);
            }
        }

        #[test]
        fn inc_dec_do_not_crash(x in proptest::num::i64::ANY) {
            let mut state = PushState::builder([], &Inputs::default()).build();
            state.int.push(x);
            IntInstruction::Inc.perform(&mut state);
        }

        #[test]
        fn int_ops_do_not_crash(instr in proptest::sample::select(all_instructions()), x in proptest::num::i64::ANY, y in proptest::num::i64::ANY, b in proptest::bool::ANY) {
            let mut state = PushState::builder([], &Inputs::default()).build();
            state.int.push(y);
            state.int.push(x);
            state.bool.push(b);
            instr.perform(&mut state);
        }
    }
}
