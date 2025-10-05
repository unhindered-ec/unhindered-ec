use std::ops::Not;

use rand::Rng;

/// Sample `N` distinct values from range `start..end` returning
/// sorted result
///
/// This uses Floyd's sampling algorithm (<https://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html>)
/// to select `N` distinct values from the range `start..end`.
///
/// This implementation is optimized for small `N`. It is O(N^2), but since `N`
/// will usually be small (single digits) the overhead of using something more
/// "efficient" like `HashSet` probably won't pay off.
pub fn sample_distinct_uniform_sorted<R: Rng + ?Sized, const N: usize>(
    start: usize,
    end: usize,
    rng: &mut R,
) -> [usize; N] {
    assert!(start <= end);

    #[expect(
        clippy::arithmetic_side_effects,
        reason = "We assert above that start <= end, and as such end - start >= 0 and won't \
                  underflow."
    )]
    let length = end - start;

    assert!(
        length >= N, // i.e., length - N >= 0
        "Can't sample {N} > {length} distinct values from a set of {start}..{end} values."
    );

    // The array that will hold the sorted, selected values.
    let mut result = [0; N];

    #[expect(
        clippy::arithmetic_side_effects,
        reason = "
            length - N:
                we know due to the assertion above that length >= N and as such length - N >= 0 \
                  and won't underflow.

            i + start:
                Since `i` is in the range `((length - N)..length)`, we know `i < length`.
                  Then we can bound `i+start` by:
                    i + start < length + start
                              = (end - start) + start
                              = end
                  so `i + start < end`. Thus `i + start` can't overflow because `end` is a legal \
                  value of type `usize`.

            pos + 1:
                Since we iterate over (length - N)..length which is of size N we know that the loop
                will have exactly N iterations (to fill the entire array).

                Throughout the `for` loop we know:
                  - `filled < N`
                  - `pos<=filled` because it is in the range `0..=filled`,

                Then the `copy_within` call is passed `pos..filled` as the source range and
                  `pos + 1` as the start of the destination range; thus the destination range
                  is `(pos + 1)..(filled + 1)`. All four range ends are at most `N` because
                  (as shown above) `pos <= filled < N`.

                We also know that `pos <= filled < N`, so `pos + 1 <= N`, so it can't overflow
                since both `N` and `pos` are of type `usize`.
        "
    )]
    for (filled, i) in ((length - N)..length).enumerate() {
        debug_assert!(i + start < end);
        debug_assert!((start..=(i + start)).is_empty().not());
        // Since `i: usize` and thus `i >= 0`, `start..=(i+start)` always has at least
        // one element, namely `start`.
        let t = rng.random_range(start..=(i + start));

        // See if the selected value `t` is already in `result`, i.e., we've already
        // selected that value.
        match result[..filled].binary_search(&t) {
            // We've selected this before, so we actually insert `i+start`, i.e., the current
            // index. We place this at the end of the `result`, which ensures that `result`
            // remains sorted since all the previous values must be < i+start.
            Ok(_) => {
                result[filled] = i + start;
            }
            // We haven't selected this before, so we have to insert it into the correct location,
            // shifting all the larger values one position to the right.
            Err(pos) => {
                debug_assert!(pos <= filled);
                debug_assert!(pos < N); // i.e., pos + 1 <= N
                result.copy_within(pos..filled, pos + 1);
                result[pos] = t;
            }
        }
    }

    result
}
