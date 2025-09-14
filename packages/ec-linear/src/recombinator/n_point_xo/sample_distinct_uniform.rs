use std::{cmp::Ordering, ops::Range};

use rand::{
    Rng,
    distr::uniform::{SampleUniform, UniformSampler},
};

#[expect(clippy::arithmetic_side_effects, reason = "frogs")]
pub fn sample_distinct_uniform_sorted_inplace<R: Rng + ?Sized, const N: usize>(
    length: usize,
    rng: &mut R,
) -> [usize; N] {
    assert!(
        length >= N,
        "Can't sample {N} > {length} distinct values from a set of {length} values."
    );

    let mut result = [0; N];

    for (filled, i) in ((length - N)..length).enumerate() {
        let t = rng.random_range(1..=(i + 1));

        match result[..filled].binary_search(&t) {
            Ok(_) => {
                result[filled] = i + 1;
            }
            Err(pos) => {
                result.copy_within(pos..filled, pos + 1);
                result[pos] = t;
            }
        }
    }

    result
}

pub struct SampleDistinctUniform<T> {
    range_from: Range<T>,
}

impl<T> SampleDistinctUniform<T> {
    pub const fn new(from: T, to: T) -> Self {
        Self {
            range_from: from..to,
        }
    }
}
impl<T> SampleDistinctUniform<T>
where
    T: SampleUniform
        + Default
        + std::ops::Sub<Output = T>
        + std::ops::Add<Output = T>
        + TryFrom<usize>
        + num_traits::One
        + num_traits::Zero
        + Copy
        + Eq
        + Ord
        + num_traits::CheckedSub,
{
    pub fn sample_array<R: rand::Rng + ?Sized, const NUM_SAMPLES: usize>(
        &self,
        rng: &mut R,
    ) -> Option<[T; NUM_SAMPLES]> {
        let mut samples: [_; NUM_SAMPLES] = [T::default(); NUM_SAMPLES];

        for sample in const { 0..NUM_SAMPLES } {
            // j = end - start - sample
            let j: T = (self.range_from.end - self.range_from.start)
                .checked_sub(&<T as TryFrom<usize>>::try_from(sample).ok()?)?;

            if j < T::zero() {
                return None;
            }

            let random = T::Sampler::sample_single(T::zero(), j, rng).ok()?;
            let value = random + self.range_from.start;

            let mut pos_to_insert = Some(sample);
            for (pos, sample) in samples[..sample].iter().enumerate() {
                let ordering = sample.cmp(&value);
                match ordering {
                    Ordering::Less => { /* continue searching */ }
                    Ordering::Equal => {
                        pos_to_insert = None;
                        break;
                    }
                    Ordering::Greater => {
                        pos_to_insert = Some(pos);
                        break;
                    }
                }
            }

            let (pos_to_insert, value_to_insert) = pos_to_insert.map_or_else(
                || {
                    let value = j + T::one() + self.range_from.start;

                    (
                        samples[..sample]
                            .iter()
                            .position(|&v| v > value)
                            .unwrap_or(sample),
                        value,
                    )
                },
                |v| (v, value),
            );

            samples.copy_within(pos_to_insert..sample, pos_to_insert.saturating_add(1));
            samples[pos_to_insert] = value_to_insert;
        }

        Some(samples)
    }
}
