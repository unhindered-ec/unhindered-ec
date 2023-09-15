use crate::push_vm::push_state::PushState;
use std::{fmt::Debug, fmt::Display, sync::Arc};

#[allow(clippy::module_name_repetitions)]
pub use self::{bool::BoolInstruction, int::IntInstruction};

mod bool;
mod int;

/*
 * exec_if requires a boolean and two (additional) values on the exec stack.
 * If the bool is true, we remove the second of the two exec stack values,
 * and if it's false, we remove the first.
 */

/*
 * exec_while requires a boolean and one additional value on the exec stack.
 * If the bool is true, then you push a copy of the "body" onto the exec, followed
 * by another copy of exec_while.
 */

/*
 * Instructions that are generic over stacks:
 *
 * - push
 * - pop
 * - dup (int_dup, exec_dup, bool_dup, ...)
 */

pub trait Instruction<S> {
    fn perform(&self, state: &mut S);
}

impl<S> Instruction<S> for Box<dyn Instruction<S>> {
    fn perform(&self, state: &mut S) {
        self.as_ref().perform(state);
    }
}

// impl<F> Instruction for F
// where
//     F: Fn(dyn State) -> dyn State
// {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariableName(Arc<str>);

impl From<&str> for VariableName {
    fn from(s: &str) -> Self {
        Self(Arc::from(s))
    }
}

impl Display for VariableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod variable_name_test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn variable_name() {
        let x = VariableName::from("x");
        let x2 = VariableName::from("x");
        assert_eq!(x, x2);
        let y = VariableName::from("y");
        assert_ne!(x, y);

        let mut map = HashMap::new();
        map.insert(x.clone(), 5);
        map.insert(y.clone(), 7);

        assert_eq!(map.get(&x).unwrap(), &5);
        assert_eq!(map.get(&y).unwrap(), &7);
        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&x2).unwrap(), &5);

        let z = VariableName::from("z");
        assert_eq!(map.get(&z), None);
    }
}

#[derive(Clone, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum PushInstruction {
    InputVar(VariableName),
    BoolInstruction(BoolInstruction),
    IntInstruction(IntInstruction),
}

impl Debug for PushInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputVar(arg0) => write!(f, "{arg0}"),
            Self::BoolInstruction(arg0) => write!(f, "Bool-{arg0}"),
            Self::IntInstruction(arg0) => write!(f, "Int-{arg0}"),
        }
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
            Self::InputVar(var_name) => state.push_input(var_name),
            Self::BoolInstruction(i) => i.perform(state),
            Self::IntInstruction(i) => i.perform(state),
        }
    }
}
