use std::{fmt::Display, ops::Range};

use ec_core::{
    distributions::collection::{self, ConvertToCollectionDistribution},
    genome::Genome,
};
use rand::{Rng, distr::StandardUniform, prelude::Distribution};

use super::Linear;
use crate::recombinator::{crossover::Crossover, errors::MultipleGeneAccess};

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

impl<BG> Distribution<Bitstring> for collection::Collection<BG>
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
        StandardUniform.into_collection(num_bits).sample(rng)
    }

    pub fn random_with_probability<R: Rng + ?Sized>(
        num_bits: usize,
        probability: f64,
        rng: &mut R,
    ) -> Self {
        BoolGenerator::new(probability)
            .into_collection(num_bits)
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

impl Crossover for Bitstring {
    type GeneCrossoverError = MultipleGeneAccess<usize>;

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
        let bs = Bitstring::random_with_probability(size, 1.0, &mut rng);
        assert_eq!(bs.size(), size);
        assert!(bs.iter().all(|&b| b));
    }

    #[test]
    fn random_with_probability_all_false() {
        let mut rng = rand::rng();
        let size = 100;
        let bs = Bitstring::random_with_probability(size, 0.0, &mut rng);
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
