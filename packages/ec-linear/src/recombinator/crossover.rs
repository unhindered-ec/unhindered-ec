use std::ops::Range;

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

impl<T: 'static> Crossover for Vec<T> {
    type GeneCrossoverError = MultipleGeneAccess<usize, Self>;

    fn crossover_gene(
        &mut self,
        other: &mut Self,
        index: usize,
    ) -> Result<(), Self::GeneCrossoverError> {
        let (lhs, rhs) = match (self.gene_mut(index), other.gene_mut(index)) {
            (Some(lhs), Some(rhs)) => Ok((lhs, rhs)),
            (None, Some(_)) => Err(MultipleGeneAccess::Lhs(GeneAccess::new(index, self.size()))),
            (Some(_), None) => Err(MultipleGeneAccess::Rhs(GeneAccess::new(
                index,
                other.size(),
            ))),
            (None, None) => Err(MultipleGeneAccess::Both {
                lhs: GeneAccess::new(index, self.size()),
                rhs: GeneAccess::new(index, other.size()),
            }),
        }?;

        std::mem::swap(lhs, rhs);
        Ok(())
    }

    type SegmentCrossoverError = MultipleGeneAccess<Range<usize>, Self>;

    fn crossover_segment(
        &mut self,
        other: &mut Self,
        range: Range<usize>,
    ) -> Result<(), Self::SegmentCrossoverError> {
        let (lhs, rhs) = match (self.get_mut(range.clone()), other.get_mut(range.clone())) {
            (Some(lhs), Some(rhs)) => Ok((lhs, rhs)),
            (None, Some(_)) => Err(MultipleGeneAccess::Lhs(GeneAccess::new(range, self.size()))),
            (Some(_), None) => Err(MultipleGeneAccess::Rhs(GeneAccess::new(
                range,
                other.size(),
            ))),
            (None, None) => Err(MultipleGeneAccess::Both {
                lhs: GeneAccess::new(range.clone(), self.size()),
                rhs: GeneAccess::new(range, other.size()),
            }),
        }?;

        lhs.swap_with_slice(rhs);
        Ok(())
    }
}
