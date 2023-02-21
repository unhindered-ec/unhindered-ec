pub mod best;
pub mod lexicase;
pub mod random;
pub mod tournament;
pub mod weighted;

// TODO: We could explore named sub-traits or wrapper types later if
//   we feel they would be helpful.
// trait SelectionOp<P>
// where
//     Self: for<'pop> Operator<&'pop P, Output = &'pop P::Individual>,
//     P: Population,
// {
// }