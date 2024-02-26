use ec_core::{genome::Genome, operator::mutator::Mutator};
use rand::{prelude::Distribution, rngs::ThreadRng, Rng};

use crate::genome::Linear;

/// UMAD = Uniform Mutation through random Addition and Deletion
pub struct Umad<GeneGenerator> {
    addition_rate: f64,
    deletion_rate: f64,
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
            gene_generator,
        }
    }

    fn new_gene<G>(&self, rng: &mut ThreadRng) -> G::Gene
    where
        G: Genome,
        GeneGenerator: Distribution<G::Gene>,
    {
        self.gene_generator.sample(rng)
    }
}

impl<G, GeneGenerator> Mutator<G> for Umad<GeneGenerator>
where
    G: Linear + IntoIterator<Item = G::Gene> + FromIterator<G::Gene>,
    GeneGenerator: Distribution<G::Gene>,
{
    fn mutate(&self, genome: G, rng: &mut ThreadRng) -> anyhow::Result<G> {
        // Addition pass
        Ok(genome
            .into_iter()
            .flat_map(|gene| {
                // The body of this closure is due to MizardX@Twitch;
                // much nicer than my original approach.
                let add_gene = rng.gen_bool(self.addition_rate);
                let delete_gene = rng.gen_bool(self.deletion_rate);
                // only called when `add_gene` is true
                let delete_new_gene = add_gene && rng.gen_bool(self.deletion_rate);

                #[allow(clippy::match_bool)]
                let old_gene = match delete_gene {
                    false => Some(gene),
                    true => None,
                };

                let new_gene = match (add_gene, delete_new_gene) {
                    (true, false) => Some(self.new_gene::<G>(rng)),
                    _ => None,
                };

                [old_gene, new_gene]
            })
            .flatten()
            .collect::<G>())
    }
}

#[cfg(test)]
mod test {
    use ec_core::generator::slice_cloning::SliceCloning;
    use rand::thread_rng;

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
    #[allow(clippy::unwrap_used)]
    #[ignore = "This is stochastic, and it will fail sometimes"]
    fn umad_test() {
        let mut rng = thread_rng();

        let char_options = SliceCloning::new(&['x']).unwrap();
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
