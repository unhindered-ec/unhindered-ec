#![doc(test(attr(warn(unused))))]

// This is to allow using the macros from ec-macros inside this crate as
// ec-macros relies on the crate `ec_core` beeing present in the root module and
// this provides that (just an alias to self).
extern crate self as ec_core;

pub mod distributions;
pub mod generation;
pub mod genome;
pub mod individual;
pub mod operator;
pub mod population;
pub mod test_results;
pub mod weighted;
