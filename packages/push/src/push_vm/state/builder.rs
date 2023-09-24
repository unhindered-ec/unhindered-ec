use crate::{
    instruction::{PushInstruction, VariableName},
    push_vm::{
        stack::{
            traits::{extend::ExtendHeadIn, size::SizeLimitOfMut},
            StackError,
        },
        PushInteger,
    },
};

use super::{PushState, PushStateUnmasked};

pub struct Builder {
    partial_state: PushState,
}

impl Builder {
    #[must_use]
    pub const fn new(partial_state: PushStateUnmasked) -> Self {
        Self {
            partial_state: PushState(partial_state),
        }
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
    pub fn with_max_stack_size(mut self, max_stack_size: usize) -> Result<Self, StackError> {
        (&mut self.partial_state).set_max_size_of::<PushInteger>(max_stack_size)?;
        (&mut self.partial_state).set_max_size_of::<bool>(max_stack_size)?;
        Ok(self)
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
    pub fn with_bool_values(mut self, values: Vec<bool>) -> Result<Self, StackError> {
        (&mut self.partial_state).extend_head_in::<bool, _>(values)?;
        Ok(self)
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
    pub fn with_int_values(mut self, values: Vec<PushInteger>) -> Result<Self, StackError> {
        (&mut self.partial_state).extend_head_in::<PushInteger, _>(values)?;
        Ok(self)
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
        self.partial_state.0.input_instructions.insert(
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
        self.partial_state.0.input_instructions.insert(
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
