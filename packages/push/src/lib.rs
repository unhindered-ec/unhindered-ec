pub mod error;
pub mod evaluation;
pub mod genome;
pub mod instruction;
pub mod list_into;
pub mod push_vm;

#[cfg(feature = "macros")]
pub use push_macros::*;

// This is to allow using the macros from push-macros inside this crate as
// push-macros relies on the crate `push` beeing present in the root module and
// this provides that (just an alias to self).
#[cfg(feature = "macros")]
extern crate self as push;

#[cfg(feature = "macros")]
pub use collectable;
