
pub mod weighted;
pub mod weighted_selector_pair;
pub mod with_weighted_selector;

#[derive(Debug, thiserror::Error)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(u32, u32);

pub trait WithWeight {
    fn weight(&self) -> u32;
}