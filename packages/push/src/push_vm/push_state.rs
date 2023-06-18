use super::State;
use crate::instruction::Instruction;

#[derive(Default)]
pub struct Inputs {
    input_names: Vec<String>,
}

impl Inputs {
    #[must_use]
    pub fn with_name(mut self, name: &str) -> Self {
        self.input_names.push(name.to_string());
        self
    }

    /// Get the index for the given input variable name.
    ///
    /// # Panics
    /// This will panic if the given name hasn't been added to
    /// the `Inputs` using, e.g., `with_name()`.
    #[must_use]
    pub fn get_index(&self, name: &str) -> usize {
        self.input_names
            .iter()
            .position(|n| n == name)
            .unwrap_or_else(|| panic!("Tried to access variable '{name}' that had not been added to the `Inputs` list: {:?}.", self.input_names))
    }

    #[must_use]
    pub fn to_instructions(&self) -> Vec<PushInstruction> {
        self.input_names
            .iter()
            .enumerate()
            .map(|(index, _)| PushInstruction::InputVar(index))
            .collect()
    }
}

#[derive(Default, Debug)]
pub struct PushState {
    exec: Vec<PushInstruction>,
    int: Vec<i64>,
    bool: Vec<bool>,
    input_instructions: Vec<PushInstruction>,
}

pub struct Builder<'i> {
    inputs: &'i Inputs,
    input_instructions: Vec<Option<PushInstruction>>,
    partial_state: PushState,
}

impl<'i> Builder<'i> {
    #[must_use]
    pub fn new(inputs: &'i Inputs, partial_state: PushState) -> Self {
        Self {
            inputs,
            input_instructions: vec![None; inputs.input_names.len()],
            partial_state,
        }
    }

    /// Adds an integer input instruction to the current current state's set
    /// of instructions. The name for the input must have been included
    /// in the `Inputs` provided when the `Builder` was initially constructed.
    /// Here you provide the name and the (int, i.e., `i64`) value for that
    /// input variable. That will create a new `PushInstruction::push_int()`
    /// instruction that will push the specified value onto the integer stack
    /// when performed.
    ///
    /// # Panics
    /// This panics if the `input_name` provided isn't included in the set of
    /// names in the `Inputs` object used in the construction of the `Builder`.
    #[must_use]
    pub fn with_int_input(mut self, input_name: &str, input_value: i64) -> Self {
        let index = self.inputs.get_index(input_name);
        let Some(entry) = self.input_instructions.get_mut(index) else {
            panic!("Tried to access input name {input_name} with index {index} in set of inputs: {:?}", self.inputs.input_names);
        };
        *entry = Some(PushInstruction::push_int(input_value));
        self
    }

    /// Adds an boolean input instruction to the current current state's set
    /// of instructions. The name for the input must have been included
    /// in the `Inputs` provided when the `Builder` was initially constructed.
    /// Here you provide the name and the boolean value for that
    /// input variable. That will create a new `PushInstruction::push_bool()`
    /// instruction that will push the specified value onto the boolean stack
    /// when performed.
    ///
    /// # Panics
    /// This panics if the `input_name` provided isn't included in the set of
    /// names in the `Inputs` object used in the construction of the `Builder`.
    #[must_use]
    pub fn with_bool_input(mut self, input_name: &str, input_value: bool) -> Self {
        let index = self.inputs.get_index(input_name);
        let Some(entry) = self.input_instructions.get_mut(index) else {
            panic!("Tried to access input name {input_name} with index {index} in set of inputs: {:?}", self.inputs.input_names);
        };
        *entry = Some(PushInstruction::push_bool(input_value));
        self
    }

    /// Finalize the build process, returning the fully constructed `PushState`
    /// value. For this to successfully build, all the input variables has to
    /// have been given values. Thus every input variable provided
    /// in the `Inputs` used when constructing the `Builder` must have had a
    /// corresponding `with_X_input()` call that specified the value for that
    /// variable.
    ///
    /// # Panics
    /// Panics if one or more of the variables provided in the `Inputs` wasn't
    /// then given a value during the build process.
    /*
     * Note that the `with_x_input()` functions ensure that the instruction for
     * that input variable will be in the same position in `self.input_instructions`
     * as the name is in `self.inputs.input_names`. This allows us to zip together
     * those two lists and know that we'll be pairing up instructions with the appropriate
     * names.
     */
    #[must_use]
    pub fn build(self) -> PushState {
        let input_instructions = self
            .input_instructions
            .into_iter()
            .zip(self.inputs.input_names.iter())
            .map(|(instruction, name)| instruction.ok_or(name))
            .collect::<Result<Vec<_>, &String>>()
            .unwrap_or_else(|name| {
                panic!("The variable {name} wasn't given a value in `PushState::Builder`.")
            });
        PushState {
            input_instructions,
            ..self.partial_state
        }
    }
}

impl PushState {
    pub fn builder<P>(program: P, inputs: &Inputs) -> Builder
    where
        P: IntoIterator<Item = PushInstruction>,
        P::IntoIter: DoubleEndedIterator,
    {
        let partial_state = Self {
            exec: program.into_iter().rev().collect(),
            int: Vec::new(),
            bool: Vec::new(),
            input_instructions: Vec::new(),
        };
        Builder::new(inputs, partial_state)
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

    fn push_input(&mut self, var_index: usize) {
        // TODO: This `.expect()` is icky, and we really should deal with it better.
        //   I wonder if the fact that this name might not be there should be telling
        //   us something...
        let instruction = self
            .input_instructions.get(var_index)
            .unwrap_or_else(|| panic!("We tried to get an instruction for the input variable with index '{var_index}' that hadn't been added"))
            .clone();
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
    fn run_to_completion(&mut self) -> &Self {
        while let Some(instruction) = self.exec.pop() {
            self.perform(&instruction);
        }
        self
    }
}

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

#[cfg(test)]
mod simple_check {
    use crate::push_vm::push_state::{Inputs, PushInstruction, PushState};

    use super::{BoolInstruction, IntInstruction, State};

    #[test]
    fn run_simple_program() {
        fn push_bool(b: bool) -> PushInstruction {
            PushInstruction::push_bool(b)
        }

        fn push_int(i: i64) -> PushInstruction {
            PushInstruction::push_int(i)
        }

        let inputs = Inputs::default()
            .with_name("x")
            .with_name("y")
            .with_name("a")
            .with_name("b");

        // TODO: Can I make this a Vec<dyn Into<PushInstruction>> and
        //   then just `map.(Into::into)` across them all so I don't
        //   have to repeat the `.into()` over and over?
        let program = vec![
            // push_int(5),
            // push_int(8),
            PushInstruction::InputVar(0),
            PushInstruction::InputVar(1),
            push_bool(true),
            PushInstruction::InputVar(2),
            push_int(9),
            BoolInstruction::BoolOr.into(),
            IntInstruction::Add.into(),
            push_int(6),
            IntInstruction::IsEven.into(),
            BoolInstruction::BoolAnd.into(),
            PushInstruction::InputVar(3),
        ];
        let mut state = PushState::builder(program, &inputs)
            .with_bool_input("a", true)
            .with_bool_input("b", false)
            // I'm reversing the order of the variables on purpose here to make sure
            // that order doesn't matter.
            .with_int_input("y", 8)
            .with_int_input("x", 5)
            .build();
        println!("{state:?}");
        state.run_to_completion();
        println!("{state:?}");
        assert!(state.exec().is_empty());
        assert_eq!(state.int(), &vec![5, 17]);
        assert_eq!(state.bool(), &vec![true, false]);
    }
}
