use super::Recombinator;

/*
 * What you really need are separate Selector, Mutator, Recombinator and Operator traits. 
 * Then Select, Mutate and Recombinate structs that take a type of their respective trait 
 * and implements Operator. The Operator trait would have a single method that takes an input, 
 * population and rng. Then a bunch of composition structs and methods implemented by default on Operator.
 */

// @esitsu's idea from Discord:
// select(Random)                           // type Select<Random>
//   .then_mutate(Random)                   // Then<^, Mutate<Random>>
//   .and_select(Best.then_mutate(Random))  // And<^Then<Best, Mutate<Random>>>
//   .then_recombinate(TwoPointCrossover)   // Then<^, Recombinate<TwoPointCrossover>>
//   .then_select(BestChild)
//   .then_mutate(Random)

// Parent A: 0011|01001
// Parent B: 1011|00101
// Children: 0011|00101, 1011|01001

// enum Recombinator {
//     Unary,  // One input genome, one output genome
//     Binary, // Two input genomes, one output genome
//     Trinary,
//     Nary,   // Arbitrary number of input genomes, one output genome
//             // Only use if you genuinely didn't care how many inputs you got.
//     GenomeSelector, // Takes no arguments and returns a single genome
//                     // Might be undesirable because it would require that
//                     // we pass in the Population.
// }

// let first = genomeSelector(Random).then_unary(OneOverLengthMutator)
// let second = genomeSelector(Best).then_unary(Mutator(0.01))
// let child = TwoPointXo(first, second).then_unary(OneOverLengthMutator)

// let 

// A simple example:
// Two parents as inputs:
//   * cross them over/recombine them
//   * mutate the resulting genome

// A more interesting:
//   * Take one parent
//      * Mutate that parent, G0
//   * Take another parent
//      * Mutate that parent, G1
//   * Recombine G0 and G1, G2
//   * Mutate G2

// Can we have the type checker help us with check
// the number of genomes each recombinator takes?
pub struct Pipeline<G> {
    recombinators: Vec<Box<dyn Recombinator<G>>>
}

impl<G> Pipeline<G> {
    pub fn new(recombinators: Vec<Box<dyn Recombinator<G>>>) -> Self {
        Pipeline { recombinators }
    }
}

impl<G> Recombinator<G> for Pipeline<G> {
    fn recombine(&self, genomes: &[&G], rng: &mut rand::rngs::ThreadRng) -> G {
        let mut recombinators = self.recombinators.iter();
        let first_recombinator = recombinators.next().unwrap();
        let first_genome = first_recombinator.recombine(genomes, rng);
        recombinators
            .fold(first_genome, |prev_genome, recombinator| {
                recombinator.recombine(&[&prev_genome], rng)
            })
    }
}
