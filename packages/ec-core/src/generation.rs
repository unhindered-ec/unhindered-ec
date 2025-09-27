use polonius_the_crab::{polonius, polonius_try};
use rayon::prelude::{FromParallelIterator, ParallelIterator};

use crate::{operator::Operator, population::Population};

/// Collection of data about each iteration in the evolution process.
///
/// Notably, this includes the Population of the current Generation, as well as
/// a child maker `Generator` which describes how to generate tne next
/// generation from this.
///
/// Take a look at [`Generation::par_next`] and [`Generation::serial_next`] for
/// further information on how this is used to generate new generations.
///
/// # Example[^ec-linear-usage]
/// ```
/// let make_new_individual = Select::new(selector)
///     .apply_twice()
///     .then_map(GenomeExtractor)
///     .then(Recombine::new(TwoPointXo))
///     .then(Mutate::new(WithOneOverLength))
///     .wrap::<GenomeScorer<_, _>>(scorer);
///
/// let generation =
/// ```
///
/// [^ec-linear-usage]: Note that this example uses [`ec-linear`](#) which is not a
///     dependency of this package to demonstrate some concepts which need
///     concrete implementations. If you want to replicate this example, make
///     sure [`ec-linear`](#) is installed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Generation<C, P> {
    child_maker: C,
    population: P,
}

impl<C, P> From<(C, P)> for Generation<C, P> {
    fn from((child_maker, population): (C, P)) -> Self {
        Self {
            child_maker,
            population,
        }
    }
}

impl<P, C> Generation<C, P> {
    pub const fn population(&self) -> &P {
        &self.population
    }

    pub fn into_population(self) -> P {
        self.population
    }
}

impl<P, C> Generation<C, P> {
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
    /// Make the next generation using a Rayon parallel iterator.
    /// # Errors
    ///
    /// This can return errors if any aspect of creating the next generation
    /// fail. That can include constructing or scoring the genomes.
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
    /// Make the next generation serially.
    /// # Errors
    ///
    /// This can return errors if any aspect of creating the next generation
    /// fail. That can include constructing or scoring the genomes.
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
