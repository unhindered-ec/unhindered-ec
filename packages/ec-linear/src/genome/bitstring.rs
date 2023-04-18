use std::fmt::Display;
use std::iter;
use std::{borrow::Borrow, slice::Iter};

use anyhow::bail;
use ec_core::gene::Genome;
use ec_core::individual::ec::EcIndividual;
use ec_core::population::Generate;
use ec_core::test_results::TestResults;
use rand::{rngs::ThreadRng, Rng};

use crate::recombinator::crossover::Crossover;

use super::LinearGenome;

// TODO: Ought to have `LinearGenome<T>` so that `Bitstring` is just
//   `LinearGenome<bool>`.

#[derive(Clone)]
pub struct Bitstring {
    pub bits: Vec<bool>,
}

impl Bitstring {
    pub fn random(len: usize, rng: &mut ThreadRng) -> Self {
        let bits = iter::repeat_with(|| rng.gen_bool(0.5)).take(len).collect();
        Bitstring { bits }
    }

    pub fn random_list(num_genomes: usize, len: usize, rng: &mut ThreadRng) -> Vec<Self> {
        iter::repeat_with(|| Self::random(len, rng))
            .take(num_genomes)
            .collect()
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

// Should we use copy-on-write when we clone genomes after selection?

// Are there other traits that we should implement for flexibility? I feel
// like esitsu has a "standard" list that they recommended several months ago.

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
    type Item = &'a bool; // <std::slice::Iter<'a, bool> as Iterator>::Item;
    type IntoIter = std::slice::Iter<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.bits.iter()
    }
}

impl<'a> IntoIterator for &'a mut Bitstring {
    type Item = &'a mut bool; // <std::slice::Iter<'a, bool> as Iterator>::Item;
    type IntoIter = std::slice::IterMut<'a, bool>;

    fn into_iter(self) -> Self::IntoIter {
        self.bits.iter_mut()
    }
}

impl Genome for Bitstring {
    type Gene = bool;
}

impl LinearGenome for Bitstring {
    fn size(&self) -> usize {
        self.bits.len()
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.bits.get_mut(index)
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

#[must_use]
pub fn fitness_vec_to_test_results(results: Vec<i64>) -> TestResults<i64> {
    let total_result = results.iter().sum();
    TestResults {
        total_result,
        results,
    }
}

// impl<R> EcIndividual<Bitstring, R> {
//     pub fn new_bitstring<H>(
//         bit_length: usize,
//         run_tests: impl Fn(&H) -> R,
//         rng: &mut ThreadRng,
//     ) -> Self
//     where
//         Bitstring: Borrow<H>,
//         H: ?Sized,
//     {
//         Self::generate(|rng| Bitstring::random(bit_length, rng), run_tests, rng)
//     }
// }

pub fn new_bitstring_population<R, H>(
    pop_size: usize,
    bit_length: usize,
    run_tests: impl Fn(&H) -> R + Send + Sync,
) -> Vec<EcIndividual<Bitstring, R>>
where
    R: Send,
    Bitstring: Borrow<H>,
    H: ?Sized,
{
    Vec::generate(
        pop_size,
        |rng| Bitstring::random(bit_length, rng),
        run_tests,
    )
}
