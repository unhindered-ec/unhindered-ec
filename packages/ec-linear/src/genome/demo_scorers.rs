use ec_core::test_results::{self, TestResults};

#[must_use]
pub fn count_ones(bits: &[bool]) -> TestResults<test_results::Score<i64>> {
    bits.iter().map(|bit| i64::from(*bit)).collect()
}

/// # Panics
///
/// This will panic if the length of the bitstring won't fit in an
/// `i64`. Since we're (currently) using an `i64` to store the score,
/// and since the largest possible score is the length of the bitstring,
/// it's necessary that the length fit in an `i64`. If it doesn't we'll
/// panic here.
#[must_use]
pub fn hiff(bits: &[bool]) -> TestResults<test_results::Score<i64>> {
    // Since the scores are represented as `i64`, and the largest
    // possible score is the length of `bits`, we need to know that
    // `bits.len()` can fit in an `i64`. If this is true we can use
    // `as` in `do_hiff` to freely convert between the length (`usize`)
    // and `i64`.
    assert!(
        i64::try_from(bits.len()).is_ok(),
        "The length of the bitstring {} won't fit in an `i64`.",
        bits.len()
    );
    let num_scores = 2 * bits.len() - 1;
    let mut scores = Vec::with_capacity(num_scores);
    do_hiff(bits, &mut scores);
    scores.into()
}

fn do_hiff(bits: &[bool], scores: &mut Vec<i64>) -> bool {
    let len = bits.len();
    // This should be safe because of the `assert!` above in `hiff()`.
    #[allow(clippy::unwrap_used)]
    // FIXME: Use some better score or result here
    let len_as_score = i64::try_from(len).unwrap();
    if len < 2 {
        scores.push(len_as_score);
        true
    } else {
        let half_len = len / 2;
        let left_all_same = do_hiff(&bits[..half_len], scores);
        let right_all_same = do_hiff(&bits[half_len..], scores);
        if left_all_same && right_all_same && bits[0] == bits[half_len] {
            scores.push(len_as_score);
            true
        } else {
            scores.push(0);
            false
        }
    }
}

#[cfg(test)]
mod test_count_ones {
    use ec_core::test_results::{self, TestResults};

    use super::count_ones;

    #[test]
    fn non_empty() {
        let input = [false, true, true, true, false, true];
        let output: TestResults<test_results::Score<i64>> =
            [0, 1, 1, 1, 0, 1].into_iter().collect();
        assert_eq!(output, count_ones(&input));
    }
}
