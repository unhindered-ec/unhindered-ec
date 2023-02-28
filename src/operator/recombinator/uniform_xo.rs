use rand::{rngs::ThreadRng, Rng};

use super::Recombinator;

pub struct UniformXo;

impl<T: Clone> Recombinator<[Vec<T>; 2]> for UniformXo {
    type Output = Vec<T>;

    fn recombine(
        &self,
        [first_genome, second_genome]: [Vec<T>; 2],
        rng: &mut ThreadRng,
    ) -> Self::Output {
        assert_eq!(first_genome.len(), second_genome.len());
        let len = first_genome.len();
        (0..len)
            .map(|pos| {
                if rng.gen_bool(0.5) {
                    first_genome[pos].clone()
                } else {
                    second_genome[pos].clone()
                }
            })
            .collect()
    }
}
