use std::ops::Not;

use rand::{rngs::ThreadRng, Rng};

use super::Recombinator;

pub struct MutateWithRate {
    pub mutation_rate: f32,
}

impl<T> Recombinator<1, Vec<T>> for MutateWithRate
where
    T: Clone + Not<Output = T>,
{
    fn recombine(&self, genome: [&Vec<T>; 1], rng: &mut ThreadRng) -> Vec<T> {
        genome[0]
            .iter()
            .map(|bit| {
                let r: f32 = rng.gen();
                if r < self.mutation_rate {
                    !bit.clone()
                } else {
                    bit.clone()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::{
        bitstring::make_random,
        recombinator::{mutate_with_rate::MutateWithRate, Recombinator},
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
        let child_bits = mutator.recombine([&parent_bits], &mut rng);

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
