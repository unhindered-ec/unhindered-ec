use super::Recombinator;

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
