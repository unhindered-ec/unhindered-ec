use std::ops::Range;

use crate::genome::LinearGenome;

// TODO: Does `Crossover` need to be visible outside
//   this module? If not, should `pub` be replaced/removed?
pub trait Crossover: LinearGenome {
    /// Swaps a gene.
    fn crossover_gene(&mut self, other: &mut Self, index: usize) -> anyhow::Result<()>;

    /// Swaps a segment.
    fn crossover_segment(&mut self, other: &mut Self, range: Range<usize>) -> anyhow::Result<()>
    {
        for index in range {
            self.crossover_gene(other, index)?;
        }
        Ok(())
    }
}
