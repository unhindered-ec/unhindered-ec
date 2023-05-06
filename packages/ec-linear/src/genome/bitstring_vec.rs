use ec_core::test_results::{self, TestResults};

// Should we keep (something like) this and have it
// implement `LinearGenome` and `Crossover` for `Vec<T>`?
// That might prove useful in some cases? If nothing else
// we can then pass through operations like mutation and
// crossover in `Bitstring` to the underlying vectors of
// values.
//
// Or, as the comment in `LinearGenome` suggests, we just
// make that generic in `T`, and then `Bitstring` becomes
// `LinearGenome<bool>`

// TODO:
//   We need an `impl Display for Bitstring` when we
//   have that in a `struct`.
#[deprecated(note = "Use `Bitstring` struct instead")]
pub type BitstringVecType = Vec<bool>;

#[must_use]
pub fn count_ones(bits: &[bool]) -> TestResults<test_results::Score> {
    bits.iter().map(|bit| i64::from(*bit)).map(Into::into).sum()
}

#[cfg(test)]
mod test_count_ones {
    use ec_core::test_results;

    use super::count_ones;

    #[test]
    fn empty() {
        let empty_vec: Vec<test_results::Score> = Vec::new();
        assert_eq!(empty_vec, count_ones(&[]).results);
    }

    #[test]
    fn non_empty() {
        let input = [false, true, true, true, false, true];
        let output: Vec<test_results::Score> =
            vec![0, 1, 1, 1, 0, 1].into_iter().map(Into::into).collect();
        assert_eq!(output, count_ones(&input).results);
    }
}

#[must_use]
pub fn hiff(bits: &[bool]) -> TestResults<test_results::Score> {
    let num_scores = 2 * bits.len() - 1;
    let mut scores = Vec::with_capacity(num_scores);
    do_hiff(bits, &mut scores);
    scores.into()
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
