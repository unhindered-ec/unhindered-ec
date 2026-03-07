use rand::Rng;

use super::{Composable, Operator, composable::Wrappable};
use crate::{
    individual::{ec::EcIndividual, scorer::Scorer},
    population::Population,
};

/// An [`Operator`] which takes a `GenomeMaker` [`Operator`] and a [`Scorer`]
/// and creates an indidividual by creating a new genome and scoring it.
///
/// Concretely this [`Operator`] is applied to a Population and then does the
/// following:
/// 1. Call the `GenomeMaker` with reference to the passed population to create
///    a new genome,
/// 2. Score the resulting genome using the passed `Scorer`.
/// 3. Return the resulting `EcIndividual`.
///
/// This means this [`Operator`] is typically applied to a population, not a
/// single genome, which is passed to the `GenomeMaker`, but it only returns a
/// single individual.
///
/// # Example
/// ```
/// # use ec_core::{
/// #     individual::{ec::EcIndividual, scorer::FnScorer},
/// #     operator::{Operator, constant::Constant, genome_scorer::GenomeScorer},
/// # };
/// let genome_maker = Constant::new(100);
/// let scorer = FnScorer(|x: &i32| 10i32.abs_diff(*x));
///
/// let genome_scorer = GenomeScorer::new(genome_maker, scorer);
///
/// let population = [0i32; 0];
/// let result = genome_scorer.apply(&population, &mut rand::rng())?;
///
/// assert_eq!(result, EcIndividual::new(100, 90));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Composable, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GenomeScorer<GM, S> {
    genome_maker: GM,
    scorer: S,
}

impl<GM, S> From<(GM, S)> for GenomeScorer<GM, S> {
    /// Convert a tuple of a genome maker and a scorer to a new
    /// [`GenomeScorer`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::{operator::{Operator, constant::Constant, genome_scorer::GenomeScorer}, individual::{ec::EcIndividual, scorer::FnScorer}};
    /// let genome_maker = Constant::new(100);
    /// let scorer = FnScorer(|x: &i32| 10i32.abs_diff(*x));
    ///
    /// let genome_scorer = GenomeScorer::from((genome_maker, scorer));
    /// #
    /// # let result = genome_scorer.apply(&[0i32; 0], &mut rand::rng())?;
    /// #
    /// # assert_eq!(result, EcIndividual::new(100, 90));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn from((genome_maker, scorer): (GM, S)) -> Self {
        Self::new(genome_maker, scorer)
    }
}

impl<G, S> GenomeScorer<G, S> {
    /// Create a new [`GenomeScorer`] [`Operator`] from a `GenomeMaker` and a
    /// [`Scorer`], scoring the genome and creating a [`EcIndividual`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     individual::{ec::EcIndividual, scorer::FnScorer},
    /// #     operator::{Operator, constant::Constant, genome_scorer::GenomeScorer},
    /// # };
    /// let genome_maker = Constant::new(100);
    /// let scorer = FnScorer(|x: &i32| 10i32.abs_diff(*x));
    ///
    /// let genome_scorer = GenomeScorer::new(genome_maker, scorer);
    /// #
    /// # let result = genome_scorer.apply(&[0i32; 0], &mut rand::rng())?;
    /// #
    /// # assert_eq!(result, EcIndividual::new(100, 90));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn new(genome_maker: G, scorer: S) -> Self {
        Self {
            genome_maker,
            scorer,
        }
    }
}

impl<G, S> Wrappable<G> for GenomeScorer<G, S> {
    type Context = S;

    /// Chainable [`Operator`] adapter to construct a [`GenomeScorer`] using
    /// [`Composable::wrap`].
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     individual::{ec::EcIndividual, scorer::FnScorer},
    /// #     operator::{Composable, Operator, constant::Constant, genome_scorer::GenomeScorer},
    /// # };
    ///
    /// let genome_maker = Constant::new(100);
    /// let scorer = FnScorer(|x: &i32| 10i32.abs_diff(*x));
    ///
    /// let genome_scorer = genome_maker.wrap::<GenomeScorer<_, _>>(scorer);
    ///
    /// let population = [0i32; 0];
    /// let result = genome_scorer.apply(&population, &mut rand::rng())?;
    ///
    /// assert_eq!(result, EcIndividual::new(100, 90));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn construct(genome_maker: G, scorer: Self::Context) -> Self {
        Self::new(genome_maker, scorer)
    }
}

// scorer: &Genome -> TestResults<R>
impl<'pop, GM, S, P> Operator<&'pop P> for GenomeScorer<GM, S>
where
    P: Population,
    GM: Operator<&'pop P>,
    S: Scorer<GM::Output>,
{
    type Output = EcIndividual<GM::Output, S::Score>;
    type Error = GM::Error;

    /// Generate a genome using the `GenomeMaker` [`Operator`], and then score
    /// it using the provided [`Scorer`], returning a [`EcIndividual`].
    ///
    /// # Errors
    /// The genome maker [`Operator`]'s error, if making the genome fails.
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     individual::{ec::EcIndividual, scorer::FnScorer},
    /// #     operator::{Operator, constant::Constant, genome_scorer::GenomeScorer},
    /// # };
    /// let genome_maker = Constant::new(100);
    /// let scorer = FnScorer(|x: &i32| 10i32.abs_diff(*x));
    ///
    /// let genome_scorer = GenomeScorer::new(genome_maker, scorer);
    ///
    /// let result = genome_scorer.apply(&[0i32; 0], &mut rand::rng())?;
    /// #
    /// # assert_eq!(result, EcIndividual::new(100, 90));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn apply<R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
    ) -> Result<Self::Output, Self::Error> {
        let genome = self.genome_maker.apply(population, rng)?;
        let score = self.scorer.score(&genome);
        Ok(EcIndividual::new(genome, score))
    }
}
