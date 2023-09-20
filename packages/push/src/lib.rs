#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
// I'm not sure how I feel about this, but my
// naming has _frequently_ violated this rule,
// and I'm not 100% sure I'd want to avoid those
// violations.
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod genome;
pub mod instruction;
pub mod push_vm;
pub(crate) mod tuples;
pub(crate) mod type_eq;
