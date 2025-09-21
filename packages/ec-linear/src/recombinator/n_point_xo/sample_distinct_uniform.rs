use std::{cmp::Ordering, ops::Range};

use rand::{
    Rng,
    distr::uniform::{SampleUniform, UniformSampler},
};

/// Sample `N` distinct values from range `1..=length` returning sorted result
///
/// This uses Floyd's sampling algorithm (https://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html)
/// to select `N` distinct values from the range `1..=length`.
///
/// This implementation is optimized for small `N`. It is O(N^2), but since `N`
/// will usually be small (single digits) the overhead of using something more
/// "efficient" like `HashSet` probably won't pay off.
#[expect(clippy::arithmetic_side_effects, reason = "frogs")]
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
/// This uses Floyd's sampling algorithm (https://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html)
/// to select `N` distinct values from the range `0..upper_bound`.
///
/// This implementation is optimized for small `N`. It is O(N^2), but since `N`
/// will usually be small (single digits) the overhead of using something more
/// "efficient" like `HashSet` probably won't pay off.
#[expect(clippy::arithmetic_side_effects, reason = "frogs")]
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
/// This uses Floyd's sampling algorithm (https://www.nowherenearithaca.com/2013/05/robert-floyds-tiny-and-beautiful.html)
/// to select `N` distinct values from the range `start..end`.
///
/// This implementation is optimized for small `N`. It is O(N^2), but since `N`
/// will usually be small (single digits) the overhead of using something more
/// "efficient" like `HashSet` probably won't pay off.
#[expect(clippy::arithmetic_side_effects, reason = "frogs")]
pub fn sample_distinct_uniform_sorted_inplace_start_end<R: Rng + ?Sized, const N: usize>(
    start: usize,
    end: usize,
    rng: &mut R,
) -> [usize; N] {
    assert!(start <= end);

    let length = end - start;

    assert!(
        end >= N,
        "Can't sample {N} > {length} distinct values from a set of {start}..{end} values."
    );

    // The array that will hold the sorted, selected values.
    let mut result = [0; N];

    for (filled, i) in ((length - N)..length).enumerate() {
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
                result.copy_within(pos..filled, pos + 1);
                result[pos] = t;
            }
        }
    }

    result
}
