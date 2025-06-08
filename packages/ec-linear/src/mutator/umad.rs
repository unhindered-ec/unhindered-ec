use std::convert::Infallible;

use ec_core::{genome::Genome, operator::mutator::Mutator};
use rand::{Rng, prelude::Distribution};

use crate::genome::Linear;

/// UMAD = Uniform Mutation through random Addition and Deletion
pub struct Umad<GeneGenerator> {
    addition_rate: f64,
    deletion_rate: f64,
    empty_addition_rate: Option<f64>,
    // Provides the generator needed to generate a new, random gene
    // during the addition phase.
    gene_generator: GeneGenerator,
}

impl<GeneGenerator> Umad<GeneGenerator> {
    pub const fn new(
        addition_rate: f64,
        deletion_rate: f64,
        gene_generator: GeneGenerator,
    ) -> Self {
        Self {
            addition_rate,
            deletion_rate,
            empty_addition_rate: Some(addition_rate),
            gene_generator,
        }
    }

    /// Construct `Umad` with "balanced" deletion rate derived from addition
    /// rate
    ///
    /// If the deletion rate is set to `addition_rate` / (1 + `addition_rate`)
    /// then on average the child genome will have the same length as the
    /// parent genome. This constructor just takes an `addition_rate`
    /// and computes the balanced deletion rate.
    pub const fn new_with_balanced_deletion(
        addition_rate: f64,
        gene_generator: GeneGenerator,
    ) -> Self {
        // Using this deletion means that _on average_ the child genome
        // will have the same length as the parent genome.
        let deletion_rate = addition_rate / (1.0 + addition_rate);
        Self::new(addition_rate, deletion_rate, gene_generator)
    }

    pub const fn new_with_empty_rate(
        addition_rate: f64,
        empty_addition_rate: f64,
        deletion_rate: f64,
        gene_generator: GeneGenerator,
    ) -> Self {
        Self {
            addition_rate,
            deletion_rate,
            empty_addition_rate: Some(empty_addition_rate),
            gene_generator,
        }
    }

    pub const fn new_without_empty(
        addition_rate: f64,
        deletion_rate: f64,
        gene_generator: GeneGenerator,
    ) -> Self {
        Self {
            addition_rate,
            deletion_rate,
            empty_addition_rate: None,
            gene_generator,
        }
    }

    fn new_gene<G, R>(&self, rng: &mut R) -> G::Gene
    where
        G: Genome,
        GeneGenerator: Distribution<G::Gene>,
        R: Rng + ?Sized,
    {
        self.gene_generator.sample(rng)
    }
}

impl<G, GeneGenerator> Mutator<G> for Umad<GeneGenerator>
where
    G: Linear + IntoIterator<Item = G::Gene> + FromIterator<G::Gene>,
    GeneGenerator: Distribution<G::Gene>,
{
    type Error = Infallible;

    fn mutate<R: Rng + ?Sized>(&self, genome: G, rng: &mut R) -> Result<G, Self::Error> {
        if genome.size() == 0 {
            if let Some(addition_rate) = self.empty_addition_rate {
                return Ok(rng
                    .random_bool(addition_rate)
                    .then(|| self.new_gene::<G, R>(rng))
                    .into_iter()
                    .collect());
            }
        }
        // Addition pass
        Ok(genome
            .into_iter()
            .flat_map(|gene| {
                // The body of this closure is due to MizardX@Twitch;
                // much nicer than my original approach.
                let add_gene = rng.random_bool(self.addition_rate);
                let delete_gene = rng.random_bool(self.deletion_rate);
                // only called when `add_gene` is true
                let delete_new_gene = add_gene && rng.random_bool(self.deletion_rate);

                let old_gene = (!delete_gene).then_some(gene);

                let new_gene = match (add_gene, delete_new_gene) {
                    (true, false) => Some(self.new_gene::<G, R>(rng)),
                    _ => None,
                };

                [old_gene, new_gene]
            })
            .flatten()
            .collect::<G>())
    }
}

#[cfg(test)]
#[expect(
    clippy::arithmetic_side_effects,
    reason = "The tradeoff safety <> ease of writing arguably lies on the ease of writing side \
              for test code."
)]
mod test {
    use ec_core::uniform_distribution_of;
    use rand::rng;

    use super::*;
    use crate::genome::vector::Vector;

    fn count_missing(short: &[char], long: &[char]) -> Option<usize> {
        let mut short_index = 0;
        let mut long_index = 0;
        let mut num_missing = 0;
        while short_index < short.len() && long_index < long.len() {
            if short[short_index] == long[long_index] {
                short_index += 1;
            } else {
                num_missing += 1;
            }
            long_index += 1;
        }
        (short_index == short.len()).then_some(num_missing + (long.len() - long_index))
    }

    #[test]
    #[ignore = "This is stochastic, and it will fail sometimes"]
    fn umad_test() {
        let mut rng = rng();

        let char_options = uniform_distribution_of!['x'];
        let umad = Umad::new(0.3, 0.3, char_options);

        let parent_chars = "Morris, Minnesota".chars().collect::<Vec<_>>();
        let parent = Vector {
            genes: parent_chars.clone(),
        };

        let child = umad.mutate(parent, &mut rng).unwrap();

        // println!("{child:?}");
        let num_xs = child.genes.iter().filter(|c| **c == 'x').count();
        assert!(
            num_xs > 0,
            "Expected at least one 'x' to be added, but none were."
        );
        let remaining_chars = child
            .genes
            .iter()
            .filter(|c| **c != 'x')
            .copied()
            .collect::<Vec<_>>();
        // println!("There were {num_xs} 'x's.");
        // println!("The remaining characters were {remaining_chars:?}");
        let num_missing = count_missing(&remaining_chars, &parent_chars);
        assert!(
            num_missing.is_some(),
            "The remaining chars {remaining_chars:?} should have beenan ordered subsequence of \
             the parent chars {parent_chars:?}."
        );
        assert!(
            num_missing.unwrap() > 0,
            "There should have been at least one character dropped from the parent in {child:?}"
        );
    }
}
