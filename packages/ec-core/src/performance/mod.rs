// TODO: We can probably use things in the `num` family of traits
//   (https://github.com/rust-num/num) to genericize `Score` and
//   `Error` so they're not tied to `i64`s anymore.
// TODO: I think that we want `Score` and `Error` to implement
//   some common trait so that we can mixed vectors of `Score`s
//   and `Error`s. Or maybe we already have that? Do (Partial)Ord,
//   (Partial)Eq, Ord, and Sum get us where we need to be? That's
//   lot to keep track of, so it might be useful to have a named
//   trait that has all those as super-traits so we have one name
//   that pulls them all together.

// TODO: Should there just be one struct (e.g., `Result<T>` with a `result: T`
// field)   and then `Error` and `Score` should be traits that these structs can
//   implement? I feel like that might avoid some duplication here.

// TODO: I'm not convinced that `Score` & `Error` need `Clone` and `Copy`
//   anymore. At a minimum we should try to push those requirements
//   closer to where they're actually needed.

pub mod error;
pub mod score;
pub mod test_results;

mod test_result;
