use super::{
    weighted::Weighted, weighted_selector_pair::WeightedSelectorPair, WeightSumOverflow, WithWeight,
};

pub trait WithWeightedSelector
where
    Self: Sized,
{
    type OutputSelector<WS>;

    /// # Errors
    /// - [`WeightSumOverflow`] if trying to add this new selector with the
    ///   given weight to the existing chain would overflow the total weight
    ///   (i.e. weight sum `> u32::MAX`)
    fn with_selector_and_weight<S>(
        self,
        selector: S,
        weight: u32,
    ) -> Result<Self::OutputSelector<Weighted<S>>, WeightSumOverflow> {
        self.with_weighted_selector(Weighted::new(selector, weight))
    }

    /// # Errors
    /// - [`WeightSumOverflow`] if trying to add this new selector to the
    ///   existing chain would overflow the total weight (i.e. weight sum `>
    ///   u32::MAX`)
    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<Self::OutputSelector<WS>, WeightSumOverflow>
    where
        WS: WithWeight;
}

impl<T> WithWeightedSelector for Weighted<T> {
    type OutputSelector<WS> = WeightedSelectorPair<Self, WS>;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<Self::OutputSelector<WS>, WeightSumOverflow>
    where
        WS: WithWeight,
    {
        WeightedSelectorPair::new(self, weighted_selector)
    }
}

impl<A, B> WithWeightedSelector for WeightedSelectorPair<A, B> {
    type OutputSelector<WS> = WeightedSelectorPair<Self, WS>;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<Self::OutputSelector<WS>, WeightSumOverflow>
    where
        WS: WithWeight,
    {
        WeightedSelectorPair::new(self, weighted_selector)
    }
}

impl<T> WithWeightedSelector for Result<T, WeightSumOverflow>
where
    T: WithWeightedSelector,
{
    type OutputSelector<WS> = T::OutputSelector<WS>;

    fn with_weighted_selector<WS>(
        self,
        weighted_selector: WS,
    ) -> Result<Self::OutputSelector<WS>, WeightSumOverflow>
    where
        WS: WithWeight,
    {
        self?.with_weighted_selector(weighted_selector)
    }
}
