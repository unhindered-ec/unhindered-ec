use push_state::printing::generate_builder::generate_builder;
use quote::quote;
use syn::{spanned::Spanned, DeriveInput};

use crate::push_state::{parsing::parse_fields, printing::derive_has_stack::derive_has_stack};

mod doctest_tokenstream;
mod push_state;

/// A macro for generating the corresponding code to create a new push state
/// type
///
/// This macro supports several feature flags which may or may not be enabled by
/// default. You can set feature flags like so:
/// ```text
/// #[push_state(!default_enabled_feature, default_disabled_feature)]
/// ```
/// This would set the default_enabled_feature to be disabled
/// and the default_disabled_feature to enabled
///
/// # Features
/// ## HasStack (enabled by default)
/// This derives the HasStack trait for all stacks in the state.
/// You need to indicate which fields are stacks
/// using the `#[stack]` attribute on the corresponding field.
///
/// ## Builder (disabled by default)
/// This creates a builder for this state.
/// You need to indicate which fields are stacks using the `#[stack]`
/// attribute, which field is the exec stack using the `#[stack(exec)]`
/// attribute.
///
/// You may change the name of the builder functions using `#[stack(builder_name
/// = <name>)]` and the Instruction used for input values with
/// `#[stack(instruction_name = <path>)]` and you can add sample values to stack
/// with `#[stack(sample_values = [val1, val2, val3])]`. Note that for this to
/// work, every foreign type used in the stack type as well as to define the
/// sample values (eg. `sample_values = [OrderedFloat(0.3)]`) need to be `pub
/// use`'d inside the module where state is defined. This is currently required
/// as doctests are complied as their own seperate module. Alternatively, you
/// can use the `ignore_doctests` flag to annotate every code example of the
/// stack with the `ignore` attribute.
///
/// # Example
/// ```ignore
/// #[push_state::push_state(builder)]
/// struct SomeState {
///     #[stack(exec)]
///     exec: Stack<PushInstruction>,
///     #[stack(builder_name = number, instruction_name = MyInput::int_input)]
///     int: Stack<MyInteger>,
///
///     #[input_instructions]
///     input_instructions: HashMap<VariableName, MyInput>
/// }
///
/// fn main() -> Result<(), StackError> {
///     let stack = SomeState::builder()
///         .with_max_stack_size(1000)
///         .with_program(Default::default())?
///         .with_max_number_stack_size(100)
///         .with_int_values([100, 10000, 10])?
///         .build();
///
///     Ok(())
/// }
/// ```
#[manyhow::manyhow(proc_macro_attribute)]
pub fn push_state(
    attrs: proc_macro2::TokenStream,
    tokens: proc_macro2::TokenStream,
) -> manyhow::Result {
    let macro_span = attrs.span();
    let macro_flags = syn::parse2(attrs)?;

    let mut struct_defn: DeriveInput = syn::parse2(tokens)?;

    let DeriveInput {
        data:
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { named: fields, .. }),
                ..
            }),
        ident: struct_ident,
        generics: struct_generics,
        vis: struct_visibility,
        ..
    } = &mut struct_defn
    else {
        return Err(syn::Error::new(
            macro_span,
            "This macro only supports structs with named fields.",
        )
        .into());
    };

    let (stacks, exec_stack, input_instructions) = parse_fields(fields, macro_span, &macro_flags)?;

    let has_stack_derives = macro_flags
        .has_stack
        .then(|| derive_has_stack(struct_ident, &stacks, &exec_stack));

    let builder = macro_flags
        .builder
        .then(|| {
            generate_builder(
                macro_span,
                struct_ident,
                struct_visibility,
                struct_generics,
                &stacks,
                &exec_stack,
                input_instructions,
            )
        })
        .transpose()?;

    Ok(quote! {
        #struct_defn
        #has_stack_derives
        #builder
    })
}
