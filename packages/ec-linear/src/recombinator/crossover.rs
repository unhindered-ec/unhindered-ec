use std::ops::Range;

use crate::{genome::Linear, recombinator::errors::MultipleGeneAccess};

// TODO: Does `Crossover` need to be visible outside
//   this module? If not, should `pub` be replaced/removed?
pub trait Crossover {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recombinator::errors::MultipleGeneAccess;

    // --- crossover_gene ---

    #[test]
    fn crossover_gene_success() {
        let mut v1 = vec![1, 2, 3, 4, 5];
        let mut v2 = vec![6, 7, 8, 9, 10];
        let original_v1 = v1.clone();
        let original_v2 = v2.clone();

        let result = v1.crossover_gene(&mut v2, 2);

        assert!(result.is_ok());
        assert_eq!(v1[2], original_v2[2]); // 8
        assert_eq!(v2[2], original_v1[2]); // 3

        // Check that other elements are untouched
        assert_eq!(&v1[0..2], &original_v1[0..2]);
        assert_eq!(&v1[3..], &original_v1[3..]);
        assert_eq!(&v2[0..2], &original_v2[0..2]);
        assert_eq!(&v2[3..], &original_v2[3..]);
    }

    #[test]
    fn crossover_gene_fails_lhs() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6, 7, 8];
        let index = 4;

        let err = v1.crossover_gene(&mut v2, index).unwrap_err();

        if let MultipleGeneAccess::Lhs(access) = err {
            assert_eq!(access.index, index);
            assert_eq!(access.size, v1.len());
        } else {
            panic!("Expected MultipleGeneAccess::Lhs, got {err:?}");
        }
    }

    #[test]
    fn crossover_gene_fails_rhs() {
        let mut v1 = vec![1, 2, 3, 4, 5];
        let mut v2 = vec![6, 7, 8];
        let index = 4;

        let err = v1.crossover_gene(&mut v2, index).unwrap_err();

        if let MultipleGeneAccess::Rhs(access) = err {
            assert_eq!(access.index, index);
            assert_eq!(access.size, v2.len());
        } else {
            panic!("Expected MultipleGeneAccess::Rhs, got {err:?}");
        }
    }

    #[test]
    fn crossover_gene_fails_both() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6];
        let index = 5;

        let err = v1.crossover_gene(&mut v2, index).unwrap_err();

        if let MultipleGeneAccess::Both { lhs, rhs } = err {
            assert_eq!(lhs.index, index);
            assert_eq!(lhs.size, v1.len());
            assert_eq!(rhs.index, index);
            assert_eq!(rhs.size, v2.len());
        } else {
            panic!("Expected MultipleGeneAccess::Both, got {err:?}");
        }
    }

    // --- crossover_segment ---

    #[test]
    fn crossover_segment_success() {
        let mut v1 = vec![1, 2, 3, 4, 5];
        let mut v2 = vec![6, 7, 8, 9, 10];
        let original_v1 = v1.clone();
        let original_v2 = v2.clone();
        let range = 1..4; // Swaps [2, 3, 4] with [7, 8, 9]

        let result = v1.crossover_segment(&mut v2, range.clone());

        assert!(result.is_ok());
        assert_eq!(&v1[range.clone()], &original_v2[range.clone()]);
        assert_eq!(&v2[range.clone()], &original_v1[range]);

        // Check that other elements are untouched
        assert_eq!(v1[0], original_v1[0]);
        assert_eq!(v1[4], original_v1[4]);
        assert_eq!(v2[0], original_v2[0]);
        assert_eq!(v2[4], original_v2[4]);
    }

    #[test]
    fn crossover_segment_fails_lhs() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6, 7, 8, 9];
        let range = 2..5;

        let err = v1.crossover_segment(&mut v2, range.clone()).unwrap_err();

        if let MultipleGeneAccess::Lhs(access) = err {
            assert_eq!(access.index, range);
            assert_eq!(access.size, v1.len());
        } else {
            panic!("Expected MultipleGeneAccess::Lhs, got {err:?}");
        }
    }

    #[test]
    fn crossover_segment_fails_rhs() {
        let mut v1 = vec![1, 2, 3, 4, 5, 6];
        let mut v2 = vec![7, 8, 9];
        let range = 2..5;

        let err = v1.crossover_segment(&mut v2, range.clone()).unwrap_err();

        if let MultipleGeneAccess::Rhs(access) = err {
            assert_eq!(access.index, range);
            assert_eq!(access.size, v2.len());
        } else {
            panic!("Expected MultipleGeneAccess::Rhs, got {err:?}");
        }
    }

    #[test]
    fn crossover_segment_fails_both() {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6];
        let range = 2..5;

        let err = v1.crossover_segment(&mut v2, range.clone()).unwrap_err();

        if let MultipleGeneAccess::Both { lhs, rhs } = err {
            assert_eq!(lhs.index, range);
            assert_eq!(lhs.size, v1.len());
            assert_eq!(rhs.index, range);
            assert_eq!(rhs.size, v2.len());
        } else {
            panic!("Expected MultipleGeneAccess::Both, got {err:?}");
        }
    }
}
