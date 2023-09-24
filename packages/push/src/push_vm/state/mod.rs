pub mod builder;
pub mod with_state;

use self::builder::Builder;

use super::{
    stack::{
        simple::{Limited, SimpleStack, SimpleStackLimited},
        traits::has_stack::{HasStack, HasStackMut},
    },
    State,
};
use crate::{
    error::{stateful::FatalError, try_recover::TryRecover, InstructionResult},
    instruction::{Instruction, PushInstruction, PushInstructionError, VariableName},
    push_vm::PushInteger,
    type_eq::TypeEq,
};
use std::collections::HashMap;

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct PushState {
    pub(crate) exec: Vec<PushInstruction>,
    pub(crate) int: SimpleStackLimited<PushInteger>,
    pub(crate) bool: SimpleStackLimited<bool>,
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
    type StackType = SimpleStack<bool, Limited>;

    fn stack<U: TypeEq<This = bool>>(&self) -> &Self::StackType {
        &self.bool
    }
}

impl HasStackMut<bool> for PushState {
    fn stack_mut<U: TypeEq<This = bool>>(&mut self) -> &mut Self::StackType {
        &mut self.bool
    }
}

impl HasStack<PushInteger> for PushState {
    type StackType = SimpleStackLimited<PushInteger>;

    fn stack<U: TypeEq<This = PushInteger>>(&self) -> &Self::StackType {
        &self.int
    }
}

impl HasStackMut<PushInteger> for PushState {
    fn stack_mut<U: TypeEq<This = PushInteger>>(&mut self) -> &mut Self::StackType {
        &mut self.int
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
            int: SimpleStackLimited::<PushInteger>::default(),
            bool: SimpleStackLimited::<bool>::default(),
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
        &mut self,
        var_name: &VariableName,
    ) -> InstructionResult<&mut Self, <PushInstruction as Instruction<Self>>::Error> {
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
    fn run_to_completion(&mut self) -> Result<(), FatalError<&mut Self, PushInstructionError>> {
        while let Some(instruction) = self.exec.pop() {
            self.perform(&instruction).try_recover()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod simple_check {
    use crate::{
        instruction::{BoolInstruction, IntInstruction, PushInstruction, VariableName},
        push_vm::{
            stack::traits::{get::GetHeadIn, size::StackSizeOf},
            state::{with_state::WithStateOps, PushInteger, PushState},
            State,
        },
    };

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
        state.run_to_completion().unwrap();
        println!("{state:?}");
        assert!(state.exec().is_empty());
        assert_eq!(state.size_of::<PushInteger>().drop_state(), 2);
        assert_eq!(
            state.get_n_head_in::<PushInteger, _>().drop_state(),
            Ok((&5, &17))
        );
        assert_eq!(state.size_of::<bool>().drop_state(), 2);
        assert_eq!(
            state.get_n_head_in::<bool, _>().drop_state(),
            Ok((&true, &false))
        );
    }
}
