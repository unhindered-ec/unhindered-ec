use rand::rngs::ThreadRng;

pub mod uniform_xo;
pub mod two_point_xo;
pub mod mutate_with_rate;
pub mod mutate_with_one_over_length;

// TODO: This forces us to decide a fixed number of parent
//  genomes at compile time, so we won't be able to impl
//  this trait for a recombinator that takes an arbitrary
//  number of parent genomes. That might be a problem later,
//  but we'll deal with that if/when it comes up.
pub trait Recombinator<const NUM_PARENTS: usize, G>
{
    fn recombine(
        &self,
        genomes: [&G; NUM_PARENTS],
        rng: &mut ThreadRng,
    ) -> G;
}

// TODO: Should I create a macro that, for example, reduces
//   the boilerplate in implementations of `Recombinator`, e.g.,
//   the roughly 2/3 of the two Xo implementations that are identical?
//   NOTE: This became a lot less gross when we switched to an array
//   of genomes instead of passing in the population and selector, so
//   it may not be as big a deal.
