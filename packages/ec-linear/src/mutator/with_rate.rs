use anyhow::Result;
use ec_core::operator::mutator::Mutator;
use std::ops::Not;

use rand::{rngs::ThreadRng, Rng};

pub struct WithRate {
    mutation_rate: f32,
}

impl<T> Mutator<Vec<T>> for WithRate
where
    T: Not<Output = T>,
{
    fn mutate(&self, genome: Vec<T>, rng: &mut ThreadRng) -> Result<Vec<T>> {
        Ok(genome
            .into_iter()
            .map(|bit| {
                let r: f32 = rng.gen();
                if r < self.mutation_rate {
                    !bit
                } else {
                    bit
                }
            })
            .collect())
    }
}

impl WithRate {
    #[must_use]
    pub const fn new(mutation_rate: f32) -> Self {
        Self { mutation_rate }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use ec_core::operator::mutator::Mutator;

    use crate::{genome::bitstring::make_random, mutator::with_rate::WithRate};

    // This test is stochastic, so I'm going to ignore it most of the time.
    #[test]
    #[ignore]
    #[allow(clippy::unwrap_used)]
    fn mutate_with_rate_does_not_change_much() {
        let mutator = WithRate {
            mutation_rate: 0.05,
        };

        let mut rng = rand::thread_rng();
        let num_bits = 100;
        let parent_bits = make_random(num_bits, &mut rng);
        let child_bits = mutator.mutate(parent_bits.clone(), &mut rng).unwrap();

        let num_differences = zip(parent_bits, child_bits)
            .filter(|(p, c)| *p != *c)
            .count();
        println!("Num differences = {num_differences}");
        assert!(
            0 < num_differences,
            "We're expecting at least one difference"
        );
        assert!(
            num_differences < num_bits / 10,
            "We're not expecting lots of differences, and got {num_differences}."
        );
    }
}
