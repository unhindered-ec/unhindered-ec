// TODO: We can probably use things in the `num` family of traits
//   (https://github.com/rust-num/num) to genericize `Score` and
//   `Error` so they're not tied to `i64`s anymore.

pub mod error_value;
pub mod score_value;

pub mod summation_behavior;

// I don't think we ever _use_ `TestResult` (singular), so maybe it goes away.
pub mod test_result;

// TODO: Should there just be one struct (e.g., `Result<T>` with a `result: T`
// field)   and then `Error` and `Score` should be traits that these structs can
//   implement? I feel like that might avoid some duplication here.

pub mod test_results;
