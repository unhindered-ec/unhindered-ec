use std::collections::{LinkedList, VecDeque};

use rand::prelude::Distribution;

/// [`Distribution`] adapter that turns a [`Distribution`] of elements `T` into
/// a [`Distribution`] of collections of elements (i.e. `Vec<T>`) of a given
/// length.
///
/// Sampling from this distribution samples from the element collection `length`
/// times.
///
/// # Provided conversion methods
///
/// For ease of use, the trait [`ConvertToCollectionDistribution`] provides
/// chainable constructors on any [`Distribution`] to turn that distribution
/// into a collection distribution.
///
/// Those constructors are:
/// - [`ConvertToCollectionDistribution::into_collection`], a owning
///   constructor, moving the child distribution
/// - [`ConvertToCollectionDistribution::to_collection`], a borrowing
///   constructor, borrowing the child distribution.
///
/// Usage:
///
/// ```
/// # use rand::distr::StandardUniform;
/// # use ec_core::distributions::collection::{Collection, ConvertToCollectionDistribution};
/// #
/// let collection_distribution: Collection<StandardUniform> = StandardUniform.into_collection(10);
/// # let _ = collection_distribution;
/// ```
///
/// ```
/// # use rand::distr::StandardUniform;
/// # use ec_core::distributions::collection::{Collection, ConvertToCollectionDistribution};
/// #
/// let collection_distribution: Collection<&StandardUniform> = StandardUniform.to_collection(10);
/// # let _ = collection_distribution;
/// ```
///
/// Also see [`ConvertToCollectionDistribution`]
///
/// # Example
/// ```
/// # use rand::{distr::{Distribution, StandardUniform}, rng};
/// # use ec_core::distributions::collection::Collection;
/// #
/// let singular_distribution = StandardUniform;
/// let collection_distribution = Collection::new(&singular_distribution, 10);
///
/// let mut rng = rng();
///
/// let single_element: i32 = singular_distribution.sample(&mut rng);
/// # let _ = single_element;
/// let collection: Vec<i32> = collection_distribution.sample(&mut rng);
/// assert_eq!(collection.len(), 10)
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Collection<C> {
    pub item_distribution: C,
    pub length: usize,
}

impl<C> From<(C, usize)> for Collection<C> {
    /// Turn a tuple of a [`Distribution`] and length into a [`Collection`]
    /// [`Distribution`]
    ///
    /// # Examples
    /// ```
    /// # use ec_core::distributions::collection::Collection;
    /// # use rand::distr::StandardUniform;
    /// #
    /// let my_distribution = Collection::from((StandardUniform, 10));
    /// # let _ = my_distribution;
    /// ```
    /// ```
    /// # use ec_core::distributions::collection::Collection;
    /// # use rand::distr::StandardUniform;
    /// #
    /// let my_distribution: Collection<_> = (StandardUniform, 10).into();
    /// # let _ = my_distribution;
    /// ```
    fn from((item_distribution, length): (C, usize)) -> Self {
        Self::new(item_distribution, length)
    }
}

impl<C> Collection<C> {
    /// Create a new [`Distribution`] of collections of size `length`,
    /// sampling individual elements from the given child [`Distribution`].
    ///
    /// # Example
    /// ```
    /// # use rand::distr::StandardUniform;
    /// # use ec_core::distributions::collection::Collection;
    /// #
    /// let collection_distribution = Collection::new(StandardUniform, 10);
    /// # let _ = collection_distribution;
    /// ```
    pub const fn new(item_distribution: C, length: usize) -> Self {
        Self {
            item_distribution,
            length,
        }
    }
}

/// Helper methods to apply the [`Collection`] [`Distribution`] adapter to other
/// [`Distribution`]'s.
///
/// This trait isn't meant to be implemented externally, rather
/// it is an extension trait that is blanket-implemented on all
/// types, although it is most useful for [`Distribution`]'s.
pub trait ConvertToCollectionDistribution {
    /// Apply the [`Collection`] [`Distribution`] adapter to self, moving self.
    ///
    /// This is semantically equivalent to calling `Collection::new(self,
    /// length)`.
    ///
    /// The resulting distribution generates collections of length `length`,
    /// sampling individual elements from `Self`.
    fn into_collection(self, length: usize) -> Collection<Self>
    where
        Self: Sized;

    /// Apply the [`Collection`] [`Distribution`] adapter to self, borrowing
    /// self.
    ///
    /// This is semantically equivalent to calling `Collection::new(&self,
    /// length)`.
    ///
    /// The resulting distribution generates collections of length `length`,
    /// sampling individual elements from `Self`.
    fn to_collection(&self, length: usize) -> Collection<&Self>;
}

impl<C> ConvertToCollectionDistribution for C
where
    C: ?Sized,
{
    fn into_collection(self, length: usize) -> Collection<Self>
    where
        Self: Sized,
    {
        Collection::new(self, length)
    }

    fn to_collection(&self, length: usize) -> Collection<&Self> {
        Collection::new(self, length)
    }
}

impl<T, C> Distribution<Vec<T>> for Collection<C>
where
    C: Distribution<T>,
{
    /// Sample the [`Vec`] collection from this [`Collection`] distribution, of
    /// length `length` as specified by the distribution.
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec<T> {
        (&self.item_distribution)
            .sample_iter(rng)
            .take(self.length)
            .collect()
    }
}

impl<T, C> Distribution<LinkedList<T>> for Collection<C>
where
    C: Distribution<T>,
{
    /// Sample the [`LinkedList`] collection from this [`Collection`]
    /// distribution, of length `length` as specified by the distribution.
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> LinkedList<T> {
        (&self.item_distribution)
            .sample_iter(rng)
            .take(self.length)
            .collect()
    }
}

impl<T, C> Distribution<VecDeque<T>> for Collection<C>
where
    C: Distribution<T>,
{
    /// Sample the [`VecDeque`] collection from this [`Collection`]
    /// distribution, of length `length` as specified by the distribution.
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> VecDeque<T> {
        (&self.item_distribution)
            .sample_iter(rng)
            .take(self.length)
            .collect()
    }
}
