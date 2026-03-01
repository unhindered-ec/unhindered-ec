/// Types with an associated weight
///
/// The weight is usually used to randomly select between different choices, as
/// in [`WeightedPair`](super::weighted_pair::WeightedPair) and then perform a
/// operation on the selected one, as in
/// [`Selector`](crate::operator::selector::Selector)
///
/// # Example
/// ```
/// # use ec_core::{
/// #     weighted::{Weighted, with_weight::WithWeight},
/// #     operator::selector::{best::Best, Selector}
/// # };
/// # use rand::rng;
/// #
/// let selector = Weighted::new(Best, 10);
///
/// assert_eq!(selector.weight(), 10);
/// #
/// # let selected = selector.select(&[10, 2, 1, 8, 0], &mut rng())?;
/// # assert_eq!(selected, &10);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub trait WithWeight {
    /// Get the weight of this item
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     weighted::{Weighted, with_weight::WithWeight},
    /// #     operator::selector::{best::Best, Selector}
    /// # };
    /// # use rand::rng;
    /// #
    /// let selector = Weighted::new(Best, 10);
    ///
    /// assert_eq!(selector.weight(), 10);
    /// #
    /// # let selected = selector.select(&[10, 2, 1, 8, 0], &mut rng())?;
    /// # assert_eq!(selected, &10);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn weight(&self) -> u32;
}

static_assertions::assert_obj_safe!(WithWeight);
