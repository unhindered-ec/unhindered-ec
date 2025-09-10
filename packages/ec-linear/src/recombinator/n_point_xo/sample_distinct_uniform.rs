use std::{cmp::Ordering, ops::Range};

use rand::distr::uniform::{SampleUniform, UniformSampler};

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
    pub fn sample_array<R: rand::Rng + ?Sized, const N: usize>(
        &self,
        rng: &mut R,
    ) -> Option<[T; N]> {
        let mut samples: [_; N] = [T::default(); N];

        for k in const { 0..N } {
            let j: T = (self.range_from.end - self.range_from.start)
                .checked_sub(&<T as TryFrom<usize>>::try_from(k).ok()?)?;

            if j < T::zero() {
                return None;
            }

            let random = T::Sampler::sample_single(T::zero(), j, rng).ok()?;
            let value = random + self.range_from.start;

            let mut pos_to_insert = Some(k);
            for (pos, sample) in samples[..k].iter().enumerate() {
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
                        samples[..k].iter().position(|&v| v > value).unwrap_or(k),
                        value,
                    )
                },
                |v| (v, value),
            );

            samples.copy_within(pos_to_insert..k, pos_to_insert.saturating_add(1));
            samples[pos_to_insert] = value_to_insert;
        }

        Some(samples)
    }
}
