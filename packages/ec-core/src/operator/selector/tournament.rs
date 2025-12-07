use std::num::NonZeroUsize;

use miette::Diagnostic;
use rand::{Rng, prelude::IndexedRandom};

use super::Selector;
use crate::population::Population;

/// Tournament selector.
///
/// This selector works by creating a tournament and letting `size` randomly
/// selected Individuals face off against each other, where the best one wins.
///
/// # Examples
/// With a tournament of size `3` (a ternary tournament):
/// ```
/// # use ec_core::operator::selector::{Selector, tournament::{Tournament, TournamentSizeError}};
/// #
/// let population = [1, 2, 5, 8];
///
/// let tournament = Tournament::of_size::<3>();
/// let selected = tournament.select(&population, &mut rand::rng())?;
///
/// assert!([5, 8].contains(selected));
/// # Ok::<(), TournamentSizeError>(())
/// ```
/// With a binary tournament:
/// ```
/// # use ec_core::operator::selector::{Selector, tournament::{Tournament, TournamentSizeError}};
/// #
/// let population = [1, 2, 5, 8];
///
/// let tournament = Tournament::binary();
/// let selected = tournament.select(&population, &mut rand::rng())?;
///
/// assert!([2, 5, 8].contains(selected));
/// # Ok::<(), TournamentSizeError>(())
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tournament {
    size: NonZeroUsize,
}

impl Default for Tournament {
    /// Construct a default (binary) tournament
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::tournament::Tournament;
    /// #
    /// let default = Tournament::default();
    /// let tournament = Tournament::binary();
    ///
    /// assert_eq!(default, tournament)
    /// ```
    fn default() -> Self {
        Self::of_size::<2>()
    }
}

/// Error that occurs when trying to perform [`Tournament`] selection on a
/// population with a size smaller than the tournament size
#[derive(Debug, thiserror::Error, Diagnostic, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error("Tournament size {tournament_size} was larger than population size {population_size}")]
#[diagnostic(help = "Ensure that the population has at least {tournament_size} individuals")]
// invariant: tournament_size > population_size, please use the constructor
// functions
pub struct TournamentSizeError {
    tournament_size: NonZeroUsize,
    population_size: usize,
}

impl Default for TournamentSizeError {
    fn default() -> Self {
        Self {
            tournament_size: NonZeroUsize::MIN,
            population_size: 0,
        }
    }
}

impl TournamentSizeError {
    /// Construct a new tournament size error from size of the population and
    /// tournament
    ///
    /// # Errors
    /// Returns `None` when the invarant that `tournament_size >
    /// population_size` is not upheld.
    #[must_use]
    pub const fn new(tournament_size: NonZeroUsize, population_size: usize) -> Option<Self> {
        if tournament_size.get() <= population_size {
            return None;
        }

        Some(Self {
            tournament_size,
            population_size,
        })
    }

    /// Construct a new tournament size error from the size of the population as
    /// well as the difference from the expected tournament size minimum.
    #[must_use]
    pub const fn new_diff(population_size: usize, num_fewer: NonZeroUsize) -> Self {
        Self {
            tournament_size: num_fewer.saturating_add(population_size),
            population_size,
        }
    }
}

impl Tournament {
    /// Construct a tournament selector with the given tournament size.
    ///
    /// This will select `size` individuals from the population, randomly
    /// without replacement, and return the "best" from that set.
    ///
    /// If the size you construct from is a constant, consider using
    /// [`Tournament::of_size`] instead.
    ///
    /// # Example
    ///
    /// Create a tournament selector with tournament size 3:
    /// ```
    /// # use ec_core::operator::selector::tournament::Tournament;
    /// # use std::num::NonZero;
    /// #
    /// let selector = Tournament::new(const { NonZero::new(3).unwrap() });
    /// # let _ = selector;
    /// ```
    #[must_use]
    pub const fn new(size: NonZeroUsize) -> Self {
        Self { size }
    }

    /// Construct a tournament selector with the given _constant_
    /// tournament size.
    ///
    /// This allows for compile-time checks that the
    /// tournament size `N` is greater than zero, and as such is easier to use
    /// (no [`NonZero`](std::num::NonZero) required).
    ///
    /// This selector will select `N` individuals from the population, randomly
    /// without replacement, and return the "best" from that set.
    ///
    /// If you instead only have a dynamic tournament size, consider using
    /// [`Tournament::new`] instead.
    ///
    /// # Example
    ///
    /// Create a tournament selector with constant tournament size 3:
    /// ```
    /// # use ec_core::operator::selector::tournament::Tournament;
    /// #
    /// let selector = Tournament::of_size::<3>();
    /// # let _ = selector;
    /// ```
    #[must_use]
    pub const fn of_size<const N: usize>() -> Self {
        Self::new(
            const {
                match NonZeroUsize::new(N) {
                    Some(x) => x,
                    None => panic!("only positive tournament sizes are permitted"),
                }
            },
        )
    }

    /// Construct a binary tournament selector.
    ///
    /// This is a tournament
    /// selector that selects two random individuals from the population
    /// and returns the "better" of the two.
    ///
    /// If you instead want a varying tournament size, consider using
    /// [`Tournament::of_size`] or [`Tournament::new`] instead.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::tournament::Tournament;
    /// #
    /// let selector = Tournament::binary();
    /// # let _ = selector;
    /// ```
    #[must_use]
    pub const fn binary() -> Self {
        Self::of_size::<2>()
    }
}

impl<P> Selector<P> for Tournament
where
    P: Population + AsRef<[P::Individual]>,
    P::Individual: Ord,
{
    type Error = TournamentSizeError;

    /// Select an Individual from the given Population using tournament
    /// selection.
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::selector::{Selector, tournament::{Tournament, TournamentSizeError}};
    /// #
    /// let population = [1, 2, 5, 8];
    ///
    /// let tournament = Tournament::binary();
    /// let selected = tournament.select(&population, &mut rand::rng())?;
    ///
    /// assert!([2, 5, 8].contains(selected));
    /// # Ok::<(), TournamentSizeError>(())
    /// ```
    ///
    /// # Errors
    /// - [`TournamentSizeError`] if the population is of smaller size than the
    ///   tournament
    fn select<'pop, R: Rng + ?Sized>(
        &self,
        population: &'pop P,
        rng: &mut R,
    ) -> Result<&'pop P::Individual, Self::Error> {
        if let Some(err) = TournamentSizeError::new(self.size, population.size()) {
            return Err(err);
        }
        population
            .as_ref()
            .choose_multiple(rng, self.size.into())
            .max()
            // This should never happen, because an empty population will cause the
            // `if` test test to return an `Err` since `self.size` is guaranteed to
            // be greater than zero.
            .ok_or_else(|| unreachable!("Population can't be empty here"))
    }
}

#[cfg(test)]
mod tests {
    use std::{num::NonZeroUsize, ops::Not};

    use rand::rng;
    use test_strategy::proptest;

    use super::Tournament;
    use crate::{
        individual::ec::EcIndividual,
        operator::selector::{Selector, tournament::TournamentSizeError},
    };

    #[test]
    fn empty_population() {
        let pop: Vec<i32> = Vec::new();
        let mut rng = rand::rng();
        let selector = Tournament::new(NonZeroUsize::MIN);
        let expected_error = TournamentSizeError::default();
        assert_eq!(selector.select(&pop, &mut rng), Err(expected_error));
        assert!(matches!(
            selector.select(&pop, &mut rng),
            Err(TournamentSizeError {
                tournament_size: NonZeroUsize::MIN,
                population_size: 0
            })
        ));
    }

    #[test]
    fn tournament_size_larger_than_population() {
        let pop = [0];
        let mut rng = rand::rng();
        let selector = Tournament::of_size::<2>();
        assert!(matches!(
            selector.select(&pop, &mut rng),
            Err(TournamentSizeError {
               tournament_size,
               population_size: 1
            }) if tournament_size == NonZeroUsize::new(2).unwrap()));
    }

    #[test]
    fn tournament_size_1() {
        let mut rng = rng();
        let scores = &[5, 8, 9];
        let population = scores
            .iter()
            .enumerate()
            .map(|(genome, score)| EcIndividual::new(genome, score))
            .collect::<Vec<_>>();
        let selector = Tournament::new(NonZeroUsize::MIN);
        let winner = selector.select(&population, &mut rng).unwrap();
        assert!(scores.contains(winner.test_results));
    }

    #[proptest]
    fn tournament_size_2_pop_size_2(#[any] x: i32, #[any] y: i32) {
        let mut rng = rng();
        let scores = &[x, y];
        let population = scores
            .iter()
            .enumerate()
            .map(|(genome, score)| EcIndividual::new(genome, score))
            .collect::<Vec<_>>();
        let selector = Tournament::binary();
        let winner = selector.select(&population, &mut rng).unwrap();
        assert_eq!(winner.test_results, &x.max(y));
    }

    #[proptest]
    fn tournament_size_2_pop_size_3(
        #[any] x: i32,
        #[filter(|v| *v != #x)] y: i32,
        #[filter(|v| [#x, #y].contains(v).not())] z: i32,
    ) {
        let mut rng = rng();
        // We know from the filters that all the scores are unique, so the selected
        // score should always be better than the smallest score.
        let scores = &[x, y, z];
        let population = scores
            .iter()
            .enumerate()
            .map(|(genome, score)| EcIndividual::new(genome, score))
            .collect::<Vec<_>>();
        let selector = Tournament::binary();
        let selected = selector.select(&population, &mut rng).unwrap();
        assert!(scores.contains(selected.test_results));
        assert!(selected.test_results > scores.iter().min().unwrap());
    }
}
