use std::fmt::Display;

use anyhow::bail;
use ec_core::{
    generator::{collection::CollectionGenerator, Generator},
    genome::Genome,
};
use rand::rngs::ThreadRng;

use crate::recombinator::crossover::Crossover;

use super::Linear;

// TODO: Ought to have `LinearGenome<T>` so that `Bitstring` is just
//   `LinearGenome<bool>`.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitstring {
    pub bits: Vec<bool>,
}

impl Generator<Bitstring> for CollectionGenerator<f64> {
    fn generate(&self, rng: &mut ThreadRng) -> anyhow::Result<Bitstring> {
        let bits = self.generate(rng)?;
        Ok(Bitstring { bits })
    }
}

impl Bitstring {
    /// # Errors
    ///
    /// This returns an `anyhow::Result<>` as required by the `Generate` trait. I shouldn't actually
    ///   ever return an error in this setting, as we should always be able to generate a vector of
    ///   booleans.
    /// TODO: I think that the `!` type could be used here to indicate that this can't fail, but that's
    ///   still experimental.
    pub fn random(num_bits: usize, rng: &mut ThreadRng) -> anyhow::Result<Self> {
        Self::random_with_probability(num_bits, 0.5, rng)
    }

    /// # Errors
    ///
    /// This returns an `anyhow::Result<>` as required by the `Generate` trait. I shouldn't actually
    ///   ever return an error in this setting, as we should always be able to generate a vector of
    ///   booleans.
    /// TODO: I think that the `!` type could be used here to indicate that this can't fail, but that's
    ///   still experimental.
    pub fn random_with_probability(
        num_bits: usize,
        probability: f64,
        rng: &mut ThreadRng,
    ) -> anyhow::Result<Self> {
        CollectionGenerator {
            size: num_bits,
            element_generator: probability,
        }
        .generate(rng)
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
    fn crossover_gene(&mut self, other: &mut Self, index: usize) -> anyhow::Result<()> {
        if let (Some(lhs), Some(rhs)) = (self.gene_mut(index), other.gene_mut(index)) {
            std::mem::swap(lhs, rhs);
            Ok(())
        } else {
            bail!("Crossing {self} and {other} at position {index} failed")
        }
    }

    fn crossover_segment(
        &mut self,
        other: &mut Self,
        range: std::ops::Range<usize>,
    ) -> anyhow::Result<()> {
        let lhs = &mut self.bits[range.clone()];
        let rhs = &mut other.bits[range.clone()];
        if lhs.len() == rhs.len() {
            lhs.swap_with_slice(rhs);
            Ok(())
        } else {
            bail!("Crossing {self} and {other} with range {range:?} failed")
        }
    }
}

// TODO: We need to move `count_ones` and `hiff` (and their tests)
//   out into their own module, and possibly their own package?

#[must_use]
pub fn count_ones(bits: &[bool]) -> Vec<i64> {
    bits.iter().map(|bit| i64::from(*bit)).collect()
}

#[cfg(test)]
mod test_count_ones {
    use super::count_ones;

    #[test]
    fn empty() {
        let empty_vec: Vec<i64> = Vec::new();
        assert_eq!(empty_vec, count_ones(&[]));
    }

    #[test]
    fn non_empty() {
        let input = [false, true, true, true, false, true];
        let output = vec![0, 1, 1, 1, 0, 1];
        assert_eq!(output, count_ones(&input));
    }
}

#[must_use]
pub fn hiff(bits: &[bool]) -> Vec<i64> {
    let num_scores = 2 * bits.len() - 1;
    let mut scores = Vec::with_capacity(num_scores);
    do_hiff(bits, &mut scores);
    scores
}

pub fn do_hiff(bits: &[bool], scores: &mut Vec<i64>) -> bool {
    let len = bits.len();
    if len < 2 {
        scores.push(len as i64);
        true
    } else {
        let half_len = len / 2;
        let left_all_same = do_hiff(&bits[..half_len], scores);
        let right_all_same = do_hiff(&bits[half_len..], scores);
        if left_all_same && right_all_same && bits[0] == bits[half_len] {
            scores.push(bits.len() as i64);
            true
        } else {
            scores.push(0);
            false
        }
    }
}
