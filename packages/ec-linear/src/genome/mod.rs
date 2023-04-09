pub mod bitstring;

pub trait Genome {
    type Gene;
}

pub trait LinearGenome: Genome
{
    fn size(&self) -> usize;

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene>;
}
