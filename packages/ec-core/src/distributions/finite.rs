use std::num::NonZeroUsize;

use rand::distr::slice::Choose;

pub trait Finite {
    fn sample_space_size(&self) -> NonZeroUsize;
}

static_assertions::assert_obj_safe!(Finite);

impl<T> Finite for Choose<'_, T> {
    fn sample_space_size(&self) -> NonZeroUsize {
        self.num_choices()
    }
}

impl<T> Finite for &T
where
    T: Finite + ?Sized,
{
    fn sample_space_size(&self) -> NonZeroUsize {
        (**self).sample_space_size()
    }
}

impl<T> Finite for &mut T
where
    T: Finite + ?Sized,
{
    fn sample_space_size(&self) -> NonZeroUsize {
        (**self).sample_space_size()
    }
}
