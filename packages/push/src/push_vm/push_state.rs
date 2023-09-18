use crate::{
    error::{stateful::FatalError, try_recover::TryRecover, InstructionResult},
    instruction::{Instruction, PushInstruction, PushInstructionError, VariableName},
    push_vm::{
        stack::{HasStack, Stack, TypeEq},
        PushInteger, State,
    },
};
use std::collections::HashMap;

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct PushState {
    pub(crate) exec: Vec<PushInstruction>,
    pub(crate) int: Stack<PushInteger>,
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
    fn stack<U: TypeEq<This = bool>>(&self) -> &Stack<bool> {
        &self.bool
    }

    fn stack_mut<U: TypeEq<This = bool>>(&mut self) -> &mut Stack<bool> {
        &mut self.bool
    }
}

impl HasStack<PushInteger> for PushState {
    fn stack<U: TypeEq<This = PushInteger>>(&self) -> &Stack<PushInteger> {
        &self.int
    }

    fn stack_mut<U: TypeEq<This = PushInteger>>(&mut self) -> &mut Stack<PushInteger> {
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

    // TODO: These Doctests fail because of the change in the visibility of `Stack`.
    //   I'm not sure what environment Doctests are run in, so I'm not entirely
    //   sure how to fix this.

    /// Sets the maximum stack size for all the stacks in this state.
    ///
    /// # Arguments
    ///
    /// * `max_stack_size` - A `usize` specifying the maximum stack size
    ///
    /// # Examples
    ///
    /// ```
    /// use push::push_vm::push_state::Stack;
    /// use crate::push::push_vm::push_state::HasStack;
    /// use push::push_vm::push_state::PushState;
    /// use push::push_vm::push_state::Builder;
    /// let mut state = Builder::new(PushState::default())
    ///     .with_max_stack_size(100)
    ///     .build();
    /// let bool_stack: &Stack<bool> = state.stack();
    /// assert_eq!(bool_stack.max_stack_size, 100);
    /// ```  
    #[must_use]
    pub fn with_max_stack_size(mut self, max_stack_size: usize) -> Self {
        self.partial_state.int.set_max_stack_size(max_stack_size);
        self.partial_state.bool.set_max_stack_size(max_stack_size);
        self
    }

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
    /// let bool_stack: &Stack<bool> = state.stack();
    /// assert_eq!(bool_stack.size(), 3);
    /// // Now the top of the stack is `true`, followed by `false`, then `false` at the bottom.
    /// assert_eq!(bool_stack.top().unwrap(), &true);
    /// ```  
    #[must_use]
    pub fn with_bool_values(mut self, values: Vec<bool>) -> Self {
        let bool_stack = self.partial_state.stack_mut::<bool>();
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
    /// let int_stack: &Stack<PushInteger> = state.stack();
    /// assert_eq!(int_stack.size(), 3);
    /// // Now the top of the stack is 5, followed by 8, then 9 at the bottom.
    /// assert_eq!(int_stack.top().unwrap(), &5);
    /// ```  
    #[must_use]
    pub fn with_int_values(mut self, values: Vec<PushInteger>) -> Self {
        let int_stack = self.partial_state.stack_mut::<PushInteger>();
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
    pub fn with_int_input(mut self, input_name: &str, input_value: PushInteger) -> Self {
        self.partial_state.input_instructions.insert(
            VariableName::from(input_name),
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
            VariableName::from(input_name),
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
            int: Stack::<PushInteger>::default(),
            bool: Stack::<bool>::default(),
            input_instructions: HashMap::new(),
        };
        Builder::new(partial_state)
    }

    #[must_use]
    pub const fn exec(&self) -> &Vec<PushInstruction> {
        &self.exec
    }

    // /// # Panics
    // ///
    // /// This panics if we try to access a variable whose `var_index` isn't in the
    // /// variable map.
    // pub fn push_input(&mut self, var_name: &VariableName) {
    //     let instruction = self
    //         .input_instructions
    //         .iter()
    //         .find_map(|(n, v)| if n == var_name { Some(v) } else { None })
    //         .unwrap_or_else(|| panic!("Failed to get an instruction for the input variable '{var_name}' that hadn't been defined"))
    //         .clone();
    //     instruction.perform(self);
    // }

    /// # Errors
    ///
    /// This returns an error if the `PushInstruction` returns an error, which really shouldn't happen.
    ///
    /// # Panics
    ///
    /// This panics if there is no instruction associated with `var_name`, i.e.,
    /// we have not yet added that variable name to the map of names to instructions.
    pub fn with_input(
        self,
        var_name: &VariableName,
    ) -> InstructionResult<Self, <PushInstruction as Instruction<Self>>::Error> {
        // TODO: This `panic` here is icky, and we really should deal with it better.
        //   I wonder if the fact that this index might not be there should be telling
        //   us something...
        let instruction = self
            .input_instructions
            .iter()
            .find_map(|(n, v)| if n == var_name { Some(v) } else { None })
            .unwrap_or_else(|| panic!("Failed to get an instruction for the input variable '{var_name}' that hadn't been defined"))
            .clone();
        instruction.perform(self)
    }
}

impl State for PushState {
    type Instruction = PushInstruction;

    // TODO: Need to have some kind of execution limit to prevent infinite loops.
    fn run_to_completion(mut self) -> Result<Self, FatalError<Self, PushInstructionError>> {
        // This smells off to me. In places we're using side effects on mutable structures (e.g., `pop`)
        // while in other places we're taking a more functional approach (e.g., `self = self.perform()`).
        // It seems that maybe I should pick one or the other. Being able to store the state in
        // the errors appears to be part of the source of the problem here (again), which more and
        // more makes me wonder if that's a good idea.
        //
        // TODO: Justus_Fluegel@Twitch suggested a `try_recover()?` method to encapsulate the
        //   `match error.severity()` logic.
        while let Some(instruction) = self.exec.pop() {
            self = self.perform(&instruction).try_recover()?;
        }
        Ok(self)
    }
}

#[cfg(test)]
mod simple_check {
    use crate::{
        instruction::{BoolInstruction, IntInstruction, PushInstruction, VariableName},
        push_vm::push_state::{PushInteger, PushState},
    };

    use super::State;

    #[test]
    fn run_simple_program() {
        fn push_bool(b: bool) -> PushInstruction {
            PushInstruction::push_bool(b)
        }

        fn push_int(i: PushInteger) -> PushInstruction {
            PushInstruction::push_int(i)
        }

        let program = vec![
            // push_int(5),
            // push_int(8),
            PushInstruction::InputVar(VariableName::from("x")),
            PushInstruction::InputVar(VariableName::from("y")),
            push_bool(true),
            PushInstruction::InputVar(VariableName::from("a")),
            push_int(9),
            BoolInstruction::Or.into(),
            IntInstruction::Add.into(),
            push_int(6),
            IntInstruction::IsEven.into(),
            BoolInstruction::And.into(),
            PushInstruction::InputVar(VariableName::from("b")),
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
        #[allow(clippy::unwrap_used)]
        let state = state.run_to_completion().unwrap();
        println!("{state:?}");
        assert!(state.exec().is_empty());
        assert_eq!(&state.int, &vec![5, 17]);
        assert_eq!(&state.bool, &vec![true, false]);
    }
}
