use ec_core::test_results::{self, TestResults};

#[must_use]
pub fn count_ones(bits: &[bool]) -> TestResults<test_results::Score> {
    bits.iter().map(|bit| i64::from(*bit)).collect()
}

#[must_use]
pub fn hiff(bits: &[bool]) -> TestResults<test_results::Score> {
    let num_scores = 2 * bits.len() - 1;
    let mut scores = Vec::with_capacity(num_scores);
    do_hiff(bits, &mut scores);
    scores.into()
}

fn do_hiff(bits: &[bool], scores: &mut Vec<i64>) -> bool {
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

#[cfg(test)]
mod test_count_ones {
    use ec_core::test_results::{self, TestResults};

    use super::count_ones;

    #[test]
    fn empty() {
        let empty_vec: TestResults<test_results::Score> = Vec::new().iter().collect();
        assert_eq!(empty_vec, count_ones(&[]));
    }

    #[test]
    fn non_empty() {
        let input = [false, true, true, true, false, true];
        let output: TestResults<test_results::Score> = [0, 1, 1, 1, 0, 1].iter().collect();
        assert_eq!(output, count_ones(&input));
    }
}
