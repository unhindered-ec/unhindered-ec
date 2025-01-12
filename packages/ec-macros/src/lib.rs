#![doc(test(attr(warn(unused))))]

mod dyn_ref_impls;

#[manyhow::manyhow(proc_macro_attribute)]
pub use dyn_ref_impls::dyn_ref_impls;


