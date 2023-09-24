use super::{Instruction, InstructionResult, PushInstruction, PushInstructionError};
use crate::{
    error::stateful::SpecifySeverity,
    push_vm::{
        stack::traits::{
            discard::DiscardHead,
            discard::DiscardHeadIn,
            get::{GetHead, GetHeadIn},
            has_stack::{HasStack, HasStackMut},
            push::{AttemptPushHead, PushHead, PushHeadIn},
            size::{SizeLimit, SizeLimitOf, StackSize},
            TypedStack,
        },
        state::with_state::{AddState, WithStateOps},
        PushInteger,
    },
};
use std::ops::Neg;
use strum_macros::EnumIter;

#[derive(Debug, strum_macros::Display, Copy, Clone, PartialEq, Eq, EnumIter)]
pub enum IntInstruction {
    Push(PushInteger),

    Negate,
    Abs,
    Min,
    Max,
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
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    FromBoolean,
}

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum IntInstructionError {
    #[error("Integer arithmetic overflow for instruction {op}")]
    Overflow {
        op: IntInstruction,
        // I liked the idea of keeping track of the arguments to the instruction
        // that led to the overflow, but that complicated `.perform()` in a variety
        // of ways so I'm removing that for now.
        // args: Vec<PushInteger>,
    },
}

impl<S> Instruction<S> for IntInstruction
where
    S: Clone + HasStackMut<PushInteger> + HasStackMut<bool>,
    <S as HasStack<PushInteger>>::StackType:
        TypedStack<Item = PushInteger> + PushHead + GetHead + DiscardHead,
    <S as HasStack<bool>>::StackType: TypedStack<Item = bool> + StackSize + SizeLimit + PushHead,
{
    type Error = PushInstructionError;

    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    #[allow(unreachable_code, clippy::let_unit_value)] // Remove this
    fn perform<'a>(&'a self, state: &'a mut S) -> InstructionResult<&'a mut S, Self::Error> {
        match self {
            Self::Push(_)
            | Self::Negate
            | Self::Abs
            | Self::Inc
            | Self::Dec
            | Self::Square
            | Self::Add
            | Self::Subtract
            | Self::Multiply
            | Self::ProtectedDivide
            | Self::Mod
            | Self::Power
            | Self::Min
            | Self::Max => {
                // All these instructions pop at least one value from the integer stack, so we're
                // guaranteed that there will be space for the result. So we don't have to check that
                // any stacks are full before we start.
                let int_stack = state.stack_mut::<PushInteger>();
                match self {
                    Self::Push(i) => state.push_head_in::<PushInteger>(*i).make_recoverable()?,
                    Self::Negate => int_stack
                        .head()
                        .map(Neg::neg)
                        .with_state(state)
                        .make_recoverable()?
                        .discard_head_in::<PushInteger>()
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,
                    Self::Abs => int_stack
                        .head()
                        .copied()
                        .map(i64::abs)
                        .with_state(state)
                        .make_recoverable()?
                        .discard_head_in::<PushInteger>()
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    // This works, but is going to be nasty after we repeat a lot. There should
                    // perhaps be another trait method somewhere that eliminates a lot of this
                    // boilerplate.
                    Self::Inc => int_stack
                        .head()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|i| i.checked_add(1))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .with_state(state)
                        .make_recoverable()?
                        .discard_head_in::<PushInteger>()
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    Self::Dec => int_stack
                        .head()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|i| i.checked_sub(1))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .with_state(state)
                        .make_recoverable()?
                        .discard_head_in::<PushInteger>()
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    Self::Square => int_stack
                        .head()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|x| (*x).checked_mul(*x))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .with_state(state)
                        .make_recoverable()?
                        .discard_head_in::<PushInteger>()
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    Self::Add => int_stack
                        .get_n_head()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(x, y)| (*x).checked_add(*y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .with_state(state)
                        .make_recoverable()?
                        .discard_n_head_in::<PushInteger>(2)
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    Self::Subtract => int_stack
                        .get_n_head()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(x, y)| (*x).checked_sub(*y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .with_state(state)
                        .make_recoverable()?
                        .discard_n_head_in::<PushInteger>(2)
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    Self::Multiply => int_stack
                        .get_n_head()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(x, y)| (*x).checked_mul(*y))
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .with_state(state)
                        .make_recoverable()?
                        .discard_n_head_in::<PushInteger>(2)
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    Self::ProtectedDivide => int_stack
                        .get_n_head()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(x, y)| {
                            if *y == 0 {
                                Some(1)
                            } else {
                                (*x).checked_div(*y)
                            }
                        })
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .with_state(state)
                        .make_recoverable()?
                        .discard_n_head_in::<PushInteger>(2)
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    Self::Mod => int_stack
                        .get_n_head()
                        .map_err(Into::<PushInstructionError>::into)
                        .map(|(x, y)| {
                            if *y == 0 {
                                Some(0)
                            } else {
                                (*x).checked_rem(*y)
                            }
                        })
                        .and_then(|v| {
                            v.ok_or(IntInstructionError::Overflow { op: *self })
                                .map_err(Into::into)
                        })
                        .with_state(state)
                        .make_recoverable()?
                        .discard_n_head_in::<PushInteger>(2)
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_fatal()?,

                    // Self::Power => int_stack
                    //     .pop2()
                    //     .map_err(Into::into)
                    //     .map(|(x, y)| u32::try_from(y).map_or(Some(0), |y| x.checked_pow(y)))
                    //     .and_then(|v| {
                    //         v.ok_or(IntInstructionError::Overflow { op: *self })
                    //             .map_err(Into::into)
                    //     }),

                    // Self::Min => int_stack.pop2().map_err(Into::into).map(|(x, y)| x.min(y)),
                    // Self::Max => int_stack.pop2().map_err(Into::into).map(|(x, y)| x.max(y)),
                    _ => {
                        unreachable!("We failed to handle an arithmetic Int instruction: {self:?}")
                    }
                };
            }
            Self::IsEven
            | Self::IsOdd
            | Self::Equal
            | Self::NotEqual
            | Self::LessThan
            | Self::LessThanEqual
            | Self::GreaterThan
            | Self::GreaterThanEqual => {
                // None of these instructions pop anything off the boolean stack, but
                // they will push a result onto that stack. Thus before we start performing
                // the instruction, we need to check for the case that the boolean stack is
                // already full, and return an `Overflow` error if it is.
                {
                    state.not_full_in::<bool>().make_recoverable()?;
                }
                match self {
                    // TODO: Write a test for IsEven that makes sure all the stack manipulation is correct.
                    Self::IsEven => state
                        .head_in::<PushInteger>()
                        .map_value(|x| x % 2 == 0)
                        .make_recoverable()?
                        .attempt_push_head()
                        .make_recoverable()?
                        .discard_head_in::<PushInteger>()
                        .make_fatal()?,
                    //     Self::IsOdd => int_stack.pop().map_err(Into::into).map(|x| x % 2 != 0),
                    //     Self::Equal => int_stack.pop2().map_err(Into::into).map(|(x, y)| x == y),
                    //     Self::NotEqual => int_stack.pop2().map_err(Into::into).map(|(x, y)| x != y),
                    //     Self::LessThan => int_stack.pop2().map_err(Into::into).map(|(x, y)| x < y),
                    //     Self::LessThanEqual => {
                    //         int_stack.pop2().map_err(Into::into).map(|(x, y)| x <= y)
                    //     }
                    //     Self::GreaterThan => int_stack.pop2().map_err(Into::into).map(|(x, y)| x > y),
                    //     Self::GreaterThanEqual => {
                    //         int_stack.pop2().map_err(Into::into).map(|(x, y)| x >= y)
                    //     }
                    _ => unreachable!(
                        "We failed to implement a boolean-valued operation on integers: {self:?}"
                    ),
                };
                // match result {
                //     Err::<_, PushInstructionError>(error) => {
                //         // error.make_recoverable(state)
                //         Err(Error::recoverable(state, error))
                //     }
                //     Ok(b) => {
                //         let bool_stack: &mut Stack<bool> = state.stack_mut();
                //         let push_result = bool_stack.push(b);
                //         match push_result {
                //             Err(error) => Err(Error::fatal(state, error)), // error.make_fatal(state)
                //             Ok(_) => Ok(state),
                //         }
                //     }
                // }
                // .map_err(|error: PushInstructionError| Error::recoverable_error(state, error))?;
                // We know there's room on the boolean stack for the result because we confirmed
                // it wasn't full before we started performing any instructions in this group.
                // state.stack_mut().push(result);
                // todo!()
            }
            Self::FromBoolean => todo!(), // Self::FromBoolean => todo!(),
        }
        // match self {
        //     Self::Push(i) => {
        //         // TODO: We might want `push` to be able to fail if, e.g., the size of the
        //         //   resulting stack exceeded some specified max stack depth.
        //         int_stack.push(*i);
        //     }
        //     Self::Negate => {
        //         if let Some(i) = int_stack.pop() {
        //             int_stack.push(-i);
        //         }
        //     }
        //     Self::Abs => {
        //         if let Some(i) = int_stack.pop() {
        //             int_stack.push(i.abs());
        //         }
        //     }
        //     Self::Inc => {
        //         if let Some(x) = int_stack.pop() {
        //             if let Some(result) = x.checked_add(1) {
        //                 int_stack.push(result);
        //             } else {
        //                 // Do nothing, i.e., put the arguments back
        //                 int_stack.push(x);
        //                 return Err(Error::recoverable_error(
        //                     state,
        //                     IntInstructionError::Overflow,
        //                 ));
        //             }
        //         }
        //     }
        //     Self::Dec => {
        //         if let Some(x) = int_stack.pop() {
        //             if let Some(result) = x.checked_sub(1) {
        //                 int_stack.push(result);
        //             } else {
        //                 // Do nothing, i.e., put the arguments back
        //                 int_stack.push(x);
        //                 return Err(Error::recoverable_error(
        //                     state,
        //                     IntInstructionError::Overflow,
        //                 ));
        //             }
        //         }
        //     }
        //     Self::Square => {
        //         if let Some(x) = int_stack.pop() {
        //             if let Some(result) = x.checked_mul(x) {
        //                 int_stack.push(result);
        //             } else {
        //                 // Do nothing, i.e., put the arguments back
        //                 int_stack.push(x);
        //                 return Err(Error::recoverable_error(
        //                     state,
        //                     IntInstructionError::Overflow,
        //                 ));
        //             }
        //         }
        //     }
        //     Self::Add => {
        //         // TODO: We should probably check that this addition succeeds and do something
        //         //   sensible if it doesn't. That requires having these return a `Result` or
        //         //   `Option`, however, which we don't yet do.
        //         if let Some((x, y)) = int_stack.pop2() {
        //             // We quietly ignore it if this returns `None`.
        //             if let Some(result) = x.checked_add(y) {
        //                 // What's the right thing here?
        //                 int_stack.push(result);
        //             } else {
        //                 // Do nothing, i.e., put the arguments back
        //                 int_stack.push(y);
        //                 int_stack.push(x);
        //                 return Err(Error::recoverable_error(
        //                     state,
        //                     IntInstructionError::Overflow,
        //                 ));
        //             }
        //         }
        //     }
        //     Self::Subtract => {
        //         // TODO: We should probably check that this addition succeeds and do something
        //         //   sensible if it doesn't. That requires having these return a `Result` or
        //         //   `Option`, however, which we don't yet do.
        //         if let Some((x, y)) = int_stack.pop2() {
        //             if let Some(result) = x.checked_sub(y) {
        //                 int_stack.push(result);
        //             } else {
        //                 // Do nothing, i.e., put the arguments back
        //                 int_stack.push(y);
        //                 int_stack.push(x);
        //                 return Err(Error::recoverable_error(
        //                     state,
        //                     IntInstructionError::Overflow,
        //                 ));
        //             }
        //         }
        //     }
        //     Self::Multiply => {
        //         // TODO: We should probably check that this addition succeeds and do something
        //         //   sensible if it doesn't. That requires having these return a `Result` or
        //         //   `Option`, however, which we don't yet do.
        //         if let Some((x, y)) = int_stack.pop2() {
        //             if let Some(result) = x.checked_mul(y) {
        //                 int_stack.push(result);
        //             } else {
        //                 // Do nothing, i.e., put the arguments back
        //                 int_stack.push(y);
        //                 int_stack.push(x);
        //                 return Err(Error::recoverable_error(
        //                     state,
        //                     IntInstructionError::Overflow,
        //                 ));
        //             }
        //         }
        //     }
        //     Self::ProtectedDivide => {
        //         // TODO: We should probably check that this addition succeeds and do something
        //         //   sensible if it doesn't. That requires having these return a `Result` or
        //         //   `Option`, however, which we don't yet do.
        //         if let Some((x, y)) = int_stack.pop2() {
        //             if y == 0 {
        //                 int_stack.push(1);
        //             } else {
        //                 int_stack.push(x / y);
        //             }
        //         }
        //     }
        //     Self::Mod => {
        //         if let Some((x, y)) = int_stack.pop2() {
        //             if y == 0 {
        //                 // Do nothing, i.e., put the values back
        //                 int_stack.push(y);
        //                 int_stack.push(x);
        //             } else {
        //                 int_stack.push(x % y);
        //             }
        //         }
        //     }
        //     // TODO: I'm not convinced that Clojush handles negative y correctly.
        //     // TODO: I assume that this blows up for large values of y and I'm not sure
        //     //   what actually happens. There's a `checked_pow` function that might be
        //     //   the preferable choice?
        //     Self::Power => {
        //         if let Some((x, y)) = int_stack.pop2() {
        //             if let Ok(y) = u32::try_from(y) {
        //                 int_stack.push(x.pow(y));
        //             } else {
        //                 // Do nothing, i.e., put the values back
        //                 int_stack.push(y);
        //                 int_stack.push(x);
        //             }
        //         }
        //     }
        //     Self::IsEven => {
        //         if let Some(i) = int_stack.pop() {
        //             state.stack_mut().push(i % 2 == 0);
        //         }
        //     }
        //     Self::IsOdd => {
        //         if let Some(i) = int_stack.pop() {
        //             state.stack_mut().push(i % 2 != 0);
        //         }
        //     }
        //     Self::LessThan => {
        //         if let Some((x, y)) = int_stack.pop2() {
        //             state.stack_mut().push(x < y);
        //         }
        //     }
        //     Self::LessThanEqual => {
        //         if let Some((x, y)) = int_stack.pop2() {
        //             state.stack_mut().push(x <= y);
        //         }
        //     }
        //     Self::GreaterThan => {
        //         if let Some((x, y)) = int_stack.pop2() {
        //             state.stack_mut().push(x > y);
        //         }
        //     }
        //     Self::GreaterThanEqual => {
        //         if let Some((x, y)) = int_stack.pop2() {
        //             state.stack_mut().push(x >= y);
        //         }
        //     }
        //     Self::FromBoolean => {
        //         let bool_stack: &mut Stack<bool> = state.stack_mut();
        //         if let Some(b) = bool_stack.pop() {
        //             let int_stack: &mut Stack<PushInteger> = state.stack_mut();
        //             if b {
        //                 int_stack.push(1);
        //             } else {
        //                 int_stack.push(0);
        //             }
        //         }
        //     }
        //     Self::Min => {
        //         if let Some((x, y)) = int_stack.pop2() {
        //             int_stack.push(x.min(y));
        //         }
        //     }
        //     Self::Max => {
        //         if let Some((x, y)) = int_stack.pop2() {
        //             int_stack.push(x.max(y));
        //         }
        //     }
        // }
        // Ok(state)
        Ok(())
    }
}

impl From<IntInstruction> for PushInstruction {
    fn from(instr: IntInstruction) -> Self {
        Self::IntInstruction(instr)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {

    use crate::push_vm::{
        stack::traits::{get::GetHeadIn, size::StackSizeOf},
        state::{with_state::WithStateOps, PushState},
    };

    use super::*;

    #[test]
    fn add() {
        let x = 409;
        let y = 512;
        let mut state = PushState::builder([]).build();
        state.push_n_head_in::<PushInteger, _>((y, x)).unwrap();
        IntInstruction::Add.perform(&mut state).unwrap();
        assert_eq!(state.size_of::<PushInteger>().drop_state(), 1);
        assert_eq!(*state.head_in::<PushInteger>().drop_state().unwrap(), x + y);
    }

    #[test]
    fn add_overflows() {
        let x = 4_098_586_571_925_584_936;
        let y = 5_124_785_464_929_190_872;
        let mut state = PushState::builder([]).build();
        state.push_n_head_in::<PushInteger, _>((y, x)).unwrap();
        let err = IntInstruction::Add.perform(&mut state).unwrap_err();
        assert_eq!((*err.state()).size_of::<PushInteger>().drop_state(), 2);
        assert_eq!(
            err.error(),
            &PushInstructionError::from(IntInstructionError::Overflow {
                op: IntInstruction::Add
            })
        );
        assert!(err.is_recoverable());
    }

    #[test]
    fn inc_overflows() {
        let x = PushInteger::MAX;
        let mut state = PushState::builder([]).build();
        state.push_head_in::<PushInteger>(x).unwrap();
        let err = IntInstruction::Inc.perform(&mut state).unwrap_err();
        assert_eq!((*err.state()).size_of::<PushInteger>().drop_state(), 1);
        assert_eq!(
            (*err.state())
                .head_in::<PushInteger>()
                .drop_state()
                .unwrap(),
            &PushInteger::MAX
        );
        assert_eq!(
            err.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::Inc
            }
            .into()
        );
        assert!(err.is_recoverable());
    }

    #[test]
    fn dec_overflows() {
        let x = PushInteger::MIN;
        let mut state = PushState::builder([]).build();
        state.push_head_in::<PushInteger>(x).unwrap();
        let err = IntInstruction::Dec.perform(&mut state).unwrap_err();
        assert_eq!((*err.state()).size_of::<PushInteger>().drop_state(), 1);
        assert_eq!(
            (*err.state())
                .head_in::<PushInteger>()
                .drop_state()
                .unwrap(),
            &PushInteger::MIN
        );
        assert_eq!(
            err.error(),
            &IntInstructionError::Overflow {
                op: IntInstruction::Dec
            }
            .into()
        );
        assert!(err.is_recoverable());
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod property_tests {
    use crate::{
        instruction::{int::IntInstructionError, Instruction, IntInstruction},
        push_vm::{
            stack::traits::{get::GetHeadIn, push::PushHeadIn, size::StackSizeOf},
            state::{with_state::WithStateOps, PushState},
            PushInteger,
        },
    };
    use proptest::{prop_assert_eq, proptest};
    use strum::IntoEnumIterator;

    fn all_instructions() -> Vec<IntInstruction> {
        IntInstruction::iter().collect()
    }

    proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(1_000))]

        #[test]
        fn negate(x in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_head_in::<PushInteger>(x).unwrap();
            IntInstruction::Negate.perform(&mut state).unwrap();
            prop_assert_eq!(state.size_of::<PushInteger>().drop_state(), 1);
            prop_assert_eq!(*state.head_in::<PushInteger>().drop_state().unwrap(), -x);
        }

        #[test]
        fn abs(x in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_head_in::<PushInteger>(x).unwrap();
            IntInstruction::Abs.perform(&mut state).unwrap();
            prop_assert_eq!(state.size_of::<PushInteger>().drop_state(), 1);
            prop_assert_eq!(*state.head_in::<PushInteger>().drop_state().unwrap(), x.abs());
        }

        #[test]
        fn sqr(x in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_head_in::<PushInteger>(x).unwrap();
            let err = IntInstruction::Square.perform(&mut state);
            if let Some(x_squared) = x.checked_mul(x) {
                prop_assert_eq!(state.size_of::<PushInteger>().drop_state(), 1);
                let output = *state.head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(output, x_squared);
            } else {
                let err = err.unwrap_err();
                assert_eq!(
                    err.error(),
                    &IntInstructionError::Overflow {
                        op: IntInstruction::Square
                    }.into()
                );
                assert!(err.is_recoverable());
                let top_int = (*err.state()).head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(*top_int, x);
            }
        }

        #[test]
        fn add_doesnt_crash(x in proptest::num::i64::ANY, y in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((y,x)).unwrap();
            IntInstruction::Add.perform(&mut state).unwrap();
        }

        #[test]
        fn add_adds_or_does_nothing(x in proptest::num::i64::ANY, y in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((y,x)).unwrap();
            let err = IntInstruction::Add.perform(&mut state);
            #[allow(clippy::unwrap_used)]
            if let Some(expected_result) = x.checked_add(y) {
                let output = state.head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(output, &expected_result);
            } else {
                // This only checks that `x` is still on the top of the stack.
                // We arguably want to confirm that the entire state of the system
                // is unchanged, except that the `Add` instruction has been
                // removed from the `exec` stack.
                let err = err.unwrap_err();
                assert_eq!(
                    err.error(),
                    &IntInstructionError::Overflow {
                        op: IntInstruction::Add
                    }
                    .into()
                );
                assert!(err.is_recoverable());
                let top_int = (*err.state()).head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(*top_int, x);
            }
        }

        #[test]
        fn subtract_subs_or_does_nothing(x in proptest::num::i64::ANY, y in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((y,x)).unwrap();
            let err = IntInstruction::Subtract.perform(&mut state);
            #[allow(clippy::unwrap_used)]
            if let Some(expected_result) = x.checked_sub(y) {
                let output = state.head_in::<PushInteger>().unwrap().drop_state();
                prop_assert_eq!(output, &expected_result);
            } else {
                // This only checks that `x` is still on the top of the stack.
                // We arguably want to confirm that the entire state of the system
                // is unchanged, except that the `Add` instruction has been
                // removed from the `exec` stack.
                let err = err.unwrap_err();
                assert_eq!(
                    err.error(),
                    &IntInstructionError::Overflow {
                        op: IntInstruction::Subtract
                    }
                    .into()
                );
                assert!(err.is_recoverable());
                let top_int = (*err.state()).head_in::<PushInteger>().unwrap().drop_state();
                prop_assert_eq!(*top_int, x);
            }
        }

        #[test]
        fn multiply_muls_or_does_nothing(x in proptest::num::i64::ANY, y in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((y,x)).unwrap();
            let err = IntInstruction::Multiply.perform(&mut state);
            #[allow(clippy::unwrap_used)]
            if let Some(expected_result) = x.checked_mul(y) {
                let output = state.head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(output, &expected_result);
            } else {
                // This only checks that `x` is still on the top of the stack.
                // We arguably want to confirm that the entire state of the system
                // is unchanged, except that the `Add` instruction has been
                // removed from the `exec` stack.
                let err = err.unwrap_err();
                assert_eq!(
                    err.error(),
                    &IntInstructionError::Overflow {
                        op: IntInstruction::Multiply
                    }
                    .into()
                );
                assert!(err.is_recoverable());
                let top_int = (*err.state()).head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(*top_int, x);
            }
        }

        #[test]
        fn protected_divide_zero_denominator(x in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((0,x)).unwrap();
            IntInstruction::ProtectedDivide.perform(&mut state).unwrap();
            let output = state.head_in::<PushInteger>().drop_state().unwrap();
            // Dividing by zero should always return 1.
            prop_assert_eq!(output, &1);
        }

        #[test]
        fn protected_divide_divs_or_does_nothing(x in proptest::num::i64::ANY, y in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((y,x)).unwrap();
            let err = IntInstruction::ProtectedDivide.perform(&mut state);
            #[allow(clippy::unwrap_used)]
            if let Some(expected_result) = x.checked_div(y) {
                let output = state.head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(output, &expected_result);
            } else {
                // This only checks that `x` is still on the top of the stack.
                // We arguably want to confirm that the entire state of the system
                // is unchanged, except that the `Add` instruction has been
                // removed from the `exec` stack.
                let err = err.unwrap_err();
                assert_eq!(
                    err.error(),
                    &IntInstructionError::Overflow {
                        op: IntInstruction::ProtectedDivide
                    }
                    .into()
                );
                assert!(err.is_recoverable());
                let top_int = (*err.state()).head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(*top_int, x);
            }
        }

        #[test]
        fn mod_zero_denominator(x in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((0,x)).unwrap();
            IntInstruction::Mod.perform(&mut state).unwrap();
            #[allow(clippy::unwrap_used)]
            let output = state.head_in::<PushInteger>().drop_state().unwrap();
            // Modding by zero should always return 0 since x % x = 0 for all x != 0.
            prop_assert_eq!(output, &0);
        }

        #[test]
        fn mod_rems_or_does_nothing(x in proptest::num::i64::ANY, y in proptest::num::i64::ANY) {
            let mut state = PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((y,x)).unwrap();
            let err = IntInstruction::Mod.perform(&mut state);
            #[allow(clippy::unwrap_used)]
            if let Some(expected_result) = x.checked_rem(y) {
                let output = state.head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(output, &expected_result);
            } else if y == 0 {
                let output = state.head_in::<PushInteger>().drop_state().unwrap();
                // Modding by zero should always return 0 since x % x == 0 for all x != 0.
                prop_assert_eq!(output, &0);
            } else {
                // This only checks that `x` is still on the top of the stack.
                // We arguably want to confirm that the entire state of the system
                // is unchanged, except that the `Add` instruction has been
                // removed from the `exec` stack.
                let err = err.unwrap_err();
                assert_eq!(
                    err.error(),
                    &IntInstructionError::Overflow {
                        op: IntInstruction::Mod
                    }
                    .into()
                );
                assert!(err.is_recoverable());
                let top_int = err.state().head_in::<PushInteger>().drop_state().unwrap();
                prop_assert_eq!(*top_int, x);
            }
        }

        #[test]
        fn inc_does_not_crash(x in proptest::num::i64::ANY) {
            let  state = &mut PushState::builder([]).build();
            state.push_head_in::<PushInteger>(x).unwrap();
            let _ = IntInstruction::Inc.perform(state);
        }

        #[test]
        #[ignore]
        fn int_ops_do_not_crash(instr in proptest::sample::select(all_instructions()), x in proptest::num::i64::ANY, y in proptest::num::i64::ANY, b in proptest::bool::ANY) {
            let state =&mut PushState::builder([]).build();
            state.push_n_head_in::<PushInteger,_>((y,x)).unwrap();
            state.push_head_in::<bool>(b).unwrap();
            let _ = instr.perform(state);
        }
    }
}
