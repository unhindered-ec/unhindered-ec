use std::{collections::HashMap, io::Cursor, string::FromUtf8Error};

pub use ordered_float::OrderedFloat;

use super::push_io::HasStdout;
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
    #[stack(exec)]
    pub(crate) exec: Stack<PushProgram>,
    #[stack(sample_values = [4, 5, 7])]
    pub(crate) int: Stack<i64>,
    #[stack(sample_values = [OrderedFloat(4.3), OrderedFloat(5.1), OrderedFloat(2.1)])]
    pub(crate) float: Stack<OrderedFloat<f64>>,
    #[stack(sample_values = [true, false, true, true])]
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

    /// Used to represent the standard output used by an evolved
    /// program when "printing".
    stdout: Cursor<Vec<u8>>,

    #[instruction_step_limit]
    max_instruction_steps: usize,
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

    #[must_use]
    pub const fn max_instruction_steps(&self) -> usize {
        self.max_instruction_steps
    }

    // TODO: I thought about unwrapping here instead of returning a `Result`.
    //   It seems that if the only way we put things in our `Cursor` is through
    //   `write!()` calls, then the results in the `Cursor` should be legal.
    //   Evolution is weird, though, so it might still make sense to return
    //   a `Result` just in case we evolve something that breaks the interpretation
    //   of the `Cursor` as a `String`.
    //
    /// Return the contents of `stdout` as a `String`
    ///
    /// # Errors
    ///
    /// Returns a `FromUtf8Error` if there is a problem converting
    /// the contents of `Self::Stdout` into a `String`.
    pub fn stdout_string(&mut self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.stdout.clone().into_inner())
    }
}

impl State for PushState {
    type Instruction = PushProgram;

    fn run_to_completion(mut self) -> Result<Self, FatalError<Self, PushInstructionError>> {
        let mut instruction_steps = 0;
        // If we exceed the maximum number of instructions, then
        // we return the final state. The scorer can then use whatever
        // values are in that state for its scoring.
        while instruction_steps < self.max_instruction_steps() {
            // The `pop()` call can only return a `StackError`, which is either underflow or
            // overflow, with the latter not possible when just popping. So I'm not going to
            // bother capturing the error here.
            let Ok(program) = self.exec.pop() else {
                break;
            };
            self = self.perform(&program).try_recover()?;
            match instruction_steps.checked_add(1) {
                Some(new_steps) => instruction_steps = new_steps,
                None => break,
            }
        }
        Ok(self)
    }
}

impl HasStdout for PushState {
    type Stdout = Cursor<Vec<u8>>;

    fn stdout(&mut self) -> &mut Self::Stdout {
        &mut self.stdout
    }
}

#[cfg(test)]
mod tests {
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
            FloatInstruction::dup(),    // [3.5, 3.5]
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
            .with_instruction_step_limit(1_000)
            .build();

        let state = state.run_to_completion().unwrap();

        assert!(state.exec.is_empty());
        assert_eq!(&state.int, &[5, 17]);
        assert_eq!(&state.bool, &[true, false]);
        assert_eq!(&state.float, &[OrderedFloat(13.0)]);
    }

    #[test]
    fn run_simple_program_with_zero_instruction_steps() {
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
            FloatInstruction::dup(),    // [3.5, 3.5]
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
            .with_instruction_step_limit(0)
            .build();
        let initial_state = state.clone();

        let state = state.run_to_completion().unwrap();

        // Nothing should happen because we've set the maximum instruction steps
        // to 0, and no instructions should be performed.
        assert_eq!(state, initial_state);
    }
}
