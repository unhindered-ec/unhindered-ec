use ec_core::genome::Genome;

pub mod bitstring;

pub trait Linear: Genome {
    fn size(&self) -> usize;

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene>;
}

static_assertions::assert_obj_safe!(Linear<Gene = ()>);

impl<T> Linear for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }
}
