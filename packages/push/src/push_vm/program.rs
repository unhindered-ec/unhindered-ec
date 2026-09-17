use super::{HasStack, push_state::PushState, stack::StackError};
use crate::{
    error::{Error, InstructionResult},
    genome::plushy::{Plushy, PushGene},
    instruction::{
        Instruction, NumOpens, PushInstruction, instruction_error::PushInstructionError,
    },
};

/// A Push program: a single instruction or a block of programs.
///
/// `PushProgram` is generic over its instruction type, defaulting to
/// [`PushInstruction`]. Programs with other instruction types can still be
/// constructed, printed, and parsed from a [`Plushy`], but only
/// `PushProgram<PushInstruction>` can be *executed*: the [`Instruction`]
/// implementation that runs a program is defined for `PushState` and
/// `PushInstruction` only.
#[derive(Debug, strum_macros::Display, Clone, Eq, PartialEq)]
pub enum PushProgram<I = PushInstruction> {
    Instruction(I),
    Block(Vec<Self>),
}

impl<I> Default for PushProgram<I> {
    fn default() -> Self {
        Self::Block(Vec::new())
    }
}

impl<I> From<Plushy<I>> for Vec<PushProgram<I>>
where
    I: NumOpens,
{
    fn from(plushy: Plushy<I>) -> Self {
        let mut genes = plushy.into_iter();
        let mut program = Self::new();
        PushProgram::parse_from_plushy(true, &mut genes, &mut program);
        program
    }
}

impl<I> From<I> for PushProgram<I> {
    fn from(instruction: I) -> Self {
        Self::Instruction(instruction)
    }
}

impl<T> PushProgram<T> {
    /// Create a program consisting of the single instruction `i`, converting
    /// `i` to the program's instruction type `T` via `Into`.
    ///
    /// The surrounding context usually determines `T`. When it doesn't, as in
    /// a bare `PushProgram::new_instruction(x)`, specify it explicitly with
    /// `PushProgram::<T>::new_instruction(x)`.
    pub fn new_instruction<I>(i: I) -> Self
    where
        I: Into<T>,
    {
        Self::Instruction(i.into())
    }
}

impl<I> PushProgram<I>
where
    I: NumOpens,
{
    // Take a vector of genes, parse out the next complete Push program and
    // return that program and the remaining slice of genes.
    fn parse_from_plushy(
        is_top_level: bool,
        genes: &mut impl Iterator<Item = PushGene<I>>,
        program: &mut Vec<Self>,
    ) {
        while let Some(gene) = genes.next() {
            match gene {
                PushGene::Close => {
                    if !is_top_level {
                        // This closes a block, so return up to the caller.
                        return;
                    } // Otherwise ignore the `Close` and continue on to the next instruction
                }
                PushGene::Instruction(i) => {
                    let num_opens = i.num_opens();
                    program.push(Self::Instruction(i));
                    for _ in 0..num_opens {
                        let mut block = Vec::new();
                        Self::parse_from_plushy(false, genes, &mut block);
                        program.push(Self::Block(block));
                    }
                }
            }
        }
    }
}

// This is for "performing" an instruction that is in
// fact a block of instructions. To perform this instruction
// we need to push all the instructions in the block onto
// the stack in the correct order, i.e., the first instruction
// in the block should be the top instruction on the exec
// stack after all the pushing is done.
// TODO: Revisit this after genericizing `PushState` to see if this needs
// updating.
impl<S, I> Instruction<S> for Vec<I>
where
    S: HasStack<I>,
    I: Instruction<S> + Clone,
    I::Error: From<StackError>,
{
    type Error = I::Error;

    fn perform(&self, mut state: S) -> InstructionResult<S, Self::Error> {
        // If the size of the block + the size of the exec stack exceed the
        // max stack size then we generate a fatal error.
        if let Err(err) = state.stack_mut::<I>().push_many(self.iter().cloned()) {
            return Err(Error::fatal(state, err));
        }
        Ok(state)
    }
}

// TODO: Revisit this after genericizing `PushState` to see if this can be
// made more generic as well.
impl Instruction<PushState> for PushProgram {
    type Error = PushInstructionError;

    fn perform(&self, state: PushState) -> InstructionResult<PushState, Self::Error> {
        match self {
            Self::Instruction(i) => i.perform(state),
            Self::Block(block) => block.perform(state),
        }
    }
}

#[cfg(test)]
mod test {
    use std::iter::once;

    use super::PushProgram;
    use crate::{
        genome::plushy::{Plushy, PushGene},
        instruction::{
            BoolInstruction, ExecInstruction, FloatInstruction, Instruction, IntInstruction,
            NumOpens, PushInstruction,
        },
        list_into::arr_into,
        push_vm::{HasStack, push_state::PushState},
        test_utils::p,
    };

    #[test]
    fn new_instruction() {
        assert_eq!(
            PushProgram::<u32>::new_instruction(1u16),
            PushProgram::Instruction(1)
        );
        assert_eq!(
            PushProgram::<PushInstruction>::new_instruction(IntInstruction::Add),
            PushProgram::Instruction(IntInstruction::Add.into())
        );
    }

    #[test]
    fn conversion() {
        let genes = arr_into![
            PushGene::new_instruction(IntInstruction::Add),
            PushGene::new_instruction(ExecInstruction::if_else()),
            PushGene::new_instruction(IntInstruction::Multiply),
            PushGene::Close,
            PushGene::new_instruction(ExecInstruction::dup_block()),
            PushGene::new_instruction(IntInstruction::Subtract),
        ];
        let plushy: Plushy = genes.into_iter().collect();
        let program: Vec<PushProgram> = plushy.into();
        // [Instruction(Int-Add), Instruction(Exec-IfElse),
        // Block([Instruction(Int-Multiply)]), Block([Instruction(Exec-Dup),
        // Block([Instruction(Int-Subtract)])])]
        assert_eq!(
            program,
            vec![
                p(IntInstruction::Add),
                p(ExecInstruction::if_else()),
                PushProgram::Block(vec![p(IntInstruction::Multiply)]),
                PushProgram::Block(vec![
                    p(ExecInstruction::dup_block()),
                    PushProgram::Block(vec![p(IntInstruction::Subtract)])
                ])
            ]
        );
    }

    #[test]
    fn block() {
        let instructions = vec![
            p(IntInstruction::Add),
            p(FloatInstruction::Multiply),
            p(BoolInstruction::And),
        ];
        let block = PushProgram::Block(instructions);
        let state = PushState::builder()
            .with_max_stack_size(3)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();
        let mut result = block.perform(state).unwrap();
        let exec_stack = result.stack_mut::<PushProgram>();
        assert_eq!(exec_stack.size(), 3);
        assert_eq!(exec_stack.pop().unwrap(), p(IntInstruction::Add));
        assert_eq!(exec_stack.pop().unwrap(), p(FloatInstruction::Multiply));
        assert_eq!(exec_stack.pop().unwrap(), p(BoolInstruction::And));
        assert_eq!(exec_stack.size(), 0);
    }

    #[test]
    fn block_overflows() {
        let instructions = vec![
            p(IntInstruction::Add),
            p(FloatInstruction::Multiply),
            p(BoolInstruction::And),
        ];
        let block = PushProgram::Block(instructions);
        let state = PushState::builder()
            // Set the max stack size to 2, so when we execute the block it overflows
            .with_max_stack_size(0)
            .with_no_program()
            .with_instruction_step_limit(1000)
            .build();

        assert!(
            block.perform(state).is_err(),
            "Performing the block didn't generate an overflow error"
        );
    }

    #[test]
    fn custom_instruction() {
        #[derive(Debug, PartialEq, Eq)]
        enum CustomInstruction {
            A,
            B,
            HasOpen,
        }

        impl NumOpens for CustomInstruction {
            fn num_opens(&self) -> usize {
                match self {
                    Self::A | Self::B => 0,
                    Self::HasOpen => 1,
                }
            }
        }

        fn p(i: impl Into<CustomInstruction>) -> PushProgram<CustomInstruction> {
            PushProgram::Instruction(i.into())
        }

        let plushy: Plushy<CustomInstruction> = [
            CustomInstruction::A,
            CustomInstruction::B,
            CustomInstruction::HasOpen,
            CustomInstruction::A,
        ]
        .into_iter()
        .map(PushGene::Instruction)
        .chain(once(PushGene::Close))
        .chain(once(PushGene::Instruction(CustomInstruction::B)))
        .collect();
        let program: Vec<PushProgram<CustomInstruction>> = plushy.into();
        assert_eq!(
            program,
            [
                p(CustomInstruction::A),
                p(CustomInstruction::B),
                p(CustomInstruction::HasOpen),
                PushProgram::Block(vec![p(CustomInstruction::A)]),
                p(CustomInstruction::B)
            ]
        );
    }
}
