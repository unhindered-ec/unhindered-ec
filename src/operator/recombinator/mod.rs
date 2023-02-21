pub mod mutate_with_rate;
pub mod mutate_with_one_over_length;
pub mod two_point_xo;
pub mod uniform_xo;

// TODO: Move mutations into `operator::mutator` and
//   crossovers into `operator::crossover`, and simplify
//   the type names to not repeat "mutate_with" and "xo"
//   everywhere.
