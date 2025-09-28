use std::ops::Not;

use rand::Rng;

/// Sample `N` distinct values from range `1..=length` returning sorted result
///
/// This uses Floyd's sampling algorithm (<https://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html>)
/// to select `N` distinct values from the range `1..=length`.
///
/// This implementation is optimized for small `N`. It is O(N^2), but since `N`
/// will usually be small (single digits) the overhead of using something more
/// "efficient" like `HashSet` probably won't pay off.
pub fn sample_distinct_uniform_sorted_inplace<R: Rng + ?Sized, const N: usize>(
    length: usize,
    rng: &mut R,
) -> [usize; N] {
    assert!(
        length >= N,
        "Can't sample {N} > {length} distinct values from a set of {length} values."
    );

    // The array that will hold the sorted, selected values.
    let mut result = [0; N];

    #[expect(
        clippy::arithmetic_side_effects,
        reason = "
            length - N:
                we know due to the assertion above that length >= N and as such length - N >= 0 \
                  and won't underflow.

            i + 1:
                we know that i is in the range (length - N)..length and as such i < length (the \
                  upper bound is exclusive), and as such i + 1 <= length and since they are of \
                  the same data type so we know i + 1 won't overflow.

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
        let t = rng.random_range(1..=(i + 1));

        // See if the selected value `t` is already in `result`, i.e., we've already
        // selected that value.
        match result[..filled].binary_search(&t) {
            // We've selected this before, so we actually insert `i+1`, i.e., the current
            // index. We place this at the end of the `result`, which ensures that `result`
            // remains sorted since all the previous values must be < i+1.
            Ok(_) => {
                result[filled] = i + 1;
            }
            // We haven't selected this before, so we have to insert it into the correct location,
            // shifting all the larger values one position to the right.
            Err(pos) => {
                result.copy_within(pos..filled, pos + 1);
                result[pos] = t;
            }
        }
    }

    result
}

/// Sample `N` distinct values from range `0..upper_bound` returning
/// sorted result
///
/// This uses Floyd's sampling algorithm (<https://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html>)
/// to select `N` distinct values from the range `0..upper_bound`.
///
/// This implementation is optimized for small `N`. It is O(N^2), but since `N`
/// will usually be small (single digits) the overhead of using something more
/// "efficient" like `HashSet` probably won't pay off.
pub fn sample_distinct_uniform_sorted_inplace_start_at_0<R: Rng + ?Sized, const N: usize>(
    upper_bound: usize,
    rng: &mut R,
) -> [usize; N] {
    assert!(
        upper_bound >= N,
        "Can't sample {N} > {upper_bound} distinct values from a set of 0..{upper_bound} values."
    );

    // The array that will hold the sorted, selected values.
    let mut result = [0; N];

    #[expect(
        clippy::arithmetic_side_effects,
        reason = "
            upper_bound - N:
                we know due to the assertion above that upper_bound >= N and as such upper_bound - \
                  N >= 0 and won't underflow.

            pos + 1:
                since we iterate over (upper_bound - N)..upper_bound which is of size N we know \
                  that the loop will have exactly N iterations (to fill the entire array)

                As such, in the last iteration (where the array is biggest and as such pos which \
                  is a index of the array can be largest) we are still searching in a sub-slice \
                  of the array, namely..filled of length less than N. (since filled <= N (loop \
                  counter & loop count reasoning from above) and upper bound is exclusive). So we \
                  know that pos < filled and as such pos + 1 <= filled which is of the same data \
                  type as filled and as such is also representable without wrapping (overflow) \
                  (see reasoning for i+1).
        "
    )]
    for (filled, i) in ((upper_bound - N)..upper_bound).enumerate() {
        let t = rng.random_range(0..=i);

        // See if the selected value `t` is already in `result`, i.e., we've already
        // selected that value.
        match result[..filled].binary_search(&t) {
            // We've selected this before, so we actually insert `i+1`, i.e., the current
            // index. We place this at the end of the `result`, which ensures that `result`
            // remains sorted since all the previous values must be < i+1.
            Ok(_) => {
                result[filled] = i;
            }
            // We haven't selected this before, so we have to insert it into the correct location,
            // shifting all the larger values one position to the right.
            Err(pos) => {
                result.copy_within(pos..filled, pos + 1);
                result[pos] = t;
            }
        }
    }

    result
}

// TODO: Keep this one and get rid of the other two.

/// Sample `N` distinct values from range `start..end` returning
/// sorted result
///
/// This uses Floyd's sampling algorithm (<https://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html>)
/// to select `N` distinct values from the range `start..end`.
///
/// This implementation is optimized for small `N`. It is O(N^2), but since `N`
/// will usually be small (single digits) the overhead of using something more
/// "efficient" like `HashSet` probably won't pay off.
pub fn sample_distinct_uniform_sorted_inplace_start_end<R: Rng + ?Sized, const N: usize>(
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
            // We've selected this before, so we actually insert `i+1`, i.e., the current
            // index. We place this at the end of the `result`, which ensures that `result`
            // remains sorted since all the previous values must be < i+1.
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
