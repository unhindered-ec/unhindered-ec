use std::ops::Not;

use num_traits::ToPrimitive;
use rand::rngs::ThreadRng;

use super::{mutate_with_rate::MutateWithRate, Mutator};

pub struct MutateWithOneOverLength;

impl<T> Mutator<Vec<T>> for MutateWithOneOverLength
where
    T: Not<Output = T>,
{
    fn mutate(&self, genome: Vec<T>, rng: &mut ThreadRng) -> Vec<T> {
        let mutation_rate = genome.len().to_f32().map_or(f32::MIN_POSITIVE, |l| 1.0 / l);
        let mutator = MutateWithRate::new(mutation_rate);
        mutator.mutate(genome, rng)
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use crate::{
        bitstring::make_random,
        operator::recombinator::mutate_with_one_over_length::MutateWithOneOverLength,
        operator::{Operator, recombinator::Mutator},
    };

    // This test is stochastic, so I'm going to ignore it most of the time.
    #[test]
    #[ignore]
    fn mutate_one_over_does_not_change_much() {
        let mut rng = rand::thread_rng();
        let num_bits = 100;
        let parent_bits = make_random(num_bits, &mut rng);

        let child_bits = MutateWithOneOverLength.mutate(parent_bits.clone(), &mut rng);

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
