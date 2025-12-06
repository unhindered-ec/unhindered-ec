/// A collection of `Individual`'s
///
/// This trait is auto-implemented for all collections with both the `&T:
/// IntoIter<&Individual>` trait and the `T: IntoIter<Individual>` trait
/// implemeted.
///
/// # Example
/// ```
/// # use ec_core::{individual::ec::EcIndividual, population::Population};
/// #
/// let my_population = vec![
///     EcIndividual::new(0b11001110, 5),
///     EcIndividual::new(0b11001111, 6),
///     EcIndividual::new(0b10000011, 3),
/// ];
///
/// fn check_not_empty(population: &impl Population) -> bool {
///     population.is_empty()
/// }
///
/// assert_eq!(check_not_empty(&my_population), my_population.is_empty())
/// ```
pub trait Population {
    /// The type of the individual of this population
    type Individual;

    /// Returns `true` if this Popluation doesn't contain any individuals
    ///
    /// # Example
    /// ```
    /// # use ec_core::population::Population;
    /// #
    /// let my_population = vec![1, 2, 3];
    ///
    /// assert!(!Population::is_empty(&my_population))
    /// ```
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Returns the number of individuals in this [`Population`]
    /// # Example
    /// ```
    /// # use ec_core::population::Population;
    /// #
    /// let my_population = vec![1, 2, 3];
    ///
    /// assert_eq!(Population::size(&my_population), 3)
    /// ```
    fn size(&self) -> usize;
}

static_assertions::assert_obj_safe!(Population<Individual = ()>);

impl<T, I> Population for T
where
    // An alternative would be `T: Deref<[T]>` but this also supports
    // hashsets, etc.
    //
    // This might look a bit confusing but essentially what this bound
    // combination does is require both a `iter(&self) -> impl Iterator<Item
    // = &T>` as well as a `into_iter(self) -> impl Iterator<Item = T>`
    // method on the collection implementing population,
    // which enables us to get the size of the collection without consuming
    // it (through the `.iter()` method, since it takes `&self`) as well
    // as turning this population into an iterator.
    //
    // Just requiring `.into_iter()` wouldn't work, since we don't want to
    // consume the population to implement the `.size()` method.
    for<'a> &'a T: IntoIterator<Item = &'a I, IntoIter: ExactSizeIterator>,
    T: IntoIterator<Item = I>,
{
    type Individual = I;

    fn size(&self) -> usize {
        self.into_iter().len()
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Range;

    use rand::{Rng, prelude::Distribution, rng};

    use crate::{
        distributions::collection::ConvertToCollectionDistribution, population::Population,
    };

    struct RandValue {
        val: i32,
    }

    impl Distribution<RandValue> for Range<i32> {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RandValue {
            RandValue {
                val: rng.random_range(self.clone()),
            }
        }
    }

    #[test]
    fn generator_works() {
        let mut rng = rng();
        let population_size = 10;
        let range = -10..25;
        let vec_pop: Vec<_> = range.to_collection(population_size).sample(&mut rng);

        assert_eq!(population_size, Population::size(&vec_pop));
        for i in vec_pop {
            assert!(range.contains(&i.val));
        }
    }
}
