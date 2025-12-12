use std::{fmt::Display, ops::Range};

use ec_core::{
    distributions::collection::{self, ConvertToCollectionDistribution},
    genome::Genome,
};
use rand::{
    Rng,
    distr::{Bernoulli, BernoulliError, StandardUniform},
    prelude::Distribution,
};

use super::Linear;
use crate::recombinator::{crossover::Crossover, errors::MultipleGeneAccess};

/// A linear, variable-length genome of true/false values.
///
/// # Example
/// ```
/// # use ec_linear::genome::bitstring::Bitstring;
/// # use rand::rng;
/// #
/// let my_bitstring = Bitstring::random(10, &mut rng());
/// assert_eq!(my_bitstring.iter().count(), 10)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bitstring {
    pub bits: Vec<bool>,
}

impl<BD> Distribution<Bitstring> for collection::Collection<BD>
where
    BD: Distribution<bool>,
{
    /// Sample a [`Bitstring`] from a collection generator
    ///
    /// This creates a random [`Bitstring`] of the size as specified in the
    /// generator.
    ///
    /// # Example
    /// ```
    /// # use ec_linear::genome::bitstring::Bitstring;
    /// # use ec_core::distributions::collection::ConvertToCollectionDistribution;
    /// # use rand::{rng, distr::{Bernoulli, Distribution}};
    /// #
    /// let my_bitstring: Bitstring = Bernoulli::new(0.3)?.into_collection(10).sample(&mut rng());
    /// assert_eq!(my_bitstring.iter().count(), 10);
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Bitstring {
        Bitstring {
            bits: self.sample(rng),
        }
    }
}

impl Bitstring {
    /// Create a new random bitstring genome of size `num_bits`.
    ///
    /// # Example
    /// ```
    /// # use ec_linear::genome::bitstring::Bitstring;
    /// # use rand::rng;
    /// #
    /// let my_bitstring = Bitstring::random(10, &mut rng());
    /// assert_eq!(my_bitstring.iter().count(), 10)
    /// ```
    pub fn random<R: Rng + ?Sized>(num_bits: usize, rng: &mut R) -> Self {
        StandardUniform.into_collection(num_bits).sample(rng)
    }

    /// Create a new random bitstring genome of size `num_bits` with a given
    /// `probability` of each gene beeing true.
    ///
    /// # Example
    /// ```
    /// # use ec_linear::genome::bitstring::Bitstring;
    /// # use rand::rng;
    /// #
    /// let my_bitstring = Bitstring::random(10, &mut rng());
    /// assert_eq!(my_bitstring.iter().count(), 10)
    /// ```
    ///
    /// # Errors
    /// - [`BernoulliError`] if the passed probability is invalid. (i.e not in
    ///   range 0.0..=1.0)
    pub fn random_with_probability<R: Rng + ?Sized>(
        num_bits: usize,
        probability: f64,
        rng: &mut R,
    ) -> Result<Self, BernoulliError> {
        Ok(Bernoulli::new(probability)?
            .into_collection(num_bits)
            .sample(rng))
    }

    /// Iterate over references to the individual genes of this genome
    ///
    /// # Example
    /// ```
    /// # use ec_linear::genome::bitstring::Bitstring;
    /// # use rand::rng;
    /// #
    /// let my_bitstring = Bitstring::random(10, &mut rng());
    /// assert_eq!(my_bitstring.iter().count(), 10)
    /// ```
    pub fn iter(&self) -> std::slice::Iter<'_, bool> {
        self.bits.iter()
    }

    /// Iterate over mutable references to the individual genes of this genome
    ///
    /// # Example
    /// ```
    /// # use ec_linear::genome::bitstring::Bitstring;
    /// # use rand::rng;
    /// #
    /// let mut my_bitstring = Bitstring::random(10, &mut rng());
    ///
    /// for gene in my_bitstring.iter_mut() {
    ///     *gene = true;
    /// }
    ///
    /// assert!(my_bitstring.iter().all(|&v| v))
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, bool> {
        self.bits.iter_mut()
    }
}

/// Display a bitstring as individual 0/1 bits.
///
/// # Example
/// ```
/// # use ec_linear::genome::bitstring::Bitstring;
/// #
/// let ones_bitstring = Bitstring::from_iter([true; 10]);
///
/// assert_eq!(format!("{ones_bitstring}"), "1111111111".to_string())
/// ```
impl Display for Bitstring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in &self.bits {
            write!(f, "{}", u8::from(*bit))?;
        }
        Ok(())
    }
}

// TODO: Should we use copy-on-write when we clone genomes after selection?

/// Construct a bitstring genome from a iterator of [`bool`]'s
///
/// # Example
/// ```
/// # use ec_linear::genome::bitstring::Bitstring;
/// #
/// let ones_bitstring = Bitstring::from_iter([true; 10]);
///
/// assert_eq!(format!("{ones_bitstring}"), "1111111111".to_string())
/// ```
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
    /// The bitstring genome has [`bool`] genes
    type Gene = bool;
}

impl Linear for Bitstring {
    /// Get the size of this bitstring genome
    ///
    /// # Example
    /// ```
    /// # use ec_linear::genome::{bitstring::Bitstring, Linear};
    /// # use rand::rng;
    /// #
    /// let my_bitstring = Bitstring::random(10, &mut rng());
    /// assert_eq!(my_bitstring.size(), 10);
    /// ```
    fn size(&self) -> usize {
        self.bits.len()
    }

    /// Get a mutable reference to the gene at index `index`.
    ///
    /// # Example
    /// ```
    /// # use ec_linear::genome::{bitstring::Bitstring, Linear};
    /// # use rand::rng;
    /// #
    /// # fn foo() -> Option<()> {
    /// #
    /// let mut my_bitstring = Bitstring::random(10, &mut rng());
    /// *my_bitstring.gene_mut(2)? = true;
    /// assert_eq!(my_bitstring.iter().nth(2), Some(&true));
    /// #
    /// # Some(())
    /// # }
    /// # foo().unwrap();
    /// ```
    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.bits.get_mut(index)
    }
}

impl Crossover for Bitstring {
    type GeneCrossoverError = MultipleGeneAccess<usize>;

    /// Crossover a single gene at index `index` with a second [`Bitstring`]
    /// `other`.
    ///
    ///  # Example
    /// ```
    /// # use ec_linear::{genome::bitstring::Bitstring, recombinator::crossover::Crossover};
    /// # use rand::rng;
    /// let mut my_bitstring_1 = Bitstring::random(10, &mut rng());
    /// let mut my_bitstring_2 = Bitstring::random(10, &mut rng());
    ///
    /// my_bitstring_1.crossover_gene(&mut my_bitstring_2, 5)?;
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn crossover_gene(
        &mut self,
        other: &mut Self,
        index: usize,
    ) -> Result<(), Self::GeneCrossoverError> {
        self.bits
            .crossover_gene(&mut other.bits, index)
            .map_err(MultipleGeneAccess::for_genome_type::<Self>)
    }

    type SegmentCrossoverError = MultipleGeneAccess<Range<usize>>;

    /// Crossover a segment of genes at the indecies in `range` with a second
    /// [`Bitstring`] `other`.
    ///
    ///  # Example
    /// ```
    /// # use ec_linear::{genome::bitstring::Bitstring, recombinator::crossover::Crossover};
    /// # use rand::rng;
    /// let mut my_bitstring_1 = Bitstring::random(10, &mut rng());
    /// let mut my_bitstring_2 = Bitstring::random(10, &mut rng());
    ///
    /// my_bitstring_1.crossover_segment(&mut my_bitstring_2, 5..8)?;
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn crossover_segment(
        &mut self,
        other: &mut Self,
        range: Range<usize>,
    ) -> Result<(), Self::SegmentCrossoverError> {
        self.bits
            .crossover_segment(&mut other.bits, range)
            .map_err(MultipleGeneAccess::for_genome_type::<Self>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recombinator::{crossover::Crossover, errors::MultipleGeneAccess};

    #[test]
    fn display_format() {
        let bs: Bitstring = [true, false, true, true, false].into_iter().collect();
        assert_eq!(bs.to_string(), "10110");
    }

    #[test]
    fn from_iterator() {
        let bits = vec![true, false, true];
        let bs: Bitstring = bits.clone().into_iter().collect();
        assert_eq!(bs.bits, bits);
    }

    #[test]
    fn linear_trait_impl() {
        let mut bs: Bitstring = [true, false, true].into_iter().collect();
        assert_eq!(bs.size(), 3);

        let gene = bs.gene_mut(1);
        assert_eq!(gene, Some(&mut false));
        if let Some(g) = gene {
            *g = true;
        }
        assert_eq!(bs.gene_mut(1), Some(&mut true));

        assert_eq!(bs.gene_mut(5), None);
    }

    #[test]
    fn random_with_probability_all_true() {
        let mut rng = rand::rng();
        let size = 100;
        let bs = Bitstring::random_with_probability(size, 1.0, &mut rng).unwrap();
        assert_eq!(bs.size(), size);
        assert!(bs.iter().all(|&b| b));
    }

    #[test]
    fn random_with_probability_all_false() {
        let mut rng = rand::rng();
        let size = 100;
        let bs = Bitstring::random_with_probability(size, 0.0, &mut rng).unwrap();
        assert_eq!(bs.size(), size);
        assert!(bs.iter().all(|&b| !b));
    }

    // --- Crossover Tests ---

    #[test]
    fn crossover_gene_success() {
        let mut first: Bitstring = [true, false, true].into_iter().collect();
        let mut second: Bitstring = [false, true, false].into_iter().collect();

        let result = first.crossover_gene(&mut second, 1);

        assert!(result.is_ok());
        assert_eq!(first.bits, vec![true, true, true]);
        assert_eq!(second.bits, vec![false, false, false]);
    }

    #[test]
    fn crossover_segment_success() {
        let mut first: Bitstring = [true, false, true, false, true].into_iter().collect();
        let mut second: Bitstring = [false, true, false, true, false].into_iter().collect();

        let result = first.crossover_segment(&mut second, 1..4);

        assert!(result.is_ok());
        assert_eq!(first.bits, vec![true, true, false, true, true]);
        assert_eq!(second.bits, vec![false, false, true, false, false]);
    }

    #[test]
    fn gene_access_error_both() {
        let size = 20;
        let mut first = Bitstring::from_iter(vec![false; size]);
        let mut second = Bitstring::from_iter(vec![false; size]);

        let err = first.crossover_gene(&mut second, size).unwrap_err();
        if let MultipleGeneAccess::Both { lhs, rhs } = err {
            assert_eq!(lhs.index, size);
            assert_eq!(lhs.size, size);
            assert!(format!("{lhs}").contains("Bitstring"));

            assert_eq!(rhs.index, size);
            assert_eq!(rhs.size, size);
            assert!(format!("{rhs}").contains("Bitstring"));
        } else {
            panic!("Expected `MultipleGeneAccess::Both`, got {err:?}");
        }
    }

    #[test]
    fn gene_access_error_lhs() {
        let first_size = 20;
        let second_size = 30;
        let mut first = Bitstring::from_iter(vec![false; first_size]);
        let mut second = Bitstring::from_iter(vec![false; second_size]);

        let err = first.crossover_gene(&mut second, first_size).unwrap_err();
        if let MultipleGeneAccess::Lhs(lhs) = err {
            assert_eq!(lhs.index, first_size);
            assert_eq!(lhs.size, first_size);
            assert!(format!("{lhs}").contains("Bitstring"));
        } else {
            panic!("Expected `MultipleGeneAccess::Lhs`, got {err:?}");
        }
    }

    #[test]
    fn gene_access_error_rhs() {
        let first_size = 30;
        let second_size = 20;
        let mut first = Bitstring::from_iter(vec![false; first_size]);
        let mut second = Bitstring::from_iter(vec![false; second_size]);

        let err = first.crossover_gene(&mut second, second_size).unwrap_err();
        if let MultipleGeneAccess::Rhs(rhs) = err {
            assert_eq!(rhs.index, second_size);
            assert_eq!(rhs.size, second_size);
            assert!(format!("{rhs}").contains("Bitstring"));
        } else {
            panic!("Expected `MultipleGeneAccess::Rhs`, got {err:?}");
        }
    }

    #[test]
    fn segment_access_error_both() {
        let size = 20;
        let mut first = Bitstring::from_iter(vec![false; size]);
        let mut second = Bitstring::from_iter(vec![false; size]);
        let range = 15..25;

        let err = first
            .crossover_segment(&mut second, range.clone())
            .unwrap_err();

        if let MultipleGeneAccess::Both { lhs, rhs } = err {
            assert_eq!(lhs.index, range);
            assert_eq!(lhs.size, size);
            assert!(format!("{lhs}").contains("Bitstring"));

            assert_eq!(rhs.index, range);
            assert_eq!(rhs.size, size);
            assert!(format!("{rhs}").contains("Bitstring"));
        } else {
            panic!("Expected MultipleGeneAccess::Both, got {err:?}");
        }
    }

    #[test]
    fn segment_access_error_lhs() {
        let first_size = 20;
        let second_size = 30;
        let mut first = Bitstring::from_iter(vec![false; first_size]);
        let mut second = Bitstring::from_iter(vec![false; second_size]);
        let range = 15..25;

        let err = first
            .crossover_segment(&mut second, range.clone())
            .unwrap_err();

        if let MultipleGeneAccess::Lhs(lhs) = err {
            assert_eq!(lhs.index, range);
            assert_eq!(lhs.size, first_size);
            assert!(format!("{lhs}").contains("Bitstring"));
        } else {
            panic!("Expected MultipleGeneAccess::Lhs, got {err:?}");
        }
    }

    #[test]
    fn segment_access_error_rhs() {
        let first_size = 30;
        let second_size = 20;
        let mut first = Bitstring::from_iter(vec![false; first_size]);
        let mut second = Bitstring::from_iter(vec![false; second_size]);
        let range = 15..25;

        let err = first
            .crossover_segment(&mut second, range.clone())
            .unwrap_err();

        if let MultipleGeneAccess::Rhs(rhs) = err {
            assert_eq!(rhs.index, range);
            assert_eq!(rhs.size, second_size);
            assert!(format!("{rhs}").contains("Bitstring"));
        } else {
            panic!("Expected MultipleGeneAccess::Rhs, got {err:?}");
        }
    }
}
