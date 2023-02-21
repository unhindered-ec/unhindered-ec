use std::ops::Not;

use rand::{rngs::ThreadRng, Rng};

use crate::operator::{Composable, Operator};

pub struct MutateWithRate {
    mutation_rate: f32,
}

impl MutateWithRate {
    #[must_use]
    pub const fn new(mutation_rate: f32) -> Self {
        Self { mutation_rate }
    }
}

impl<T> Operator<Vec<T>> for MutateWithRate
where
    T: Not<Output = T>,
{
    type Output = Vec<T>;

    fn apply(&self, genome: Vec<T>, rng: &mut ThreadRng) -> Self::Output {
        genome
            .into_iter()
            .map(|bit| {
                let r: f32 = rng.gen();
                if r < self.mutation_rate {
                    !bit
                } else {
                    bit
                }
            })
            .collect()
    }
}
impl Composable for MutateWithRate {}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::{
        bitstring::make_random, operator::recombinator::mutate_with_rate::MutateWithRate,
        operator::Operator,
    };

    // This test is stochastic, so I'm going to ignore it most of the time.
    #[test]
    #[ignore]
    fn mutate_with_rate_does_not_change_much() {
        let mutator = MutateWithRate {
            mutation_rate: 0.05,
        };

        let mut rng = rand::thread_rng();
        let num_bits = 100;
        let parent_bits = make_random(num_bits, &mut rng);
        let child_bits = mutator.apply(parent_bits.clone(), &mut rng);

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
