pub mod has_stack;
pub mod with_stack;

pub mod discard;
pub mod extend;
pub mod get;
pub mod pop;
pub mod push;
pub mod size;

pub trait TypedStack {
    type Item;
}
