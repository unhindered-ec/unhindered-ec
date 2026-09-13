use super::{HasStack, push_state::PushState, stack::StackError};
use crate::{
    error::{Error, InstructionResult},
    genome::plushy::{GenericPlushy, GenericPushGene},
    instruction::{
        Instruction, NumOpens, PushInstruction, instruction_error::PushInstructionError,
    },
};

#[derive(Debug, strum_macros::Display, Clone, Eq, PartialEq)]
pub enum GenericPushProgram<I> {
    Instruction(I),
    Block(Vec<Self>),
}

pub type PushProgram = GenericPushProgram<PushInstruction>;

impl<I> Default for GenericPushProgram<I> {
    fn default() -> Self {
        Self::Block(Vec::new())
    }
}

impl<I> From<GenericPlushy<I>> for Vec<GenericPushProgram<I>>
where
    I: NumOpens,
{
    fn from(plushy: GenericPlushy<I>) -> Self {
        let mut genes = plushy.into_iter();
        let mut program = Self::new();
        GenericPushProgram::parse_from_plushy(true, &mut genes, &mut program);
        program
    }
}

impl<I> From<I> for GenericPushProgram<I> {
    fn from(instruction: I) -> Self {
        Self::Instruction(instruction)
    }
}

impl<I> GenericPushProgram<I>
where
    I: NumOpens,
{
    // Take a vector of genes, parse out the next complete Push program and
    // return that program and the remaining slice of genes.
    fn parse_from_plushy(
        is_top_level: bool,
        genes: &mut impl Iterator<Item = GenericPushGene<I>>,
        program: &mut Vec<Self>,
    ) {
        while let Some(gene) = genes.next() {
            match gene {
                GenericPushGene::Close => {
                    if !is_top_level {
                        // This closes a block, so return up to the caller.
                        return;
                    } // Otherwise ignore the `Close` and continue on to the next instruction
                }
                GenericPushGene::Instruction(i) => {
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

// TODO: Revisit this after genericizing `PushProgram` to see if this can be
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
    use super::PushProgram;
    use crate::{
        genome::plushy::{Plushy, PushGene},
        instruction::{
            BoolInstruction, ExecInstruction, FloatInstruction, Instruction, IntInstruction,
            PushInstruction,
        },
        list_into::arr_into,
        push_vm::{HasStack, program::GenericPushProgram, push_state::PushState},
    };

    fn p(i: impl Into<PushInstruction>) -> PushProgram {
        PushProgram::Instruction(i.into())
    }

    #[test]
    fn conversion() {
        fn i2g(i: impl Into<PushInstruction>) -> PushGene {
            PushGene::Instruction(i.into())
        }

        let genes = arr_into![
            i2g(IntInstruction::Add),
            i2g(ExecInstruction::if_else()),
            i2g(IntInstruction::Multiply),
            PushGene::Close,
            i2g(ExecInstruction::dup_block()),
            i2g(IntInstruction::Subtract),
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
                GenericPushProgram::Block(vec![p(IntInstruction::Multiply)]),
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
}
