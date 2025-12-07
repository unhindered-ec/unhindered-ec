use ec_core::genome::Genome;

pub mod bitstring;

/// A linear genome
///
/// A linear genome is "flat" (like for example an array or vector) and allows
/// indexed acess to individual genes.
///
/// # Example
/// ```
/// # use ec_core::genome::Genome;
/// # use ec_linear::genome::Linear;
/// #
/// # #[allow(dead_code)]
/// struct MyGenome {
///     inner: Vec<i32>,
/// }
///
/// impl Genome for MyGenome {
///     type Gene = i32;
/// }
///
/// impl Linear for MyGenome {
///     fn size(&self) -> usize {
///         self.inner.len()
///     }
///
///     fn gene_mut(&mut self, index: usize) -> Option<&mut i32> {
///         self.inner.get_mut(index)
///     }
/// }
/// ```
pub trait Linear: Genome {
    /// Get the size of this linear genome
    ///
    /// # Example
    /// ```
    /// # use ec_core::genome::Genome;
    /// # use ec_linear::genome::Linear;
    /// #
    /// # struct MyGenome {
    /// #     inner: Vec<i32>,
    /// # }
    /// #
    /// # impl MyGenome {
    /// #     fn new<const N: usize>(from: [i32; N]) -> Self {
    /// #         Self { inner: from.into() }
    /// #     }
    /// # }
    /// #
    /// # impl Genome for MyGenome {
    /// #     type Gene = i32;
    /// # }
    /// #
    /// # impl Linear for MyGenome {
    /// #     fn size(&self) -> usize {
    /// #         self.inner.len()
    /// #     }
    /// #
    /// #     fn gene_mut(&mut self, index: usize) -> Option<&mut i32> {
    /// #         self.inner.get_mut(index)
    /// #     }
    /// # }
    /// let my_genome = MyGenome::new([0; 10]);
    /// assert_eq!(my_genome.size(), 10);
    /// ```
    fn size(&self) -> usize;

    /// Get a mutable reference to the gene at position `index`
    /// # Example
    /// ```
    /// # use ec_core::genome::Genome;
    /// # use ec_linear::genome::Linear;
    /// #
    /// # struct MyGenome {
    /// #     inner: Vec<i32>,
    /// # }
    /// #
    /// # impl MyGenome {
    /// #     fn new<const N: usize>(from: [i32; N]) -> Self {
    /// #         Self { inner: from.into() }
    /// #     }
    /// # }
    /// #
    /// # impl Genome for MyGenome {
    /// #     type Gene = i32;
    /// # }
    /// #
    /// # impl Linear for MyGenome {
    /// #     fn size(&self) -> usize {
    /// #         self.inner.len()
    /// #     }
    /// #
    /// #     fn gene_mut(&mut self, index: usize) -> Option<&mut i32> {
    /// #         self.inner.get_mut(index)
    /// #     }
    /// # }
    /// # fn foo() -> Option<()> {
    /// let mut my_genome = MyGenome::new([0; 10]);
    /// *my_genome.gene_mut(5)? = 1;
    /// # Some(())
    /// # }
    /// #
    /// # foo().unwrap();
    /// ```
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
