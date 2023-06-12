use ec_core::genome::Genome;

use super::LinearGenome;

#[derive(Debug, Clone)]
pub struct Vector<T> {
    pub genes: Vec<T>,
}

impl<T> Genome for Vector<T> {
    type Gene = T;
}

impl<T> LinearGenome for Vector<T> {
    fn size(&self) -> usize {
        self.genes.len()
    }

    fn gene_mut(&mut self, index: usize) -> Option<&mut Self::Gene> {
        self.genes.get_mut(index)
    }
}

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}
