use std::num::NonZeroUsize;

use rand::distr::slice::Choose;

/// A Distribution which knows how many choices are selcted from
pub trait ChoicesDistribution {
    /// The number of choices this distribution selects from
    fn num_choices(&self) -> NonZeroUsize;
}

static_assertions::assert_obj_safe!(ChoicesDistribution);

impl<T> ChoicesDistribution for Choose<'_, T> {
    fn num_choices(&self) -> NonZeroUsize {
        self.num_choices()
    }
}

impl<T> ChoicesDistribution for &T
where
    T: ChoicesDistribution + ?Sized,
{
    fn num_choices(&self) -> NonZeroUsize {
        (**self).num_choices()
    }
}

impl<T> ChoicesDistribution for &mut T
where
    T: ChoicesDistribution + ?Sized,
{
    fn num_choices(&self) -> NonZeroUsize {
        (**self).num_choices()
    }
}
