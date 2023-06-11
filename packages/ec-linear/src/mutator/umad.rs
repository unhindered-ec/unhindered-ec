use ec_core::{generator::Generator, genome::Genome, operator::mutator::Mutator};
use rand::{rngs::ThreadRng, Rng};

use crate::genome::LinearGenome;

/// UMAD = Uniform Mutation through random Addition and Deletion
pub struct Umad<GeneContext> {
    addition_rate: f64,
    deletion_rate: f64,
    // Provides the context needed to generate a new, random gene
    // during the addition phase.
    gene_context: GeneContext,
}

impl<GeneContext> Umad<GeneContext> {
    pub const fn new(addition_rate: f64, deletion_rate: f64, gene_context: GeneContext) -> Self {
        Self {
            addition_rate,
            deletion_rate,
            gene_context,
        }
    }

    fn new_gene<G>(&self, rng: &mut ThreadRng) -> G::Gene
    where
        G: Genome,
        ThreadRng: Generator<G::Gene, GeneContext>,
    {
        rng.generate(&self.gene_context)
    }
}

impl<G, GeneContext> Mutator<G> for Umad<GeneContext>
where
    G: LinearGenome + IntoIterator<Item = G::Gene> + FromIterator<G::Gene>,
    ThreadRng: Generator<G::Gene, GeneContext>,
{
    fn mutate(&self, genome: G, rng: &mut ThreadRng) -> anyhow::Result<G> {
        // Addition pass
        Ok(genome
            .into_iter()
            .flat_map(|gene| {
                // The body of this closure is due to MizardX@Twitch; much nicer than my original approach.
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

                // match (add_gene, delete_gene) {
                //     (true, _) => {
                //         // Add a new gene, and decide whether to then immediately delete it.
                //         let delete_new_gene = rng.gen_bool(self.deletion_rate);
                //         match (delete_gene, delete_new_gene) {
                //             // Delete both the given gene and the new gene.
                //             (true, true) => [None, None],
                //             // Delete the given gene but keep the new gene.
                //             (true, false) => [None, Some(self.new_gene::<G>(rng))],
                //             // Keep the given gene but delete the new gene.
                //             (false, true) => [Some(gene), None],
                //             // Keep both the given gene and the new gene.
                //             (false, false) => [Some(gene), Some(self.new_gene::<G>(rng))],
                //         }
                //     }
                //     // Do not add a gene, and delete the given gene.
                //     (false, true) => [None, None],
                //     // Do not add a gene, and leave the given gene alone.
                //     (false, false) => [Some(gene), None],
                // }
            })
            .flatten()
            .collect::<G>())
    }
}

// TODO: Write a simple test for `Umad` on, say, `Vec<i32>`.
