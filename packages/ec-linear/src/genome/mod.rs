use ec_core::genome::Genome;

pub mod bitstring;
pub mod bitstring_vec;

pub trait LinearGenome: Genome {
    fn size(&self) -> usize;

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene>;
}
