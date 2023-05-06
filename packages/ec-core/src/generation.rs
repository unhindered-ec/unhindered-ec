use anyhow::Result;
use itertools::Itertools;
use rayon::prelude::{FromParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{operator::Operator, population::Population};

pub struct Generation<P, C> {
    population: P,
    child_maker: C,
}

impl<P, C> Generation<P, C> {
    pub const fn population(&self) -> &P {
        &self.population
    }
}

impl<P, C> Generation<P, C> {
    pub const fn new(child_maker: C, population: P) -> Self {
        Self {
            population,
            child_maker,
        }
    }
}

impl<P, C> Generation<P, C>
where
    P: Population + FromParallelIterator<P::Individual> + Send + Sync,
    P::Individual: Send,
    C: for<'a> Operator<&'a P, Output = P::Individual> + Send + Sync,
{
    /// Make the next generation using a Rayon parallel iterator.
    /// # Errors
    ///
    /// This can return errors if any aspect of creating the next generation fail. That can include constructing
    /// or scoring the genomes.
    pub fn par_next(&mut self) -> anyhow::Result<()> {
        let pop_size = self.population.size();
        let population = (0..pop_size)
            .into_par_iter()
            .map_init(rand::thread_rng, |rng, _| {
                self.child_maker.apply(&self.population, rng)
            })
            .collect::<Result<_>>()?;
        // TODO: We can reduce allocations by pre-allocating the memory for "old" and "new"
        //   population in `::new()` and then re-using those vectors here.
        self.population = population;
        Ok(())
    }
}

impl<P, C> Generation<P, C>
where
    P: Population + FromIterator<P::Individual>,
    C: for<'a> Operator<&'a P, Output = P::Individual>,
{
    /// Make the next generation serially.
    /// # Errors
    ///
    /// This can return errors if any aspect of creating the next generation fail. That can include constructing
    /// or scoring the genomes.
    pub fn serial_next(&mut self) -> anyhow::Result<()> {
        let pop_size = self.population.size();
        let mut rng = rand::thread_rng();
        // Switch to `repeat_with` and `take`
        let new_population = (0..pop_size)
            .map(|_| self.child_maker.apply(&self.population, &mut rng))
            .try_collect()?;
        // TODO: We can reduce allocations by pre-allocating the memory for "old" and "new"
        //   population in `::new()` and then re-using those vectors here.
        self.population = new_population;
        Ok(())
    }
}
