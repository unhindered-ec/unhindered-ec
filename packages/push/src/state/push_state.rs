use std::{collections::HashMap, iter::repeat_with};

use ec_core::generator::Generator;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::instruction::Instruction;

use super::State;

#[derive(Default, Debug)]
pub struct PushState {
    exec: Vec<PushInstruction>,
    int: Vec<i64>,
    bool: Vec<bool>,
    inputs: HashMap<String, PushInstruction>,
}

impl PushState {
    pub fn new<P>(program: P) -> Self
    where
        P: IntoIterator<Item = PushInstruction>,
        P::IntoIter: DoubleEndedIterator,
    {
        Self {
            exec: program.into_iter().rev().collect(),
            int: Vec::new(),
            bool: Vec::new(),
            inputs: HashMap::new(),
        }
    }

    #[must_use]
    pub fn with_input(mut self, input_name: &str, input_value: i64) -> Self {
        self.inputs.insert(
            input_name.to_string(),
            PushInstruction::push_int(input_value),
        );
        self
    }

    #[must_use]
    pub fn with_int_stack(mut self, int_stack: Vec<i64>) -> Self {
        self.int = int_stack;
        self
    }

    #[must_use]
    pub const fn exec(&self) -> &Vec<PushInstruction> {
        &self.exec
    }

    fn push_input(&mut self, name: &str) {
        // TODO: This `.unwrap()` is icky, and we really should deal with it better.
        //   I wonder if the fact that this name might not be there should be telling
        //   us something...
        let instruction = self.inputs.get(name).unwrap().clone();
        instruction.perform(self);
    }

    #[must_use]
    pub const fn int(&self) -> &Vec<i64> {
        &self.int
    }

    #[must_use]
    pub const fn bool(&self) -> &Vec<bool> {
        &self.bool
    }
}

impl State for PushState {
    type Instruction = PushInstruction;

    // TODO: Need to have some kind of execution limit to prevent infinite loops.
    // `run` probably isn't a great name here?
    fn run_to_completion(&mut self) {
        while let Some(instruction) = self.exec.pop() {
            self.perform(&instruction);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PushInstruction {
    InputVar(String),
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
        let x = stack.pop().unwrap();
        let y = stack.pop().unwrap();
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
            Self::InputVar(name) => state.push_input(name),
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

// TODO: We probably want something like an `InstructionSet` type
//  and a `.generate()` on that that generates a random instruction.

pub struct GeneratorContext {
    pub max_initial_instructions: usize,
    pub instruction_set: Vec<PushInstruction>,
}

impl Generator<Vec<PushInstruction>, GeneratorContext> for ThreadRng {
    fn generate(&mut self, context: &GeneratorContext) -> Vec<PushInstruction> {
        let length = self.gen_range(0..context.max_initial_instructions);
        repeat_with(|| {
            context
                .instruction_set
                .choose(self)
                // TODO: Can we do better here? Should this return an `anyhow::Error`?
                .expect("The instruction set was empty")
                .clone()
        })
        .take(length)
        .collect()
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

#[cfg(test)]
mod simple_check {
    use crate::state::push_state::{PushInstruction, PushState};

    use super::{BoolInstruction, IntInstruction, State};

    #[test]
    fn run_simple_program() {
        fn push_bool(b: bool) -> PushInstruction {
            PushInstruction::push_bool(b)
        }

        fn push_int(i: i64) -> PushInstruction {
            PushInstruction::push_int(i)
        }

        // TODO: Can I make this a Vec<dyn Into<PushInstruction>> and
        //   then just `map.(Into::into)` across them all so I don't
        //   have to repeat the `.into()` over and over?
        let program = vec![
            push_int(5),
            push_int(8),
            push_bool(true),
            push_int(9),
            BoolInstruction::BoolOr.into(),
            IntInstruction::Add.into(),
            push_int(6),
            IntInstruction::IsEven.into(),
            BoolInstruction::BoolAnd.into(),
        ];
        let mut state = PushState::new(program);
        println!("{state:?}");
        state.run_to_completion();
        println!("{state:?}");
        assert!(state.exec().is_empty());
        assert_eq!(state.int(), &vec![5, 17]);
        assert_eq!(state.bool(), &vec![true]);
    }
}
