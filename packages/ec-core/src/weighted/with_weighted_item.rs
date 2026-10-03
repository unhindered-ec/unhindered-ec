use super::{
    Weighted, error::WeightSumOverflow, weighted_pair::WeightedPair, with_weight::WithWeight,
};

/// Extension trait on weighted items allowing easy construction of a set to
/// choose from.
///
/// The provided methods
/// - [`WithWeightedItem::with_item_and_weight`] and
/// - [`WithWeightedItem::with_weighted_item`]
///
/// allow easy chaining on other weighted items, creating chains of the style
/// `WeightedPair<WeightedPair<Weighted<T>, Weighted<U>>, Weighted<V>>`.
///
/// # Example
/// ```
/// # use ec_core::{
/// #     weighted::{
/// #         weighted_pair::WeightedPair,
/// #         with_weighted_item::WithWeightedItem,
/// #         Weighted
/// #     },
/// #     operator::selector::{
/// #         best::Best,
/// #         worst::Worst,
/// #         lexicase::Lexicase,
/// #         tournament::Tournament,
/// #     }
/// # };
/// #
/// # #[allow(clippy::type_complexity)]
/// let my_weighted: WeightedPair<
///     WeightedPair<WeightedPair<Weighted<Best>, Weighted<Worst>>, Weighted<Lexicase>>,
///     Weighted<Tournament>,
/// > = Weighted::new(Best, 1)
///     .with_item_and_weight(Worst, 1)
///     .with_weighted_item(Weighted::new(Lexicase::new(1), 2))
///     .with_item_and_weight(Tournament::of_size::<3>(), 5)?;
/// # let _ = my_weighted;
/// #
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub trait WithWeightedItem
where
    Self: Sized,
{
    type Output<Item>;

    /// Add a new item with the given weight to this weighted chain.
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     weighted::{
    /// #         weighted_pair::WeightedPair,
    /// #         with_weighted_item::WithWeightedItem,
    /// #         Weighted
    /// #     },
    /// #     operator::selector::{
    /// #         best::Best,
    /// #         worst::Worst,
    /// #         lexicase::Lexicase,
    /// #         tournament::Tournament,
    /// #     }
    /// # };
    /// #
    /// # #[allow(clippy::type_complexity)]
    /// let my_weighted: WeightedPair<
    ///     WeightedPair<WeightedPair<Weighted<Best>, Weighted<Worst>>, Weighted<Lexicase>>,
    ///     Weighted<Tournament>,
    /// > = Weighted::new(Best, 1)
    ///     .with_item_and_weight(Worst, 1)
    ///     .with_item_and_weight(Lexicase::new(1), 2)
    ///     .with_item_and_weight(Tournament::of_size::<3>(), 5)?;
    /// # let _ = my_weighted;
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// - [`WeightSumOverflow`] if trying to add this new item with the given
    ///   weight to the existing chain would overflow the total weight (i.e.
    ///   weight sum `> u32::MAX`)
    fn with_item_and_weight<S>(
        self,
        item: S,
        weight: u32,
    ) -> Result<Self::Output<Weighted<S>>, WeightSumOverflow> {
        self.with_weighted_item(Weighted::new(item, weight))
    }

    /// Add a new already weighted item to this weighted chain.
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     weighted::{
    /// #         weighted_pair::WeightedPair,
    /// #         with_weighted_item::WithWeightedItem,
    /// #         Weighted
    /// #     },
    /// #     operator::selector::{
    /// #         best::Best,
    /// #         worst::Worst,
    /// #         lexicase::Lexicase,
    /// #         tournament::Tournament,
    /// #     }
    /// # };
    /// #
    /// # #[allow(clippy::type_complexity)]
    /// let my_weighted: WeightedPair<
    ///     WeightedPair<WeightedPair<Weighted<Best>, Weighted<Worst>>, Weighted<Lexicase>>,
    ///     Weighted<Tournament>,
    /// > = Weighted::new(Best, 1)
    ///     .with_weighted_item(Weighted::new(Worst, 1))
    ///     .with_weighted_item(Weighted::new(Lexicase::new(1), 2))
    ///     .with_weighted_item(Weighted::new(Tournament::of_size::<3>(), 5))?;
    /// # let _ = my_weighted;
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// This for example also allows adding other already existing chains to
    /// this chain:
    ///
    /// ```
    /// # use ec_core::{
    /// #     weighted::{
    /// #         weighted_pair::WeightedPair,
    /// #         with_weighted_item::WithWeightedItem,
    /// #         Weighted
    /// #     },
    /// #     operator::selector::{
    /// #         best::Best,
    /// #         worst::Worst,
    /// #         lexicase::Lexicase,
    /// #         tournament::Tournament,
    /// #     }
    /// # };
    /// #
    /// # #[allow(clippy::type_complexity)]
    /// let my_weighted: WeightedPair<
    ///     WeightedPair<Weighted<Best>, Weighted<Worst>>,
    ///     WeightedPair<Weighted<Lexicase>, Weighted<Tournament>>,
    /// > = Weighted::new(Best, 1)
    ///     .with_item_and_weight(Worst, 1)
    ///     .with_weighted_item(
    ///         Weighted::new(Lexicase::new(1), 2)
    ///             .with_item_and_weight(Tournament::of_size::<3>(), 5)?,
    ///     )?;
    /// # let _ = my_weighted;
    /// #
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    /// - [`WeightSumOverflow`] if trying to add this new item to the existing
    ///   chain would overflow the total weight (i.e. weight sum `> u32::MAX`)
    fn with_weighted_item<Item>(
        self,
        weighted_item: Item,
    ) -> Result<Self::Output<Item>, WeightSumOverflow>
    where
        Item: WithWeight;
}

/// Implement [`WithWeightedItem`] for [`Weighted<T>`](Weighted) such that we
/// can create an initial [`WeightedPair`].
impl<T> WithWeightedItem for Weighted<T> {
    type Output<Item> = WeightedPair<Self, Item>;

    fn with_weighted_item<Item>(
        self,
        weighted_item: Item,
    ) -> Result<Self::Output<Item>, WeightSumOverflow>
    where
        Item: WithWeight,
    {
        WeightedPair::new(self, weighted_item)
    }
}

/// Implement [`WithWeightedItem`] for [`WeightedPair<T>`](Weighted) such that
/// we can extend the chain.
impl<A, B> WithWeightedItem for WeightedPair<A, B> {
    type Output<Item> = WeightedPair<Self, Item>;

    fn with_weighted_item<Item>(
        self,
        weighted_item: Item,
    ) -> Result<Self::Output<Item>, WeightSumOverflow>
    where
        Item: WithWeight,
    {
        WeightedPair::new(self, weighted_item)
    }
}

/// Implement [`WithWeightedItem`] for `Result<T, WeightSumOverflow>` such that
/// when chaining [`WithWeightedItem::with_weighted_item`] or
/// [`WithWeightedItem::with_item_and_weight`] one only needs to handle a
/// possible weight overflow error at the end instead of in between each step.
impl<T> WithWeightedItem for Result<T, WeightSumOverflow>
where
    T: WithWeightedItem,
{
    type Output<Item> = T::Output<Item>;

    fn with_weighted_item<Item>(
        self,
        weighted_item: Item,
    ) -> Result<Self::Output<Item>, WeightSumOverflow>
    where
        Item: WithWeight,
    {
        self?.with_weighted_item(weighted_item)
    }
}
