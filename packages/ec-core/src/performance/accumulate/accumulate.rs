use crate::performance::accumulate::{accumulated::Accumulated, strategy::AccumulateStrategy};

pub trait Accumulate: Iterator {
    fn accumulate<Acc>(&mut self) -> Result<Accumulated<Self::Item, Acc>, Acc::Error>
    where
        Acc: AccumulateStrategy<Self::Item>;
}

impl<I: Iterator + ?Sized> Accumulate for I {
    fn accumulate<Acc>(&mut self) -> Result<Accumulated<Self::Item, Acc>, Acc::Error>
    where
        Acc: AccumulateStrategy<Self::Item>,
    {
        match Acc::accumulate(self) {
            Ok(state) => Ok(Accumulated::from_state(state)),
            Err(e) => Err(e),
        }
    }
}
