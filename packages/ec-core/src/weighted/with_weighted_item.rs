use super::{
    Weighted, error::WeightSumOverflow, weighted_pair::WeightedPair, with_weight::WithWeight,
};

pub trait WithWeightedItem
where
    Self: Sized,
{
    type Output<Item>;

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
