use ordered_float::OrderedFloat;
use strum_macros::EnumIter;

use super::{Instruction, PushInstruction, PushInstructionError};
use crate::{
    error::{Error, InstructionResult, MapInstructionError},
    push_vm::{
        stack::{Stack, StackDiscard, StackError, StackPush},
        HasStack,
    },
};

#[derive(Debug, strum_macros::Display, Copy, Clone, EnumIter, Eq, PartialEq)]
#[non_exhaustive]
pub enum FloatInstruction {
    Push(OrderedFloat<f64>),
    Add,
    Subtract,
    Multiply,
    ProtectedDivide,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Dup,
}

impl From<FloatInstruction> for PushInstruction {
    fn from(instr: FloatInstruction) -> Self {
        Self::FloatInstruction(instr)
    }
}

impl<S> Instruction<S> for FloatInstruction
where
    S: Clone + HasStack<OrderedFloat<f64>> + HasStack<bool>,
{
    type Error = PushInstructionError;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        match self {
            Self::Push(f) => state.with_push(*f).map_err_into(),

            // All these instructions pop at least one value from the float stack, so we're
            // guaranteed that there will be space for the result. So we don't have to check that
            // any stacks are full before we start.
            Self::Add => Self::binary_arithmetic(state, std::ops::Add::add),
            Self::Subtract => Self::binary_arithmetic(state, std::ops::Sub::sub),
            Self::Multiply => Self::binary_arithmetic(state, std::ops::Mul::mul),
            Self::ProtectedDivide => Self::binary_arithmetic(state, |x, y| {
                #[allow(clippy::arithmetic_side_effects)]
                if y == 0.0 { OrderedFloat(1.0) } else { x / y }
            }),

            // None of these instructions pop anything off the boolean stack, but
            // they will push a result onto that stack. Thus before we start performing
            // the instruction, we need to check for the case that the boolean stack is
            // already full, and return an `Overflow` error if it is.
            Self::Equal => Self::binary_predicate(state, std::cmp::PartialEq::eq),
            Self::NotEqual => Self::binary_predicate(state, std::cmp::PartialEq::ne),
            Self::GreaterThan => Self::binary_predicate(state, std::cmp::PartialOrd::gt),
            Self::LessThan => Self::binary_predicate(state, std::cmp::PartialOrd::lt),
            Self::GreaterThanOrEqual => Self::binary_predicate(state, std::cmp::PartialOrd::ge),
            Self::LessThanOrEqual => Self::binary_predicate(state, std::cmp::PartialOrd::le),

            Self::Dup => {
                if state.stack::<OrderedFloat<f64>>().is_full() {
                    return Err(Error::fatal(
                        state,
                        StackError::Overflow {
                            stack_type: "float",
                        },
                    ));
                }
                let float_stack: &mut Stack<OrderedFloat<f64>> =
                    state.stack_mut::<OrderedFloat<f64>>();
                float_stack
                    .top()
                    .map_err(PushInstructionError::from)
                    .cloned()
                    .with_stack_push(state)
            }
        }
    }
}

impl FloatInstruction {
    fn binary_arithmetic<S>(
        mut state: S,
        op: impl FnOnce(OrderedFloat<f64>, OrderedFloat<f64>) -> OrderedFloat<f64>,
    ) -> Result<S, Error<S, PushInstructionError>>
    where
        S: Clone + HasStack<OrderedFloat<f64>>,
    {
        let float_stack = state.stack_mut::<OrderedFloat<f64>>();
        float_stack
            .top2()
            .map_err(PushInstructionError::from)
            .map(|(&x, &y)| op(x, y))
            .with_stack_replace(2, state)
    }

    fn binary_predicate<S>(
        mut state: S,
        op: impl FnOnce(&OrderedFloat<f64>, &OrderedFloat<f64>) -> bool,
    ) -> Result<S, Error<S, PushInstructionError>>
    where
        S: Clone + HasStack<OrderedFloat<f64>> + HasStack<bool>,
    {
        if state.stack::<bool>().is_full() {
            return Err(Error::fatal(
                state,
                StackError::Overflow { stack_type: "bool" },
            ));
        }
        let float_stack: &mut Stack<OrderedFloat<f64>> = state.stack_mut::<OrderedFloat<f64>>();
        float_stack
            .top2()
            .map_err(PushInstructionError::from)
            .map(|(x, y)| op(x, y))
            .with_stack_push(state)
            .with_stack_discard::<OrderedFloat<f64>>(1)
    }
}
