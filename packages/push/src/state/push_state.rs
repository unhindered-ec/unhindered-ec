use crate::instruction::Instruction;

use super::State;

#[derive(Default, Debug)]
pub struct PushState {
    exec: Vec<PushInstruction>,
    int: Vec<i128>,
    bool: Vec<bool>,
}

impl PushState {
    pub fn new<P>(program: P) -> PushState
    where
        P: IntoIterator<Item = PushInstruction>,
        P::IntoIter: DoubleEndedIterator,
    {
        PushState {
            exec: program.into_iter().rev().collect(),
            int: Vec::new(),
            bool: Vec::new(),
        }
    }

    pub fn exec(&self) -> &Vec<PushInstruction> {
        &self.exec
    }

    pub fn int(&self) -> &Vec<i128> {
        &self.int
    }

    pub fn bool(&self) -> &Vec<bool> {
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

pub enum PushInstruction {
    Int(i128),
    Bool(bool),
    IntAdd,
    IntIsEven,
    BoolOr,
}

#[inline(always)]
fn pop2<T>(stack: &mut Vec<T>) -> Option<(T, T)> {
    if stack.len() >= 2 {
        let x = stack.pop().unwrap();
        let y = stack.pop().unwrap();
        Some((x, y))
    } else {
        None
    }
}

impl Instruction<PushState> for PushInstruction {
    fn perform(&self, state: &mut PushState) {
        match self {
            PushInstruction::Int(i) => state.int.push(*i),
            PushInstruction::Bool(b) => state.bool.push(*b),
            PushInstruction::IntAdd => {
                if let Some((x, y)) = pop2(&mut state.int) {
                    // TODO: We should probably check that this addition succeeds and do something
                    //   sensible if it doesn't. That requires having these return a `Result` or
                    //   `Option`, however, which we don't yet do.
                    state.int.push(x + y);
                }
            }
            PushInstruction::IntIsEven => {
                if let Some(i) = state.int.pop() {
                    state.bool.push(i % 2 == 0);
                }
            }
            PushInstruction::BoolOr => {
                if let Some((x, y)) = pop2(&mut state.bool) {
                    state.bool.push(x || y);
                }
            }
        }
    }
}
