use miette::Diagnostic;

pub mod weighted_selector_pair;
pub mod with_weighted_selector;

#[derive(Debug, thiserror::Error, Diagnostic, PartialEq, Eq)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(u32, u32);
