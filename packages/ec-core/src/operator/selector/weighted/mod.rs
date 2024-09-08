pub mod weighted;
pub mod weighted_selector_pair;
pub mod with_weighted_selector;

#[derive(Debug, thiserror::Error)]
#[error("Overflow while trying to calculate the sum of the weights {0} and {1}.")]
pub struct WeightSumOverflow(u32, u32);

pub trait WithWeight {
    fn weight(&self) -> u32;
}

#[cfg(test)]
#[rustversion::attr(before(1.81), allow(clippy::unwrap_used))]
#[rustversion::attr(
    since(1.81),
    expect(
        clippy::unwrap_used,
        reason = "Panicking is the best way to deal with errors in unit tests"
    )
)]
mod tests {
    use itertools::Itertools;
    use test_strategy::proptest;

    use super::{weighted::Weighted, with_weighted_selector::WithWeightedSelector};
    use crate::operator::selector::{
        best::Best, random::Random, tournament::Tournament, worst::Worst, Selector,
    };

    #[proptest]
    fn best_or_worst(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::thread_rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Best, 1)
            .with_selector_and_weight(Worst, 1)
            .unwrap();
        let selection = weighted.select(&pop, &mut rng).unwrap();
        let extremes: [&i32; 2] = pop.iter().minmax().into_option().unwrap().into();
        assert!(extremes.contains(&selection));
    }

    #[proptest]
    fn several_selectors(#[map(|v: [i32;10]| v.into())] pop: Vec<i32>) {
        let mut rng = rand::thread_rng();
        // We'll make a selector that has a 50/50 chance of choosing the highest
        // or lowest value.
        let weighted = Weighted::new(Best, 1)
            .with_selector_and_weight(Worst, 1)
            .with_selector_and_weight(Random, 0)
            .with_selector_and_weight(Tournament::of_size::<3>(), 2)
            .unwrap();
        let selection = weighted.select(&pop, &mut rng).unwrap();
        assert!(pop.contains(selection));
    }
}
