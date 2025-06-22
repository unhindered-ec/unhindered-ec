use std::convert::Infallible;

use ec_core::{genome::Genome, operator::mutator::Mutator};
use rand::{Rng, prelude::Distribution};

use crate::genome::Linear;

/// The UMAD mutation operator as described in
/// [the paper by T. Helmuth et al.](https://doi.org/10.1145/3205455.3205603)[^paper-ref]
///
/// The UMAD (Uniform Mutation by Addition and Deletion) operator acts
/// on variable length linear genomes, inserting new genes and deleting existing
/// genes in a random fashion.
///
/// New genes are generated using a user-provided `GeneGenerator`.
///
/// # Behavior
///
/// For each gene in the initial genome, a decision is made about
///
/// - Whether to keep or delete that gene, and
/// - Whether to insert a new gene, either to the existing gene's left or right.
///
/// Note that this can change the length of the genome, with the child
/// potentially being shorter or longer than the parent.
///
/// ## Parameters
///
/// There are several parameters which can be used to tune the mutation
/// operator:
///
/// - The addition rate, i.e., the likelihood of adding a new gene next to each
///   of the existing genes
/// - The deletion rate, i.e., the likelihood of deleting each gene in the
///   genome, including newly added genes
/// - The empty addition rate, i.e., the likelihood of adding a new gene when
///   the initial genome is empty. This is optional, and if it is not provided
///   then new genes will never be added to empty genomes. See below for further
///   discussion of this feature.
///
/// ## Balance
///
/// If the addition rate is much higher than the
/// deletion rate then the new genome is likely to be considerably
/// longer than the original genome. Similarly, if the deletion rate
/// is much higher than the addition rate then the genome is likely
/// to shrink.
///
/// If the deletion rate is set to `addition_rate / (1 + addition_rate)`,
/// then the expected length of the new child genome is the same as that of the
/// initial parent genome. The [`Umad::new_with_balanced_deletion`] constructor
/// sets the deletion rate according to this formula.
///
/// ## Differences from published UMAD definition
///
/// The one difference between this implementation and the published definition
/// is that we support the addition of a randomly generated gene if the initial
/// genome is empty. In the published definition, an empty genome would remain
/// empty. The likelihood of adding a gene to an empty genome is set by the
/// `empty_addition_rate` parameter; setting this to `None` will effectively
/// disable this feature, behaving the same as the published definition.
///
/// # Example
///
/// In this example we create an instance of `Umad` using [`Umad::new`] that
/// mutates vectors of characters.
///
/// ```
/// # use ec_core::{operator::mutator::Mutator, uniform_distribution_of};
/// # use ec_linear::{genome::vector::Vector, mutator::umad::Umad};
/// #
/// // Use a 30% chance of inserting a gene at each location.
/// let addition_rate = 0.3;
/// // Use a 30% chance of deleting each gene.
/// let deletion_rate = 0.3;
/// // This is our `GeneGenerator`, which will always just produce
/// // a character `x`.
/// let gene_generator = uniform_distribution_of!['x'];
///
/// // Create an instance of `Umad` using these parameters.
/// let umad = Umad::new(addition_rate, deletion_rate, gene_generator);
///
/// // Create a `Vec` of characters to use as a parent genome.
/// let parent_chars = "Mutate first, ask questions later.".chars().collect();
/// // Create a parent genome
/// let parent_genome = Vector {
///     genes: parent_chars,
/// };
///
/// // Mutate the parent genome to create a child genome
/// let child_genome = umad.mutate(parent_genome, &mut rand::rng()).unwrap();
/// ```
///
/// [^paper-ref]: Thomas Helmuth, Nicholas Freitag McPhee, and Lee Spector. 2018.
///   Program synthesis using uniform mutation by addition and deletion. In
///   Proceedings of the Genetic and Evolutionary Computation Conference
///   (GECCO '18). Association for Computing Machinery, New York, NY, USA,
///   1127–1134. <https://doi.org/10.1145/3205455.3205603>
#[must_use]
pub struct Umad<GeneGenerator> {
    /// The likelihood of adding a new gene next to each gene in the original
    /// genome
    addition_rate: f64,
    /// The likelihood of deleting each gene in the genome, including newly
    /// added genes
    deletion_rate: f64,
    /// The likelihood of adding a single new gene when the initial genome is
    /// empty. This is optional, and if its value is `None` then new genes
    /// will never be added to empty genomes.
    empty_addition_rate: Option<f64>,
    /// The generator used to generate new, random genes
    /// during the addition phase.
    gene_generator: GeneGenerator,
}

impl<GeneGenerator> Umad<GeneGenerator> {
    /// Construct an instance of `Umad` with empty addition rate the same as
    /// addition rate.
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
    /// If the deletion rate is set to `addition_rate / (1 + addition_rate)`
    /// then on average the child genome will have the same length as the
    /// parent genome. This constructor just takes an `addition_rate`
    /// and computes the balanced deletion rate.
    ///
    /// # Panics
    ///
    /// This panics if the `addition_rate` isn't a legal probability value
    /// (i.e., in range 0..=1).
    ///
    /// # Example
    ///
    /// ```
    /// # use ec_core::operator::mutator::Mutator;
    /// # use ec_linear::mutator::umad::Umad;
    /// #
    /// let addition_rate = 0.25;
    /// let expected_deletion_rate = addition_rate / (1.0 + addition_rate);
    /// let umad = Umad::new_with_balanced_deletion(addition_rate, ());
    ///
    /// assert!(
    ///     (umad.deletion_rate() - expected_deletion_rate).abs() < f64::EPSILON,
    ///     "Expected deletion rate {expected_deletion_rate} but got deletion rate {}",
    ///     umad.deletion_rate()
    /// );
    /// ```
    pub fn new_with_balanced_deletion(addition_rate: f64, gene_generator: GeneGenerator) -> Self {
        assert!(
            matches!(addition_rate, 0.0..=1.0),
            "`addition_rate` must be between 0.0 and 1.0 inclusive, but was {addition_rate}"
        );
        let deletion_rate = addition_rate / (1.0 + addition_rate);
        Self::new(addition_rate, deletion_rate, gene_generator)
    }

    /// Construct `Umad` with a given empty addition rate
    ///
    /// # Panics
    ///
    /// This panics if any of the `addition_rate`, `empty_addition_rate`, or
    /// `deletion_rate` isn't a legal probability value (i.e., in range
    /// 0..=1).
    ///
    /// # Example
    ///
    /// ```
    /// # use ec_linear::mutator::umad::Umad;
    /// #
    /// let addition_rate = 0.25;
    /// let empty_addition_rate = 0.1;
    /// let deletion_rate = 0.2;
    /// let umad =
    ///     Umad::new_with_empty_addition_rate(addition_rate, empty_addition_rate, deletion_rate, ());
    ///
    /// assert_eq!(
    ///     umad.empty_addition_rate(),
    ///     Some(empty_addition_rate),
    ///     "Expected the given empty addition rate since we used the new_with_empty_addition_rate() \
    ///      constructor"
    /// );
    /// ```
    pub fn new_with_empty_addition_rate(
        addition_rate: f64,
        empty_addition_rate: f64,
        deletion_rate: f64,
        gene_generator: GeneGenerator,
    ) -> Self {
        assert!(
            matches!(addition_rate, 0.0..=1.0),
            "`addition_rate` must be between 0.0 and 1.0 inclusive, but was {addition_rate}"
        );
        assert!(
            matches!(empty_addition_rate, 0.0..=1.0),
            "`empty_addition_rate` must be between 0.0 and 1.0 inclusive, but was \
             {empty_addition_rate}"
        );
        assert!(
            matches!(deletion_rate, 0.0..=1.0),
            "`deletion_rate` must be between 0.0 and 1.0 inclusive, but was {deletion_rate}"
        );

        Self {
            addition_rate,
            deletion_rate,
            empty_addition_rate: Some(empty_addition_rate),
            gene_generator,
        }
    }

    /// Construct `Umad` without an empty addition rate
    ///
    /// # Panics
    ///
    /// This panics if any of the `addition_rate` or
    /// `deletion_rate` isn't a legal probability value (i.e., in range
    /// 0..=1).
    ///
    /// # Example
    ///
    /// ```
    /// # use ec_linear::mutator::umad::Umad;
    /// #
    /// let addition_rate = 0.25;
    /// let deletion_rate = 0.2;
    /// let umad = Umad::new_without_empty_addition_rate(addition_rate, deletion_rate, ());
    ///
    /// assert!(
    ///     umad.empty_addition_rate().is_none(),
    ///     "Expected an empty addition rate since we used the new_without_empty_addition_rate() \
    ///      constructor"
    /// );
    /// ```
    pub fn new_without_empty_addition_rate(
        addition_rate: f64,
        deletion_rate: f64,
        gene_generator: GeneGenerator,
    ) -> Self {
        assert!(
            matches!(addition_rate, 0.0..=1.0),
            "`addition_rate` must be between 0.0 and 1.0 inclusive, but was {addition_rate}"
        );
        assert!(
            matches!(deletion_rate, 0.0..=1.0),
            "`deletion_rate` must be between 0.0 and 1.0 inclusive, but was {deletion_rate}"
        );

        Self {
            addition_rate,
            deletion_rate,
            empty_addition_rate: None,
            gene_generator,
        }
    }

    #[must_use]
    fn new_gene<G, R>(&self, rng: &mut R) -> G::Gene
    where
        G: Genome,
        GeneGenerator: Distribution<G::Gene>,
        R: Rng + ?Sized,
    {
        self.gene_generator.sample(rng)
    }

    /// The probability (in the range 0..=1) of adding a new gene next to each
    /// gene in the original genome
    #[must_use]
    pub const fn addition_rate(&self) -> f64 {
        self.addition_rate
    }

    /// The probability (in the range 0..=1) of deleting each gene in the
    /// genome, including newly added genes
    #[must_use]
    pub const fn deletion_rate(&self) -> f64 {
        self.deletion_rate
    }

    /// The probability (in the range 0..=1) of adding a single new gene when
    /// the initial genome is empty.
    ///
    /// This is optional, and if its value is
    /// `None` then new genes will never be added to empty genomes.
    #[must_use]
    pub const fn empty_addition_rate(&self) -> Option<f64> {
        self.empty_addition_rate
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

                // This randomly decides with a 50/50 probability which side of the old gene
                // to place the new gene. This provides consistency with the definition of UMAD
                // in Helmuth et al, lines 6-10 of Algorithm 1.
                if rng.random::<bool>() {
                    [old_gene, new_gene]
                } else {
                    [new_gene, old_gene]
                }
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
    use test_case::test_case;

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

    #[test]
    #[should_panic(expected = "`addition_rate` must be between 0.0 and 1.0 inclusive, but was 1.1")]
    fn panic_if_addition_rate_too_high() {
        let _ = Umad::new_with_balanced_deletion(1.1, ());
    }

    #[test]
    #[should_panic(
        expected = "`addition_rate` must be between 0.0 and 1.0 inclusive, but was -0.2"
    )]
    fn panic_if_addition_rate_too_low() {
        let _ = Umad::new_with_balanced_deletion(-0.2, ());
    }

    #[test_case(0.1, 0.1 / (1.0 + 0.1); "small addition rate")]
    #[test_case(0.0, 0.0; "zero addition rate")]
    #[test_case(1.0, 0.5; "full addition rate")]
    fn correct_balanced_rate_calculation(addition_rate: f64, deletion_rate: f64) {
        let umad = Umad::new_with_balanced_deletion(addition_rate, ());

        assert!(
            (umad.deletion_rate - deletion_rate).abs() < f64::EPSILON,
            "Expected deletion rate {deletion_rate} but got deletion rate {}",
            umad.deletion_rate
        );
    }

    #[test]
    fn test_no_empty_addition_rate() {
        let mut rng = rng();
        // This generates "genes" that are always the character 'x'.
        let char_options = uniform_distribution_of!['x'];
        let umad = Umad::new_without_empty_addition_rate(0.1, 0.1, char_options);
        // A parent with an empty genome
        let parent = Vector { genes: Vec::new() };

        // Since we don't add genes to empty genomes, this should still be an empty
        // genome
        let Ok(child) = umad.mutate(parent, &mut rng);
        assert!(child.genes.is_empty());
    }

    #[test]
    fn test_always_add_to_empty_genome() {
        let mut rng = rng();
        // This generates "genes" that are always the character 'x'.
        let char_options = uniform_distribution_of!['x'];
        let umad = Umad::new_with_empty_addition_rate(0.1, 1.0, 0.1, char_options);
        // A parent with an empty genome
        let parent = Vector { genes: Vec::new() };

        // Since we don't add genes to empty genomes, this should still be an empty
        // genome
        let Ok(child) = umad.mutate(parent, &mut rng);
        assert_eq!(child.genes, ['x']);
    }
}
