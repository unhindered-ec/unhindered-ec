use std::borrow::Borrow;
use std::fmt::{Debug, Display};

use num_traits::ToPrimitive;
use rand::{rngs::ThreadRng, Rng};

use crate::individual::ec::EcIndividual;
use crate::individual::Generate as _;
use crate::individual::Individual;
use crate::population::{Generate as _};
use crate::test_results::TestResults;

pub type Bitstring = Vec<bool>;

pub fn make_random(len: usize, rng: &mut ThreadRng) -> Bitstring {
    (0..len).map(|_| rng.gen_bool(0.5)).collect()
}

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

impl<R> EcIndividual<Bitstring, R> {
    pub fn new_bitstring<H>(
        bit_length: usize,
        run_tests: impl Fn(&H) -> R,
        rng: &mut ThreadRng,
    ) -> Self
    where
        Bitstring: Borrow<H>,
        H: ?Sized,
    {
        Self::generate(|rng| make_random(bit_length, rng), run_tests, rng)
    }
}

// TODO: Maybe change R to implement `Display` and have `TestResults` have a
//   nice-ish display function.
impl<R: Debug> Display for EcIndividual<Bitstring, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for bit in self.genome() {
            if *bit {
                result.push('1');
            } else {
                result.push('0');
            }
        }
        write!(f, "[{}]\n{:?}", result, self.test_results())
    }
}

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
    Vec::generate(pop_size, |rng| make_random(bit_length, rng), run_tests)
}
