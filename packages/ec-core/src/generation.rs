use polonius_the_crab::{polonius, polonius_try};
use rayon::prelude::{FromParallelIterator, ParallelIterator};

use crate::{operator::Operator, population::Population};

/// Collection of data about each iteration in the evolution process.
///
/// Notably, this includes the Population of the current Generation, as well as
/// a child maker [`Operator`] which describes how to generate tne next
/// generation from this.
///
/// Take a look at [`Generation::par_next`] and [`Generation::serial_next`] for
/// further information on how this is used to generate new generations.
///
/// # Example[^ec-linear-usage]
/// ```
/// # use ec_core::{
/// #     test_results::{TestResults, Score},
/// #     individual::{scorer::FnScorer, ec::WithScorer},
/// #     operator::{
/// #         selector::{lexicase::Lexicase, Select},
/// #         genome_extractor::GenomeExtractor,
/// #         recombinator::Recombine,
/// #         mutator::Mutate,
/// #         genome_scorer::GenomeScorer,
/// #         Composable
/// #     },
/// #     distributions::collection::ConvertToCollectionDistribution,
/// #     generation::Generation
/// # };
/// # use std::iter::once;
/// # use ec_linear::{
/// #     genome::bitstring::Bitstring,
/// #     mutator::with_one_over_length::WithOneOverLength,
/// #     recombinator::two_point_xo::TwoPointXo
/// # };
/// # use rand::distr::{StandardUniform, Distribution};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// # #[must_use]
/// # fn hiff(bits: &[bool]) -> (bool, TestResults<Score<usize>>) {
/// #     let len = bits.len();
/// #     if len < 2 {
/// #         (true, once(Score::from(len)).collect())
/// #     } else {
/// #         let half_len = len / 2;
/// #         let (left_all_same, left_score) = hiff(&bits[..half_len]);
/// #         let (right_all_same, right_score) = hiff(&bits[half_len..]);
/// #         let all_same = left_all_same && right_all_same && bits[0] == bits[half_len];
/// #
/// #         (
/// #             all_same,
/// #             left_score
/// #                 .into_iter()
/// #                 .chain(right_score)
/// #                 .chain(once(Score::from(if all_same { len } else { 0 })))
/// #                 .collect(),
/// #         )
/// #     }
/// # }
/// # let mut rng = rand::rng();
/// # let bit_length = 100;
/// # let population_size = 10;
/// # let my_scorer = FnScorer(|bitstring: &Bitstring| hiff(&bitstring.bits).1);
/// # let my_selector = Lexicase::new(2 * bit_length - 1);
/// let initial_population: Vec<_> = StandardUniform
///     .into_collection(bit_length)
///     .with_scorer(my_scorer)
///     .into_collection(population_size)
///     .sample(&mut rng);
///
/// let make_new_individual = Select::new(my_selector)
///     .apply_twice()
///     .then_map(GenomeExtractor)
///     .then(Recombine::new(TwoPointXo))
///     .then(Mutate::new(WithOneOverLength))
///     .wrap::<GenomeScorer<_, _>>(my_scorer);
///
/// let mut generation = Generation::new(make_new_individual, initial_population);
/// generation.par_next()?;
/// let next_generation = generation;
/// # let _ = next_generation;
/// # Ok(())
/// # }
/// ```
///
/// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
///     dependency of this package to demonstrate some concepts which need
///     concrete implementations. If you want to replicate this example, make
///     sure [`ec-linear`][ec-linear] is installed.
///
/// [ec-linear]: #
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Generation<C, P> {
    child_maker: C,
    population: P,
}

impl<C, P> From<(C, P)> for Generation<C, P> {
    /// Convert a `(ChildMaker, Population)` 2-Tuple into a [`Generation`].
    ///
    /// # Example[^ec-linear-usage]
    /// ```
    /// # use ec_core::{
    /// #     test_results::{TestResults, Score},
    /// #     individual::{scorer::FnScorer, ec::WithScorer},
    /// #     operator::{
    /// #         selector::{lexicase::Lexicase, Select},
    /// #         genome_extractor::GenomeExtractor,
    /// #         recombinator::Recombine,
    /// #         mutator::Mutate,
    /// #         genome_scorer::GenomeScorer,
    /// #         Composable
    /// #     },
    /// #     distributions::collection::ConvertToCollectionDistribution,
    /// #     generation::Generation
    /// # };
    /// # use std::iter::once;
    /// # use ec_linear::{
    /// #     genome::bitstring::Bitstring,
    /// #     mutator::with_one_over_length::WithOneOverLength,
    /// #     recombinator::two_point_xo::TwoPointXo
    /// # };
    /// # use rand::distr::{StandardUniform, Distribution};
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # #[must_use]
    /// # fn hiff(bits: &[bool]) -> (bool, TestResults<Score<usize>>) {
    /// #     let len = bits.len();
    /// #     if len < 2 {
    /// #         (true, once(Score::from(len)).collect())
    /// #     } else {
    /// #         let half_len = len / 2;
    /// #         let (left_all_same, left_score) = hiff(&bits[..half_len]);
    /// #         let (right_all_same, right_score) = hiff(&bits[half_len..]);
    /// #         let all_same = left_all_same && right_all_same && bits[0] == bits[half_len];
    /// #
    /// #         (
    /// #             all_same,
    /// #             left_score
    /// #                 .into_iter()
    /// #                 .chain(right_score)
    /// #                 .chain(once(Score::from(if all_same { len } else { 0 })))
    /// #                 .collect(),
    /// #         )
    /// #     }
    /// # }
    /// # let mut rng = rand::rng();
    /// # let bit_length = 100;
    /// # let population_size = 10;
    /// # let my_scorer = FnScorer(|bitstring: &Bitstring| hiff(&bitstring.bits).1);
    /// # let my_selector = Lexicase::new(2 * bit_length - 1);
    /// let initial_population: Vec<_> = StandardUniform
    ///     .into_collection(bit_length)
    ///     .with_scorer(my_scorer)
    ///     .into_collection(population_size)
    ///     .sample(&mut rng);
    ///
    /// let make_new_individual = Select::new(my_selector)
    ///     .apply_twice()
    ///     .then_map(GenomeExtractor)
    ///     .then(Recombine::new(TwoPointXo))
    ///     .then(Mutate::new(WithOneOverLength))
    ///     .wrap::<GenomeScorer<_, _>>(my_scorer);
    ///
    /// let mut generation: Generation<_, _> = (make_new_individual, initial_population).into();
    /// generation.par_next()?;
    /// let next_generation = generation;
    /// # let _ = next_generation;
    /// # Ok(())
    /// # }
    /// ```
    /// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
    ///     dependency of this package to demonstrate some concepts which need
    ///     concrete implementations. If you want to replicate this example,
    ///     make sure [`ec-linear`][ec-linear] is installed.
    ///
    /// [ec-linear]: #
    fn from((child_maker, population): (C, P)) -> Self {
        Self {
            child_maker,
            population,
        }
    }
}

impl<P, C> Generation<C, P> {
    /// Get a reference to the current population of this [`Generation`].
    ///
    /// # Example[^ec-linear-usage]
    /// ```
    /// # use ec_core::{
    /// #     test_results::{TestResults, Score},
    /// #     individual::{scorer::FnScorer, ec::WithScorer},
    /// #     operator::{
    /// #         selector::{lexicase::Lexicase, Select},
    /// #         genome_extractor::GenomeExtractor,
    /// #         recombinator::Recombine,
    /// #         mutator::Mutate,
    /// #         genome_scorer::GenomeScorer,
    /// #         Composable
    /// #     },
    /// #     distributions::collection::ConvertToCollectionDistribution,
    /// #     generation::Generation
    /// # };
    /// # use std::iter::once;
    /// # use ec_linear::{
    /// #     genome::bitstring::Bitstring,
    /// #     mutator::with_one_over_length::WithOneOverLength,
    /// #     recombinator::two_point_xo::TwoPointXo
    /// # };
    /// # use rand::distr::{StandardUniform, Distribution};
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # #[must_use]
    /// # fn hiff(bits: &[bool]) -> (bool, TestResults<Score<usize>>) {
    /// #     let len = bits.len();
    /// #     if len < 2 {
    /// #         (true, once(Score::from(len)).collect())
    /// #     } else {
    /// #         let half_len = len / 2;
    /// #         let (left_all_same, left_score) = hiff(&bits[..half_len]);
    /// #         let (right_all_same, right_score) = hiff(&bits[half_len..]);
    /// #         let all_same = left_all_same && right_all_same && bits[0] == bits[half_len];
    /// #
    /// #         (
    /// #             all_same,
    /// #             left_score
    /// #                 .into_iter()
    /// #                 .chain(right_score)
    /// #                 .chain(once(Score::from(if all_same { len } else { 0 })))
    /// #                 .collect(),
    /// #         )
    /// #     }
    /// # }
    /// # let mut rng = rand::rng();
    /// # let bit_length = 100;
    /// # let population_size = 10;
    /// # let my_scorer = FnScorer(|bitstring: &Bitstring| hiff(&bitstring.bits).1);
    /// # let my_selector = Lexicase::new(2 * bit_length - 1);
    /// let initial_population: Vec<_> = StandardUniform
    ///     .into_collection(bit_length)
    ///     .with_scorer(my_scorer)
    ///     .into_collection(population_size)
    ///     .sample(&mut rng);
    ///
    /// let make_new_individual = Select::new(my_selector)
    ///     .apply_twice()
    ///     .then_map(GenomeExtractor)
    ///     .then(Recombine::new(TwoPointXo))
    ///     .then(Mutate::new(WithOneOverLength))
    ///     .wrap::<GenomeScorer<_, _>>(my_scorer);
    ///
    /// let generation = Generation::new(make_new_individual, initial_population.clone());
    /// let my_population = generation.population();
    ///
    /// assert_eq!(my_population, &initial_population);
    /// # Ok(())
    /// # }
    /// ```
    /// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
    ///     dependency of this package to demonstrate some concepts which need
    ///     concrete implementations. If you want to replicate this example,
    ///     make sure [`ec-linear`][ec-linear] is installed.
    ///
    /// [ec-linear]: #
    pub const fn population(&self) -> &P {
        &self.population
    }

    /// Extract the current population of this [`Generation`].
    ///
    /// # Example[^ec-linear-usage]
    /// ```
    /// # use ec_core::{
    /// #     test_results::{TestResults, Score},
    /// #     individual::{scorer::FnScorer, ec::WithScorer},
    /// #     operator::{
    /// #         selector::{lexicase::Lexicase, Select},
    /// #         genome_extractor::GenomeExtractor,
    /// #         recombinator::Recombine,
    /// #         mutator::Mutate,
    /// #         genome_scorer::GenomeScorer,
    /// #         Composable
    /// #     },
    /// #     distributions::collection::ConvertToCollectionDistribution,
    /// #     generation::Generation
    /// # };
    /// # use std::iter::once;
    /// # use ec_linear::{
    /// #     genome::bitstring::Bitstring,
    /// #     mutator::with_one_over_length::WithOneOverLength,
    /// #     recombinator::two_point_xo::TwoPointXo
    /// # };
    /// # use rand::distr::{StandardUniform, Distribution};
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # #[must_use]
    /// # fn hiff(bits: &[bool]) -> (bool, TestResults<Score<usize>>) {
    /// #     let len = bits.len();
    /// #     if len < 2 {
    /// #         (true, once(Score::from(len)).collect())
    /// #     } else {
    /// #         let half_len = len / 2;
    /// #         let (left_all_same, left_score) = hiff(&bits[..half_len]);
    /// #         let (right_all_same, right_score) = hiff(&bits[half_len..]);
    /// #         let all_same = left_all_same && right_all_same && bits[0] == bits[half_len];
    /// #
    /// #         (
    /// #             all_same,
    /// #             left_score
    /// #                 .into_iter()
    /// #                 .chain(right_score)
    /// #                 .chain(once(Score::from(if all_same { len } else { 0 })))
    /// #                 .collect(),
    /// #         )
    /// #     }
    /// # }
    /// # let mut rng = rand::rng();
    /// # let bit_length = 100;
    /// # let population_size = 10;
    /// # let my_scorer = FnScorer(|bitstring: &Bitstring| hiff(&bitstring.bits).1);
    /// # let my_selector = Lexicase::new(2 * bit_length - 1);
    /// let initial_population: Vec<_> = StandardUniform
    ///     .into_collection(bit_length)
    ///     .with_scorer(my_scorer)
    ///     .into_collection(population_size)
    ///     .sample(&mut rng);
    ///
    /// let make_new_individual = Select::new(my_selector)
    ///     .apply_twice()
    ///     .then_map(GenomeExtractor)
    ///     .then(Recombine::new(TwoPointXo))
    ///     .then(Mutate::new(WithOneOverLength))
    ///     .wrap::<GenomeScorer<_, _>>(my_scorer);
    ///
    /// let generation = Generation::new(make_new_individual, initial_population.clone());
    /// let my_population = generation.into_population();
    ///
    /// assert_eq!(my_population, initial_population);
    /// # Ok(())
    /// # }
    /// ```
    /// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
    ///     dependency of this package to demonstrate some concepts which need
    ///     concrete implementations. If you want to replicate this example,
    ///     make sure [`ec-linear`][ec-linear] is installed.
    ///
    /// [ec-linear]: #
    pub fn into_population(self) -> P {
        self.population
    }
}

impl<P, C> Generation<C, P> {
    /// Create a new [`Generation`] from a `ChildMaker` and a `Population`.
    ///
    /// # Example[^ec-linear-usage]
    /// ```
    /// # use ec_core::{
    /// #     test_results::{TestResults, Score},
    /// #     individual::{scorer::FnScorer, ec::WithScorer},
    /// #     operator::{
    /// #         selector::{lexicase::Lexicase, Select},
    /// #         genome_extractor::GenomeExtractor,
    /// #         recombinator::Recombine,
    /// #         mutator::Mutate,
    /// #         genome_scorer::GenomeScorer,
    /// #         Composable
    /// #     },
    /// #     distributions::collection::ConvertToCollectionDistribution,
    /// #     generation::Generation
    /// # };
    /// # use std::iter::once;
    /// # use ec_linear::{
    /// #     genome::bitstring::Bitstring,
    /// #     mutator::with_one_over_length::WithOneOverLength,
    /// #     recombinator::two_point_xo::TwoPointXo
    /// # };
    /// # use rand::distr::{StandardUniform, Distribution};
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # #[must_use]
    /// # fn hiff(bits: &[bool]) -> (bool, TestResults<Score<usize>>) {
    /// #     let len = bits.len();
    /// #     if len < 2 {
    /// #         (true, once(Score::from(len)).collect())
    /// #     } else {
    /// #         let half_len = len / 2;
    /// #         let (left_all_same, left_score) = hiff(&bits[..half_len]);
    /// #         let (right_all_same, right_score) = hiff(&bits[half_len..]);
    /// #         let all_same = left_all_same && right_all_same && bits[0] == bits[half_len];
    /// #
    /// #         (
    /// #             all_same,
    /// #             left_score
    /// #                 .into_iter()
    /// #                 .chain(right_score)
    /// #                 .chain(once(Score::from(if all_same { len } else { 0 })))
    /// #                 .collect(),
    /// #         )
    /// #     }
    /// # }
    /// # let mut rng = rand::rng();
    /// # let bit_length = 100;
    /// # let population_size = 10;
    /// # let my_scorer = FnScorer(|bitstring: &Bitstring| hiff(&bitstring.bits).1);
    /// # let my_selector = Lexicase::new(2 * bit_length - 1);
    /// let initial_population: Vec<_> = StandardUniform
    ///     .into_collection(bit_length)
    ///     .with_scorer(my_scorer)
    ///     .into_collection(population_size)
    ///     .sample(&mut rng);
    ///
    /// let make_new_individual = Select::new(my_selector)
    ///     .apply_twice()
    ///     .then_map(GenomeExtractor)
    ///     .then(Recombine::new(TwoPointXo))
    ///     .then(Mutate::new(WithOneOverLength))
    ///     .wrap::<GenomeScorer<_, _>>(my_scorer);
    ///
    /// let generation = Generation::new(make_new_individual, initial_population);
    /// # let _ = generation;
    /// # Ok(())
    /// # }
    /// ```
    /// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
    ///     dependency of this package to demonstrate some concepts which need
    ///     concrete implementations. If you want to replicate this example,
    ///     make sure [`ec-linear`][ec-linear] is installed.
    ///
    /// [ec-linear]: #
    pub const fn new(child_maker: C, population: P) -> Self {
        Self {
            child_maker,
            population,
        }
    }
}

impl<P, C> Generation<C, P>
where
    P: Population + FromParallelIterator<P::Individual> + Send + Sync,
    P::Individual: Send,
    for<'a> C: Operator<&'a P, Output = P::Individual, Error: Send> + Send + Sync,
{
    /// Create a new [`Generation`] in-place using the current `Population` and
    /// `ChildMaker`.
    ///
    /// This version uses [`rayon`](mod@::rayon) to parallelize the generation
    /// of children for each individual. For a serial (single-thread)
    /// version, see [`Generation::serial_next`].
    ///
    /// # Example[^ec-linear-usage]
    /// ```
    /// # use ec_core::{
    /// #     test_results::{TestResults, Score},
    /// #     individual::{scorer::FnScorer, ec::WithScorer},
    /// #     operator::{
    /// #         selector::{lexicase::Lexicase, Select},
    /// #         genome_extractor::GenomeExtractor,
    /// #         recombinator::Recombine,
    /// #         mutator::Mutate,
    /// #         genome_scorer::GenomeScorer,
    /// #         Composable
    /// #     },
    /// #     distributions::collection::ConvertToCollectionDistribution,
    /// #     generation::Generation
    /// # };
    /// # use std::iter::once;
    /// # use ec_linear::{
    /// #     genome::bitstring::Bitstring,
    /// #     mutator::with_one_over_length::WithOneOverLength,
    /// #     recombinator::two_point_xo::TwoPointXo
    /// # };
    /// # use rand::distr::{StandardUniform, Distribution};
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # #[must_use]
    /// # fn hiff(bits: &[bool]) -> (bool, TestResults<Score<usize>>) {
    /// #     let len = bits.len();
    /// #     if len < 2 {
    /// #         (true, once(Score::from(len)).collect())
    /// #     } else {
    /// #         let half_len = len / 2;
    /// #         let (left_all_same, left_score) = hiff(&bits[..half_len]);
    /// #         let (right_all_same, right_score) = hiff(&bits[half_len..]);
    /// #         let all_same = left_all_same && right_all_same && bits[0] == bits[half_len];
    /// #
    /// #         (
    /// #             all_same,
    /// #             left_score
    /// #                 .into_iter()
    /// #                 .chain(right_score)
    /// #                 .chain(once(Score::from(if all_same { len } else { 0 })))
    /// #                 .collect(),
    /// #         )
    /// #     }
    /// # }
    /// # let mut rng = rand::rng();
    /// # let bit_length = 100;
    /// # let population_size = 10;
    /// # let my_scorer = FnScorer(|bitstring: &Bitstring| hiff(&bitstring.bits).1);
    /// # let my_selector = Lexicase::new(2 * bit_length - 1);
    /// let initial_population: Vec<_> = StandardUniform
    ///     .into_collection(bit_length)
    ///     .with_scorer(my_scorer)
    ///     .into_collection(population_size)
    ///     .sample(&mut rng);
    ///
    /// let make_new_individual = Select::new(my_selector)
    ///     .apply_twice()
    ///     .then_map(GenomeExtractor)
    ///     .then(Recombine::new(TwoPointXo))
    ///     .then(Mutate::new(WithOneOverLength))
    ///     .wrap::<GenomeScorer<_, _>>(my_scorer);
    ///
    /// let mut generation = Generation::new(make_new_individual, initial_population);
    /// generation.par_next()?;
    /// let next_generation = generation;
    /// # let _ = next_generation;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
    ///     dependency of this package to demonstrate some concepts which need
    ///     concrete implementations. If you want to replicate this example,
    ///     make sure [`ec-linear`][ec-linear] is installed.
    ///
    /// # Errors
    ///
    /// - `C::Error` if applying the `ChildMaker`-[`Operator`] fails.
    ///
    /// [ec-linear]: #
    pub fn par_next(&mut self) -> Result<(), <C as Operator<&P>>::Error> {
        // Should be able to be removed along with workaround
        let mut alias = self;

        // this is the code that should work, but currently doesn't because of NLL
        // limitations (should compile in future versions of rust just fine)
        // let population = rayon::iter::repeat_n(&self.population,
        // self.population.size())     .map_init(rand::rng, |rng, p|
        // self.child_maker.apply(p, rng))     .collect::<Result<_, _>>()?;

        // Workaround for current compiler limitations

        let new_population = polonius!(
            |alias| -> Result<(), <C as Operator<&'polonius P>>::Error> {
                polonius_try!(
                    rayon::iter::repeat_n(&alias.population, alias.population.size())
                        .map_init(rand::rng, |rng, p| alias.child_maker.apply(p, rng))
                        .collect::<Result<_, _>>()
                )
            }
        );

        // end of workaround

        // TODO: We can reduce allocations by pre-allocating the memory for "old" and
        // "new"   population in `::new()` and then re-using those vectors here.
        alias.population = new_population;

        Ok(())
    }
}

impl<P, C> Generation<C, P>
where
    P: Population + FromIterator<P::Individual>,
    C: for<'a> Operator<&'a P, Output = P::Individual>,
{
    /// Create a new [`Generation`] in-place using the current `Population` and
    /// `ChildMaker`.
    ///
    /// This version uses a simple serial loop to generate the children
    /// for each individual. For a parallel (rayon)
    /// version, see [`Generation::par_next`].
    ///
    /// # Example[^ec-linear-usage]
    /// ```
    /// # use ec_core::{
    /// #     test_results::{TestResults, Score},
    /// #     individual::{scorer::FnScorer, ec::WithScorer},
    /// #     operator::{
    /// #         selector::{lexicase::Lexicase, Select},
    /// #         genome_extractor::GenomeExtractor,
    /// #         recombinator::Recombine,
    /// #         mutator::Mutate,
    /// #         genome_scorer::GenomeScorer,
    /// #         Composable
    /// #     },
    /// #     distributions::collection::ConvertToCollectionDistribution,
    /// #     generation::Generation
    /// # };
    /// # use std::iter::once;
    /// # use ec_linear::{
    /// #     genome::bitstring::Bitstring,
    /// #     mutator::with_one_over_length::WithOneOverLength,
    /// #     recombinator::two_point_xo::TwoPointXo
    /// # };
    /// # use rand::distr::{StandardUniform, Distribution};
    /// #
    /// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// # #[must_use]
    /// # fn hiff(bits: &[bool]) -> (bool, TestResults<Score<usize>>) {
    /// #     let len = bits.len();
    /// #     if len < 2 {
    /// #         (true, once(Score::from(len)).collect())
    /// #     } else {
    /// #         let half_len = len / 2;
    /// #         let (left_all_same, left_score) = hiff(&bits[..half_len]);
    /// #         let (right_all_same, right_score) = hiff(&bits[half_len..]);
    /// #         let all_same = left_all_same && right_all_same && bits[0] == bits[half_len];
    /// #
    /// #         (
    /// #             all_same,
    /// #             left_score
    /// #                 .into_iter()
    /// #                 .chain(right_score)
    /// #                 .chain(once(Score::from(if all_same { len } else { 0 })))
    /// #                 .collect(),
    /// #         )
    /// #     }
    /// # }
    /// # let mut rng = rand::rng();
    /// # let bit_length = 100;
    /// # let population_size = 10;
    /// # let my_scorer = FnScorer(|bitstring: &Bitstring| hiff(&bitstring.bits).1);
    /// # let my_selector = Lexicase::new(2 * bit_length - 1);
    /// let initial_population: Vec<_> = StandardUniform
    ///     .into_collection(bit_length)
    ///     .with_scorer(my_scorer)
    ///     .into_collection(population_size)
    ///     .sample(&mut rng);
    ///
    /// let make_new_individual = Select::new(my_selector)
    ///     .apply_twice()
    ///     .then_map(GenomeExtractor)
    ///     .then(Recombine::new(TwoPointXo))
    ///     .then(Mutate::new(WithOneOverLength))
    ///     .wrap::<GenomeScorer<_, _>>(my_scorer);
    ///
    /// let mut generation = Generation::new(make_new_individual, initial_population);
    /// generation.serial_next()?;
    /// let next_generation = generation;
    /// # let _ = next_generation;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
    ///     dependency of this package to demonstrate some concepts which need
    ///     concrete implementations. If you want to replicate this example,
    ///     make sure [`ec-linear`][ec-linear] is installed.
    ///
    /// # Errors
    ///
    /// - `C::Error` if applying the `ChildMaker`-[`Operator`] fails.
    ///
    /// [ec-linear]: #
    pub fn serial_next(&mut self) -> Result<(), <C as Operator<&P>>::Error> {
        let mut alias = self;
        let mut rng = rand::rng();

        // this is the code that should work, but currently doesn't because of NLL
        // limitations (should compile in future versions of rust just fine)
        // let new_population = std::iter::repeat_n(&self.population,
        // self.population.size())     .map(|p| self.child_maker.apply(p, &mut
        // rng))     .collect::<Result<_, _>>()?;

        // Workaround for current compiler limitations

        let new_population = polonius!(
            |alias| -> Result<(), <C as Operator<&'polonius P>>::Error> {
                polonius_try!(
                    std::iter::repeat_n(&alias.population, alias.population.size())
                        .map(|p| alias.child_maker.apply(p, &mut rng))
                        .collect::<Result<_, _>>()
                )
            }
        );

        // TODO: We can reduce allocations by pre-allocating the memory for "old" and
        // "new"   population in `::new()` and then re-using those vectors here.
        alias.population = new_population;
        Ok(())
    }
}
