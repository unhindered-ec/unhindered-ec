use std::fmt::Display;

use easy_cast::ConvApprox;
use ec_core::{
    distributions::{collection, finite::Finite},
    genome::Genome,
};
use ec_linear::genome::Linear;
use rand::{Rng, RngExt, prelude::Distribution};

use crate::instruction::NumOpens;

/// A gene in a [`Plushy`] genome: either a `Close` marker that closes a block
/// or an `Instruction`.
///
/// A gene can be built from an instruction with [`PushGene::new_instruction`]
/// or, when the instruction type already matches, with `From`/`Into`.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum PushGene<I> {
    Close,
    Instruction(I),
}

impl<I> Display for PushGene<I>
where
    I: Display + NumOpens,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Close => {
                f.write_str("}")?;
            }
            Self::Instruction(i) => {
                i.fmt(f)?;

                for bracket in std::iter::repeat_n(" {", i.num_opens()) {
                    f.write_str(bracket)?;
                }
            }
        }

        Ok(())
    }
}

impl<I> From<I> for PushGene<I> {
    fn from(i: I) -> Self {
        Self::Instruction(i)
    }
}

impl<T> PushGene<T> {
    /// Create a gene wrapping `i` as its instruction, converting `i` to the
    /// gene's instruction type `T` via `Into`.
    ///
    /// The surrounding context usually determines `T`. When it doesn't, as in
    /// a bare `PushGene::new_instruction(x)`, specify it explicitly with
    /// `PushGene::<T>::new_instruction(x)`.
    pub fn new_instruction<I>(i: I) -> Self
    where
        I: Into<T>,
    {
        Self::Instruction(i.into())
    }
}

#[derive(Debug, Clone)]
pub struct GeneGenerator<T> {
    close_probability: f32,
    instruction_distribution: T,
}

impl<T> GeneGenerator<T> {
    #[must_use]
    pub const fn new(close_probability: f32, instructions_distribution: T) -> Self {
        Self {
            close_probability,
            instruction_distribution: instructions_distribution,
        }
    }
}
impl<T> GeneGenerator<T>
where
    T: Finite,
{
    /// Create a generator where the close tag has the same likelihood of
    /// being chosen as any of the passed in instructions.
    pub fn with_uniform_close_probability(instructions_distribution: T) -> Self {
        Self::new(
            1.0 / f32::conv_approx(
                instructions_distribution
                    .sample_space_size()
                    .get()
                    .saturating_add(1),
            ),
            instructions_distribution,
        )
    }
}

pub trait ConvertToGeneGenerator {
    fn into_gene_generator_with_close_probability(
        self,
        close_probability: f32,
    ) -> GeneGenerator<Self>
    where
        Self: Sized;

    fn to_gene_generator_with_close_probability(
        &self,
        close_probability: f32,
    ) -> GeneGenerator<&Self>;

    /// This creates a new gene generator, defaulting to a close probability
    /// that is uniform with the instructions distribution, eg. (1/(n+1)).
    fn into_gene_generator(self) -> GeneGenerator<Self>
    where
        Self: Sized + Finite;

    /// This creates a new gene generator by borrowing from self, defaulting to
    /// a close probability that is uniform with the instructions distribution,
    /// eg. (1/(n+1)).
    fn to_gene_generator(&self) -> GeneGenerator<&Self>
    where
        Self: Finite;
}

impl<T> ConvertToGeneGenerator for T
where
    T: ?Sized,
{
    fn into_gene_generator_with_close_probability(
        self,
        close_probability: f32,
    ) -> GeneGenerator<Self>
    where
        Self: Sized,
    {
        GeneGenerator::new(close_probability, self)
    }

    fn to_gene_generator_with_close_probability(
        &self,
        close_probability: f32,
    ) -> GeneGenerator<&Self> {
        GeneGenerator::new(close_probability, self)
    }

    fn into_gene_generator(self) -> GeneGenerator<Self>
    where
        Self: Sized + Finite,
    {
        GeneGenerator::with_uniform_close_probability(self)
    }

    fn to_gene_generator(&self) -> GeneGenerator<&Self>
    where
        Self: Finite,
    {
        GeneGenerator::with_uniform_close_probability(self)
    }
}

impl<I, T> Distribution<PushGene<I>> for GeneGenerator<T>
where
    T: Distribution<I>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PushGene<I> {
        if rng.random::<f32>() < self.close_probability {
            PushGene::Close
        } else {
            // this is safe since we check that the slice is not empty in the
            // constructor
            PushGene::Instruction(self.instruction_distribution.sample(rng))
        }
    }
}

/// A linear Push genome: an ordered sequence of [`PushGene`]s.
///
/// `Plushy` is generic over its instruction type, defaulting to
/// [`PushInstruction`]. A `Plushy<I>` can be constructed, printed, mutated,
/// and converted into a `Vec<PushProgram<I>>`, but only
/// `Plushy<PushInstruction>` (the default) can be *executed*; see
/// [`PushProgram`](crate::push_vm::program::PushProgram).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Plushy<I> {
    genes: Vec<PushGene<I>>,
}

impl<I> Display for Plushy<I>
where
    I: Display + NumOpens,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.genes.iter();
        if let Some(gene) = iter.next() {
            gene.fmt(f)?;
        }

        for gene in iter {
            f.write_str(" ")?;
            gene.fmt(f)?;
        }

        Ok(())
    }
}

// TODO: `Plushy` implements `IntoIterator` (consuming) and `from_instructions`,
// but `get_genes` is the only non-consuming accessor and it clones. Consider
// whether a borrowed `to_instructions()` (or an iterator over `&PushGene<I>`)
// is worth adding.
impl<I> Plushy<I> {
    /// Create a plushy from an iterator of [`PushGene`]s.
    pub fn new(iterable: impl IntoIterator<Item = PushGene<I>>) -> Self {
        Self {
            genes: iterable.into_iter().collect(),
        }
    }

    /// Create a plushy from an iterator of instructions, wrapping each one in
    /// a [`PushGene::Instruction`].
    ///
    /// This does not convert between instruction types: the plushy's
    /// instruction type is the item type `I`. When the items need converting
    /// (e.g. from a concrete instruction into [`PushInstruction`]), build the
    /// genes with [`PushGene::new_instruction`] instead.
    pub fn from_instructions(iterable: impl IntoIterator<Item = I>) -> Self {
        Self {
            genes: iterable.into_iter().map(PushGene::Instruction).collect(),
        }
    }
}

impl<I: Clone> Plushy<I> {
    #[must_use]
    pub fn get_genes(&self) -> Vec<PushGene<I>> {
        self.genes.clone()
    }
}

impl<I> Genome for Plushy<I> {
    type Gene = PushGene<I>;
}

impl<I> Linear for Plushy<I> {
    fn size(&self) -> usize {
        self.genes.len()
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.genes.get_mut(index)
    }
}

impl<I, GG> Distribution<Plushy<I>> for collection::Collection<GG>
where
    GG: Distribution<PushGene<I>>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Plushy<I> {
        Plushy {
            genes: rng.sample(self),
        }
    }
}

impl<I> IntoIterator for Plushy<I> {
    type Item = PushGene<I>;

    type IntoIter = std::vec::IntoIter<PushGene<I>>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

impl<I> FromIterator<PushGene<I>> for Plushy<I> {
    fn from_iter<T: IntoIterator<Item = PushGene<I>>>(iterable: T) -> Self {
        Self {
            genes: iterable.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use ec_core::{
        distributions::collection::ConvertToCollectionDistribution, operator::mutator::Mutator,
        uniform_distribution_of,
    };
    use ec_linear::mutator::umad::Umad;
    use rand::rng;

    use super::*;
    use crate::{
        instruction::{
            BoolInstruction, IntInstruction, PushInstruction, with_input::WithInputInstruction,
        },
        list_into::{arr_into, genes_into},
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum MyInstruction {
        Add,
        Block,
    }

    impl Display for MyInstruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                Self::Add => "Add",
                Self::Block => "Block",
            })
        }
    }

    impl NumOpens for MyInstruction {
        fn num_opens(&self) -> usize {
            match self {
                Self::Add => 0,
                Self::Block => 1,
            }
        }
    }

    #[test]
    fn generator() {
        let mut rng = rng();
        let plushy: Plushy<_> = uniform_distribution_of![<PushInstruction>
            IntInstruction::Add,
            IntInstruction::Subtract,
            IntInstruction::Multiply,
            IntInstruction::ProtectedDivide,
        ]
        .into_gene_generator()
        .into_collection(10)
        .sample(&mut rng);

        assert_eq!(10, plushy.genes.len());
    }

    #[test]
    fn generic_gene_generator() {
        let mut rng = rng();
        let plushy: Plushy<MyInstruction> = uniform_distribution_of![<MyInstruction>
            MyInstruction::Add,
            MyInstruction::Block,
        ]
        .into_gene_generator()
        .into_collection(10)
        .sample(&mut rng);

        assert_eq!(10, plushy.size());
    }

    #[test]
    fn generic_plushy_display() {
        let plushy = Plushy::<MyInstruction>::new([
            MyInstruction::Add.into(),
            MyInstruction::Block.into(),
            PushGene::Close,
        ]);

        assert_eq!("Add Block { }", plushy.to_string());
    }

    #[test]
    fn from_instructions() {
        let plushy = Plushy::from_instructions(arr_into![<PushInstruction>
            IntInstruction::Add,
            BoolInstruction::And,
        ]);
        assert_eq!(
            plushy,
            Plushy::new([
                PushGene::new_instruction(IntInstruction::Add),
                PushGene::new_instruction(BoolInstruction::And),
            ])
        );
    }

    #[test]
    fn from_instructions_generic() {
        let plushy =
            Plushy::<MyInstruction>::from_instructions([MyInstruction::Add, MyInstruction::Block]);
        assert_eq!(plushy.size(), 2);
    }

    #[ignore = "this has about a 2.665% chance on failing at least once across the three test \
                runners in ci"]
    #[test]
    fn umad() {
        let mut rng = rng();

        let instruction_options = uniform_distribution_of![<PushGene<_>> PushInstruction::from(WithInputInstruction::from("x"))];

        let umad = Umad::new(0.3, 0.3, instruction_options);

        let parent = Plushy {
            genes: genes_into![<PushGene<_>>
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
                IntInstruction::Add,
                BoolInstruction::And,
                BoolInstruction::Or,
                IntInstruction::Multiply,
            ],
        };

        let child = umad.mutate(parent, &mut rng);

        let num_inputs = child
            .unwrap()
            .genes
            .iter()
            .filter(|c| matches!(c, PushGene::Instruction(PushInstruction::WithInput(v)) if v == &WithInputInstruction::from("x")))
            .count();
        assert!(
            num_inputs > 0,
            "Expected at least one input instruction to be added, but none were."
        );
    }

    // `Umad` on `Plushy` genomes is covered end-to-end with a custom
    // instruction type in `packages/push/tests/generic_instruction.rs`.
}
