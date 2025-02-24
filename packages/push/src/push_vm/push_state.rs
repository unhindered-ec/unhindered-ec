use std::collections::HashMap;

pub use ordered_float::OrderedFloat;

use crate::{
    error::{InstructionResult, stateful::FatalError, try_recover::TryRecover},
    instruction::{
        Instruction, PushInstruction, instruction_error::PushInstructionError,
        variable_name::VariableName,
    },
    push_vm::{State, program::PushProgram, stack::Stack},
};

// TODO: It might make sense to separate out the specification of
// a Push implementation (i.e., the relevant traits) into its
// own package, and have the implementation of those traits in
// its own package as well. We could, for example, do a FFI
// implementation that just forwards to the a Clojure implementation
// or Python implementation for comparison/testing purposes.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[push_macros::push_state(builder)]
pub struct PushState {
    /// A stack of `PushProgram`s
    ///
    /// The execution of a Push program consists of running the
    /// program at the top of the exec stack until the exec stack
    /// is empty.
    #[stack(exec)]
    pub(crate) exec: Stack<PushProgram>,

    /// A stack of `i64` values
    #[stack(sample_values = [4, 5, 7])]
    pub(crate) int: Stack<i64>,

    /// A stack of `OrderedFloat<f64> values`
    #[stack(sample_values = [OrderedFloat(4.3), OrderedFloat(5.1), OrderedFloat(2.1)])]
    pub(crate) float: Stack<OrderedFloat<f64>>,

    /// A stack of boolean values
    #[stack(sample_values = [true, false, true, true])]
    pub(crate) bool: Stack<bool>,

    /// A stack of character values
    #[stack(sample_values = ['a', 'b', 'c'])]
    pub(crate) char: Stack<char>,

    /// A map that maps from (input) variable names to their values.
    ///
    /// Thus if you have an input `x` which has the value 5 in this
    /// training case, then you would add an entry to this mapping
    /// with the key `VariableName::from("x")` and the value
    /// `PushInstruction::IntInstruction(IntInstruction::Push(5))`.
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
    //         .unwrap_or_else(|| panic!(
    //              "Failed to get an instruction for the input \
    //               variable '{var_name}' that hadn't been defined"
    //         ))
    //         .clone();
    //     instruction.perform(self);
    // }

    /// # Errors
    ///
    /// This returns an error if the `PushInstruction` returns an error,
    /// which really shouldn't happen.
    ///
    /// # Panics
    ///
    /// This panics if there is no instruction associated with `var_name`, i.e.,
    /// we have not yet added that variable name to the map of names to
    /// instructions.
    pub fn with_input(
        self,
        var_name: &VariableName,
    ) -> InstructionResult<Self, <PushInstruction as Instruction<Self>>::Error> {
        #[expect(
            clippy::panic,
            reason = "This is legacy and arguably should be changed. Tracked in #172"
        )]
        let instruction = self
            .input_instructions
            .iter()
            .find_map(|(n, v)| (n == var_name).then_some(v))
            .unwrap_or_else(|| {
                panic!(
                    "Failed to get an instruction for the input variable '{var_name}' that hadn't \
                     been defined"
                )
            })
            .clone();
        instruction.perform(self)
    }
}

impl State for PushState {
    type Instruction = PushProgram;

    // TODO: Need to have some kind of execution limit to prevent infinite loops.
    fn run_to_completion(mut self) -> Result<Self, FatalError<Self, PushInstructionError>> {
        // The `pop()` call can only return a `StackError`, which is either underflow or
        // overflow, with the latter not possible when just popping. So I'm not going to
        // bother capturing the error here.
        while let Ok(program) = self.exec.pop() {
            self = self.perform(&program).try_recover()?;
        }
        Ok(self)
    }
}

#[cfg(test)]
mod simple_check {
    use ordered_float::OrderedFloat;

    use super::State;
    use crate::{
        genome::plushy::{Plushy, PushGene},
        instruction::{
            BoolInstruction, FloatInstruction, IntInstruction, PushInstruction,
            variable_name::VariableName,
        },
        list_into::vec_into,
        push_vm::{program::PushProgram, push_state::PushState},
    };

    #[test]
    fn run_simple_program() {
        fn push_bool(b: bool) -> PushInstruction {
            PushInstruction::push_bool(b)
        }

        fn push_int(i: i64) -> PushInstruction {
            PushInstruction::push_int(i)
        }

        fn push_float(f: f64) -> PushInstruction {
            PushInstruction::push_float(OrderedFloat(f))
        }

        let genes: Vec<PushGene> = vec_into![
            VariableName::from("x"),    // [5]
            VariableName::from("y"),    // [8, 5]
            push_bool(true),            // [true]
            VariableName::from("a"),    // [true, true]
            push_int(9),                // [9, 8, 5]
            BoolInstruction::Or,        // [true]
            IntInstruction::Add,        // [17, 5]
            push_int(6),                // [6, 17, 5]
            IntInstruction::IsEven,     // [17, 5], [true, true]
            BoolInstruction::And,       // [true]
            VariableName::from("b"),    // [false, true]
            push_float(3.5),            // [3.5]
            FloatInstruction::Dup,      // [3.5, 3.5]
            FloatInstruction::Multiply, // [12.25]
            VariableName::from("f"),    // [12.25, 0.75]
            FloatInstruction::Add,      // [13.0]
        ];

        let plushy = Plushy::new(genes);
        let state = PushState::builder()
            .with_max_stack_size(16)
            .with_program(Vec::<PushProgram>::from(plushy))
            .unwrap()
            .with_bool_input("a", true)
            .with_bool_input("b", false)
            // I'm reversing the order of the variables on purpose here to make sure
            // that order doesn't matter.
            .with_int_input("y", 8)
            .with_int_input("x", 5)
            .with_float_input("f", OrderedFloat(0.75))
            .build();

        let state = state.run_to_completion().unwrap();

        assert!(state.exec.is_empty());
        assert_eq!(&state.int, &[5, 17]);
        assert_eq!(&state.bool, &[true, false]);
        assert_eq!(&state.float, &[OrderedFloat(13.0)]);
    }
}
