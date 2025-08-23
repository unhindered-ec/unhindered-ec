use std::ops::Range;

use crate::{genome::Linear, recombinator::errors::MultipleGeneAccess};

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
    type GeneCrossoverError = MultipleGeneAccess<usize>;

    fn crossover_gene(
        &mut self,
        other: &mut Self,
        index: usize,
    ) -> Result<(), Self::GeneCrossoverError> {
        match (self.gene_mut(index), other.gene_mut(index)) {
            (Some(lhs), Some(rhs)) => {
                std::mem::swap(lhs, rhs);
                Ok(())
            }
            (None, Some(_)) => Err(MultipleGeneAccess::lhs::<Self>(index, self.size())),
            (Some(_), None) => Err(MultipleGeneAccess::rhs::<Self>(index, other.size())),
            (None, None) => Err(MultipleGeneAccess::both::<Self>(
                index,
                self.size(),
                other.size(),
            )),
        }
    }

    type SegmentCrossoverError = MultipleGeneAccess<Range<usize>>;

    fn crossover_segment(
        &mut self,
        other: &mut Self,
        range: Range<usize>,
    ) -> Result<(), Self::SegmentCrossoverError> {
        match (self.get_mut(range.clone()), other.get_mut(range.clone())) {
            (Some(lhs), Some(rhs)) => {
                lhs.swap_with_slice(rhs);
                Ok(())
            }
            (None, Some(_)) => Err(MultipleGeneAccess::lhs::<Self>(range, self.size())),
            (Some(_), None) => Err(MultipleGeneAccess::rhs::<Self>(range, other.size())),
            (None, None) => Err(MultipleGeneAccess::both::<Self>(
                range,
                self.size(),
                other.size(),
            )),
        }
    }
}
