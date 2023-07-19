use std::{collections::HashMap, sync::Arc};

use super::State;
use crate::instruction::{Instruction, PushInstruction, VariableName};

#[derive(Debug)]
pub struct Stack<T> {
    values: Vec<T>,
}

// We implemented this by hand instead of using `derive`
// because `derive` would have required that `T: Default`,
// but that's not necessary for an empty stack. Doing this
// by hand avoids that requirement.
impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
        }
    }
}

impl<T> Stack<T> {
    #[must_use]
    pub fn size(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn top(&self) -> Option<&T> {
        self.values.last()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.values.pop()
    }

    pub fn pop2(&mut self) -> Option<(T, T)> {
        if self.size() >= 2 {
            let x = self.pop()?;
            let y = self.pop()?;
            Some((x, y))
        } else {
            None
        }
    }

    pub fn push(&mut self, value: T) {
        self.values.push(value);
    }

    /// Adds the given sequence of values to this stack.
    ///
    /// The first value in `values` will be the new top of the
    /// stack. If the stack was initially empty, the last value
    /// in `values` will be the new bottom of the stack.
    ///
    /// # Arguments
    ///
    /// * `values` - A `Vec` holding the values to add to the stack
    ///
    /// # Examples
    ///
    /// ```
    /// use push::push_vm::push_state::Stack;
    /// let mut stack: Stack<i64> = Stack::default();
    /// assert_eq!(stack.size(), 0);
    /// stack.extend(vec![5, 8, 9]);
    /// // Now the top of the stack is 5, followed by 8, then 9 at the bottom.
    /// assert_eq!(stack.size(), 3);
    /// assert_eq!(stack.top().unwrap(), &5);
    /// stack.extend(vec![6, 3]);
    /// // Now the top of the stack is 6 and the whole stack is 6, 3, 5, 8, 9.
    /// assert_eq!(stack.size(), 5);
    /// assert_eq!(stack.top().unwrap(), &6);
    /// ```  
    pub fn extend(&mut self, values: Vec<T>) {
        self.values.extend(values.into_iter().rev());
    }
}

pub trait HasStack<T> {
    fn stack_mut(&mut self) -> &mut Stack<T>;
}

#[derive(Default, Debug)]
pub struct PushState {
    pub(crate) exec: Vec<PushInstruction>,
    pub(crate) int: Stack<i64>,
    pub(crate) bool: Stack<bool>,
    // The Internet suggests that when you have fewer than 15 entries,
    // linear search on `Vec` is faster than `HashMap`. I found that
    // using `HashMap` here did slow things down, mostly
    // through substantially increased allocation time for `HashMap` vs.
    // `Vec`. When I substantially increased the size of the programs,
    // however, the difference pretty much disappeared, presumably
    // because the execution of long programs swamps the cost of
    // initialization of `PushState`.
    input_instructions: HashMap<VariableName, PushInstruction>,
}

impl HasStack<bool> for PushState {
    fn stack_mut(&mut self) -> &mut Stack<bool> {
        &mut self.bool
    }
}

impl HasStack<i64> for PushState {
    fn stack_mut(&mut self) -> &mut Stack<i64> {
        &mut self.int
    }
}

pub struct Builder {
    partial_state: PushState,
}

impl Builder {
    #[must_use]
    pub const fn new(partial_state: PushState) -> Self {
        Self { partial_state }
    }

    // TODO: Something like the following would be nice and avoid the repetition
    //   in the next two functions. This doesn't work, though, because we don't
    //   have a way to say that the `partial_state` field implements `HasStack<T>`.
    //   I think we'd have to add a generic to `Builder` and a new `BuildableState`
    //   trait (or something like that) to make that work.
    // pub fn with_values<T>(mut self, values: Vec<T>) -> Self
    // where
    //     Self: HasStack<T>,
    // {
    //     let stack: &mut Stack<T> = self.partial_state.stack_mut();
    //     stack.extend(values);
    //     self
    // }

    /// Adds the given sequence of values to the boolean stack for the state you're building.
    ///
    /// The first value in `values` will be the new top of the
    /// stack. If the stack was initially empty, the last value
    /// in `values` will be the new bottom of the stack.
    ///
    /// # Arguments
    ///
    /// * `values` - A `Vec` holding the values to add to the stack
    ///
    /// # Examples
    ///
    /// ```
    /// use push::push_vm::push_state::Stack;
    /// use crate::push::push_vm::push_state::HasStack;
    /// use push::push_vm::push_state::PushState;
    /// use push::push_vm::push_state::Builder;
    /// let mut state = Builder::new(PushState::default())
    ///     .with_bool_values(vec![true, false, false])
    ///     .build();
    /// let bool_stack: &Stack<bool> = state.stack_mut();
    /// assert_eq!(bool_stack.size(), 3);
    /// // Now the top of the stack is `true`, followed by `false`, then `false` at the bottom.
    /// assert_eq!(bool_stack.top().unwrap(), &true);
    /// ```  
    #[must_use]
    pub fn with_bool_values(mut self, values: Vec<bool>) -> Self {
        let bool_stack: &mut Stack<bool> = self.partial_state.stack_mut();
        bool_stack.extend(values);
        self
    }

    /// Adds the given sequence of values to the integer stack for the state you're building.
    ///
    /// The first value in `values` will be the new top of the
    /// stack. If the stack was initially empty, the last value
    /// in `values` will be the new bottom of the stack.
    ///
    /// # Arguments
    ///
    /// * `values` - A `Vec` holding the values to add to the stack
    ///
    /// # Examples
    ///
    /// ```
    /// use push::push_vm::push_state::Stack;
    /// use crate::push::push_vm::push_state::HasStack;
    /// use push::push_vm::push_state::PushState;
    /// use push::push_vm::push_state::Builder;
    /// let mut state = Builder::new(PushState::default())
    ///     .with_int_values(vec![5, 8, 9])
    ///     .build();
    /// let int_stack: &Stack<i64> = state.stack_mut();
    /// assert_eq!(int_stack.size(), 3);
    /// // Now the top of the stack is 5, followed by 8, then 9 at the bottom.
    /// assert_eq!(int_stack.top().unwrap(), &5);
    /// ```  
    #[must_use]
    pub fn with_int_values(mut self, values: Vec<i64>) -> Self {
        let int_stack: &mut Stack<i64> = self.partial_state.stack_mut();
        int_stack.extend(values);
        self
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
    //
    // TODO: Create a macro that generates this instruction for a given type
    //   so we don't have to repeat this logic for every type.
    #[must_use]
    pub fn with_int_input(mut self, input_name: &str, input_value: i64) -> Self {
        self.partial_state.input_instructions.insert(
            Arc::from(input_name),
            PushInstruction::push_int(input_value),
        );
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
        self.partial_state.input_instructions.insert(
            Arc::from(input_name),
            PushInstruction::push_bool(input_value),
        );
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
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> PushState {
        self.partial_state
    }
}

impl PushState {
    pub fn builder<P>(program: P) -> Builder
    where
        P: IntoIterator<Item = PushInstruction>,
        P::IntoIter: DoubleEndedIterator,
    {
        let partial_state = Self {
            exec: program.into_iter().rev().collect(),
            int: Stack::<i64>::default(),
            bool: Stack::<bool>::default(),
            input_instructions: HashMap::new(),
        };
        Builder::new(partial_state)
    }

    #[must_use]
    pub const fn exec(&self) -> &Vec<PushInstruction> {
        &self.exec
    }

    /// # Panics
    ///
    /// This panics if we try to access a variable whose `var_index` isn't in the
    /// variable map.
    pub fn push_input(&mut self, var_name: &VariableName) {
        // TODO: This `panic` here is icky, and we really should deal with it better.
        //   I wonder if the fact that this index might not be there should be telling
        //   us something...
        let instruction = self
            .input_instructions
            .iter()
            .find_map(|(n, v)| if n == var_name { Some(v) } else { None })
            .unwrap_or_else(|| panic!("Failed to get an instruction for the input variable '{var_name}' that hadn't been defined"))
            .clone();
        instruction.perform(self);
    }
}

impl State for PushState {
    type Instruction = PushInstruction;

    // TODO: Need to have some kind of execution limit to prevent infinite loops.
    fn run_to_completion(mut self) -> Self {
        while let Some(instruction) = self.exec.pop() {
            self.perform(&instruction);
        }
        self
    }
}

#[cfg(test)]
mod simple_check {
    use std::sync::Arc;

    use crate::{
        instruction::{BoolInstruction, IntInstruction, PushInstruction},
        push_vm::push_state::PushState,
    };

    use super::State;

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
            // push_int(5),
            // push_int(8),
            PushInstruction::InputVar(Arc::from("x")),
            PushInstruction::InputVar(Arc::from("y")),
            push_bool(true),
            PushInstruction::InputVar(Arc::from("a")),
            push_int(9),
            BoolInstruction::BoolOr.into(),
            IntInstruction::Add.into(),
            push_int(6),
            IntInstruction::IsEven.into(),
            BoolInstruction::BoolAnd.into(),
            PushInstruction::InputVar(Arc::from("b")),
        ];
        let state = PushState::builder(program)
            .with_bool_input("a", true)
            .with_bool_input("b", false)
            // I'm reversing the order of the variables on purpose here to make sure
            // that order doesn't matter.
            .with_int_input("y", 8)
            .with_int_input("x", 5)
            .build();
        println!("{state:?}");
        let state = state.run_to_completion();
        println!("{state:?}");
        assert!(state.exec().is_empty());
        assert_eq!(&state.int.values, &vec![5, 17]);
        assert_eq!(&state.bool.values, &vec![true, false]);
    }
}
