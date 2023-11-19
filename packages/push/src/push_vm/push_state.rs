use crate::{
    error::{stateful::FatalError, try_recover::TryRecover, InstructionResult},
    instruction::{Instruction, PushInstruction, PushInstructionError, VariableName},
    push_vm::{stack::Stack, PushInteger, State},
};
use std::collections::HashMap;

// TODO: It might make sense to separate out the specification of
//   a Push implementation (i.e., the relevant traits) into its
//   own package, and have the implementation of those traits in
//   its own package as well. We could, for example, do a FFI
//   implementation that just forwards to the a Clojure implementation
//   or Python implementation for comparison/testing purposes.

#[derive(Default, Debug, Eq, PartialEq, Clone)]
#[push_macros::push_state(builder)]
pub struct PushState {
    #[stack(exec)]
    pub(crate) exec: Stack<PushInstruction>,
    #[stack]
    pub(crate) int: Stack<PushInteger>,
    #[stack]
    pub(crate) bool: Stack<bool>,
    // The Internet suggests that when you have fewer than 15 entries,
    // linear search on `Vec` is faster than `HashMap`. I found that
    // using `HashMap` here did slow things down, mostly
    // through substantially increased allocation time for `HashMap` vs.
    // `Vec`. When I substantially increased the size of the programs,
    // however, the difference pretty much disappeared, presumably
    // because the execution of long programs swamps the cost of
    // initialization of `PushState`.
    #[input_instructions]
    pub(super) input_instructions: HashMap<VariableName, PushInstruction>,
}

impl PushState {
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
        // The `pop()` call can only return a `StackError`, which is either underflow or
        // overflow, with the latter not possible when just popping. So I'm not going to
        // bother capturing the error here.
        while let Ok(instruction) = self.exec.pop() {
            self = self.perform(&instruction).try_recover()?;
        }
        Ok(self)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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
            PushInstruction::InputVar(VariableName::from("x")), // [5]
            PushInstruction::InputVar(VariableName::from("y")), // [8, 5]
            push_bool(true),                                    // [true]
            PushInstruction::InputVar(VariableName::from("a")), // [true, true]
            push_int(9),                                        // [9, 8, 5]
            BoolInstruction::Or.into(),                         // [true]
            IntInstruction::Add.into(),                         // [17, 5]
            push_int(6),                                        // [6, 17, 5]
            IntInstruction::IsEven.into(),                      // [17, 5], [true, true]
            BoolInstruction::And.into(),                        // [true]
            PushInstruction::InputVar(VariableName::from("b")), // [false, true]
        ];
        let state = PushState::builder()
            .with_max_stack_size(1000)
            .with_program(program)
            .unwrap()
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
        // TODO: Replace this with a `has_stack` call when it exists.
        // assert!(state.exec().is_empty());
        assert_eq!(&state.int, &vec![5, 17]);
        assert_eq!(&state.bool, &vec![true, false]);
    }
}