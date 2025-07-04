use std::fmt::Display;

use ec_core::{
    distributions::collection::{self, ConvertToCollectionGenerator},
    genome::Genome,
};
use miette::Diagnostic;
use rand::{Rng, distr::StandardUniform, prelude::Distribution};

use super::Linear;
use crate::recombinator::crossover::Crossover;

// TODO: Ought to have `LinearGenome<T>` so that `Bitstring` is just
//   `LinearGenome<bool>`.

pub struct BoolGenerator {
    pub true_probability: f64,
}

impl BoolGenerator {
    #[must_use]
    pub const fn new(true_probability: f64) -> Self {
        Self { true_probability }
    }
}

impl Distribution<bool> for BoolGenerator {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        rng.random_bool(self.true_probability)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitstring {
    pub bits: Vec<bool>,
}

impl<BG> Distribution<Bitstring> for collection::Generator<BG>
where
    BG: Distribution<bool>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Bitstring {
        Bitstring {
            bits: self.sample(rng),
        }
    }
}

impl Bitstring {
    pub fn random<R: Rng + ?Sized>(num_bits: usize, rng: &mut R) -> Self {
        StandardUniform
            .into_collection_generator(num_bits)
            .sample(rng)
    }

    pub fn random_with_probability<R: Rng + ?Sized>(
        num_bits: usize,
        probability: f64,
        rng: &mut R,
    ) -> Self {
        BoolGenerator::new(probability)
            .into_collection_generator(num_bits)
            .sample(rng)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, bool> {
        self.bits.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, bool> {
        self.bits.iter_mut()
    }
}

impl Display for Bitstring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in &self.bits {
            write!(f, "{}", u8::from(*bit))?;
        }
        Ok(())
    }
}

// TODO: Should we use copy-on-write when we clone genomes after selection?

impl<B> FromIterator<B> for Bitstring
where
    bool: From<B>,
{
    fn from_iter<T: IntoIterator<Item = B>>(iter: T) -> Self {
        Self {
            bits: iter.into_iter().map(From::from).collect(),
        }
    }
}

impl IntoIterator for Bitstring {
    type Item = bool;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.bits.into_iter()
    }
}

impl<'a> IntoIterator for &'a Bitstring {
    type Item = &'a bool;
    type IntoIter = std::slice::Iter<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.bits.iter()
    }
}

impl<'a> IntoIterator for &'a mut Bitstring {
    type Item = &'a mut bool;
    type IntoIter = std::slice::IterMut<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.bits.iter_mut()
    }
}

impl Genome for Bitstring {
    type Gene = bool;
}

impl Linear for Bitstring {
    fn size(&self) -> usize {
        self.bits.len()
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.bits.get_mut(index)
    }
}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Index {index} out of bounds for a bitstring of size {bitstring_size}")]
#[diagnostic(
    help = "Ensure that your indices are legal, i.e., at least zero and less than the size of the \
            bitstring"
)]
pub struct GeneAccess {
    index: usize,
    bitstring_size: usize,
}

#[derive(Debug, thiserror::Error, Diagnostic)]
#[error("Range {}..{} out of bounds for a bitstring of size {bitstring_size}", range.start, range.end)]
#[diagnostic(
    help("Ensure that your range bounds are legal, i.e., the start {} must be at least zero and \
            the end {} must be at most the size of the bitstring {bitstring_size}", range.start, range.end)
)]
pub struct GeneAccessRange {
    range: std::ops::Range<usize>,
    bitstring_size: usize,
}

impl Crossover for Bitstring {
    type GeneCrossoverError = GeneAccess;

    fn crossover_gene(
        &mut self,
        other: &mut Self,
        index: usize,
    ) -> Result<(), Self::GeneCrossoverError> {
        if let (Some(lhs), Some(rhs)) = (self.gene_mut(index), other.gene_mut(index)) {
            std::mem::swap(lhs, rhs);
            Ok(())
        } else {
            Err(GeneAccess {
                index,
                bitstring_size: self.size(),
            })
        }
    }

    type SegmentCrossoverError = GeneAccessRange;

    fn crossover_segment(
        &mut self,
        other: &mut Self,
        range: std::ops::Range<usize>,
    ) -> Result<(), Self::SegmentCrossoverError> {
        let lhs = &mut self.bits[range.clone()];
        let rhs = &mut other.bits[range.clone()];
        if lhs.len() == rhs.len() {
            lhs.swap_with_slice(rhs);
            Ok(())
        } else {
            Err(GeneAccessRange {
                range,
                bitstring_size: self.size(),
            })
        }
    }
}
