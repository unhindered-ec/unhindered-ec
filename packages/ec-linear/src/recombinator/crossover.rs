use std::{fmt::Debug, ops::Range, slice::SliceIndex};

use crate::{
    genome::Linear,
    recombinator::errors::{GeneAccess, MultipleGeneAccess},
};

// TODO: Does `Crossover` need to be visible outside
//   this module? If not, should `pub` be replaced/removed?
pub trait Crossover: Linear {
    /// Error that can happen when calling [`Crossover::crossover_gene`].
    type GeneCrossoverError;

    /// Error that can happen when calling [`Crossover::crossover_segment`].
    type SegmentCrossoverError;

    /// Swaps a gene at a randomly selected position, destructively
    /// modifying both this genome and `other`.
    ///
    /// # Errors
    /// This can fail if an attempt is made to crossover a gene at an index that
    /// is out of bounds for either this genome or `other`.
    fn crossover_gene(
        &mut self,
        other: &mut Self,
        index: usize,
    ) -> Result<(), Self::GeneCrossoverError>;

    /// Swaps a segment of this and the `other` genome that starts and
    /// ends at a randomly selected position. This is destructive, modifying
    /// both this genome and `other`.
    ///
    /// # Errors
    /// This can fail if an attempt is made to crossover a segments whose start
    /// and end are out of bounds for either this genome or `other`.
    fn crossover_segment(
        &mut self,
        other: &mut Self,
        range: Range<usize>,
    ) -> Result<(), Self::SegmentCrossoverError>;
}

pub(crate) fn try_get_mut<'a, 'b, I, Genome, Gene, ErrorGenome>(
    lhs: &'a mut Genome,
    rhs: &'b mut Genome,
    index: I,
) -> Result<(&'a mut I::Output, &'b mut I::Output), MultipleGeneAccess<I, ErrorGenome>>
where
    I: SliceIndex<[Gene]> + Debug + Clone,
    Genome: AsMut<[Gene]>,
    Gene: 'a + 'b,
{
    let (lhs, rhs) = (lhs.as_mut(), rhs.as_mut());
    let (lhs_size, rhs_size) = (lhs.len(), rhs.len());
    match (lhs.get_mut(index.clone()), rhs.get_mut(index.clone())) {
        (Some(lhs), Some(rhs)) => Ok((lhs, rhs)),
        (None, Some(_)) => Err(MultipleGeneAccess::Lhs(GeneAccess::new(index, lhs_size))),
        (Some(_), None) => Err(MultipleGeneAccess::Rhs(GeneAccess::new(index, rhs_size))),
        (None, None) => Err(MultipleGeneAccess::Both {
            lhs: GeneAccess::new(index.clone(), lhs_size),
            rhs: GeneAccess::new(index, rhs_size),
        }),
    }
}

impl<T: 'static> Crossover for Vec<T> {
    type GeneCrossoverError = MultipleGeneAccess<usize, Self>;

    fn crossover_gene(
        &mut self,
        other: &mut Self,
        index: usize,
    ) -> Result<(), Self::GeneCrossoverError> {
        let (lhs, rhs) = try_get_mut(self, other, index)?;

        std::mem::swap(lhs, rhs);
        Ok(())
    }

    type SegmentCrossoverError = MultipleGeneAccess<Range<usize>, Self>;

    fn crossover_segment(
        &mut self,
        other: &mut Self,
        range: Range<usize>,
    ) -> Result<(), Self::SegmentCrossoverError> {
        let (lhs, rhs) = try_get_mut(self, other, range)?;
        lhs.swap_with_slice(rhs);
        Ok(())
    }
}
