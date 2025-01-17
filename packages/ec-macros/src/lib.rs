#![doc(test(attr(warn(unused))))]

mod dyn_ref_impls;

/// Macro to generate impls for various reference types from a single impl
/// for a reference to a trait object
///
/// Annotate your trait impl for a reference type with this to automatically
/// generate impls for a predefined set of compatible smart pointer types as
/// well, as well as for the same type and the bounds `Send` and `Sync` or any
/// combination
///
/// This is usually used to generate normal trait impls for corresponding trait
/// objects of erased variants of the trait.
///
/// At the time of writing the following reference types may be auto-generated
/// for types requiring mutable access:
///
/// - [`&mut dyn T`](reference)
/// - [`Box<dyn T>`](std::boxed::Box)
/// - [`RefMut<dyn T>`](std::cell::RefMut)
///
/// for types only requiring immutable access in addition to the ones above the
/// following will also be generated:
///
/// - [`&dyn T`](reference)
/// - [`Arc<dyn T>`](std::sync::Arc)
/// - [`Rc<dyn T>`](std::rc::Rc)
/// - [`Ref<dyn T>`](std::cell::Ref)
///
/// as well as any of the above with any element of the powerset of
/// {[`Send`],[`Sync`]}, e.g. `&dyn T` will also generate `&dyn T + Send`, `&dyn
/// T + Sync` and `&dyn T + Send + Sync`

#[manyhow::manyhow(proc_macro_attribute)]
pub use dyn_ref_impls::dyn_ref_impls;

mod derive_composable;

/// Derive macro for the `Composable` trait.
///
/// Since the `Composable` trait is a marker trait (no actual contents)
/// this is just equivalent to
///
/// ```ignore
/// impl Composable for Foo {}
/// ```
/// up to the generics of the type the derive is on, i.e. it will add the
/// required generics and trait bounds as needed.
#[manyhow::manyhow(proc_macro_derive(Composable))]
pub use derive_composable::derive_composable;
