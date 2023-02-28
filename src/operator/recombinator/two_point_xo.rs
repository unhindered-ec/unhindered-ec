use rand::{rngs::ThreadRng, Rng};

use super::Recombinator;

pub struct TwoPointXo;

// TODO: Note that `TwoPointXo` doesn't strictly need
//   the two vectors to have the same length, but the
//   swapped regions need to "make sense" for both parent
//   genomes.
impl<T> Recombinator<[Vec<T>; 2]> for TwoPointXo {
    type Output = Vec<T>;

    fn recombine(
        &self,
        [mut first_genome, mut second_genome]: [Vec<T>; 2],
        rng: &mut ThreadRng,
    ) -> Self::Output {
        assert_eq!(first_genome.len(), second_genome.len());
        let len = first_genome.len();

        let mut first = rng.gen_range(0..len);
        let mut second = rng.gen_range(0..len);
        if second < first {
            (first, second) = (second, first);
        }
        // We now know that first <= second
        first_genome[first..second].swap_with_slice(&mut second_genome[first..second]);
        first_genome
    }
}
