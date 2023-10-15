use std::marker::PhantomData;

use crate::instruction::{PushInstruction, VariableName};

use super::{HasStack, PushInteger, PushState};

mod sealed {
    pub trait SealedMarker {}
}

/// The transition diagram for the builder is
///
/// ![PushState builder transition diagram][PushState_transition]
#[embed_doc_image(
    "PushState_transition",
    "../../images/PushState_builder_state_diagram.svg"
)]
pub trait StackState: sealed::SealedMarker {}
pub trait Dataless: StackState {}
pub trait SizeSet: StackState {}

pub struct SizeUnset;
impl sealed::SealedMarker for SizeUnset {}
impl StackState for SizeUnset {}
impl Dataless for SizeUnset {}

pub struct WithoutData;
impl sealed::SealedMarker for WithoutData {}
impl StackState for WithoutData {}
impl Dataless for WithoutData {}
impl SizeSet for WithoutData {}

pub struct WithData;
impl sealed::SealedMarker for WithData {}
impl StackState for WithData {}
impl SizeSet for WithData {}

builder! {
    Bool {
        field = bool,
        with_input_name = with_bool_input,
        with_values_name = with_bool_values,
        set_max_size =  with_bool_max_size,
        value_type = bool,
        instruction_name = push_bool
    },
    Int {
        field = int,
        with_input_name = with_int_input,
        with_values_name = with_int_values,
        set_max_size =  with_int_max_size,
        value_type = PushInteger,
        instruction_name = push_int
    }
}

macro_rules! replace {
    ($id:ident | $tokens:ty) => {
        $tokens
    };
}
// For properly scoping the macro
use replace;

macro_rules! builder {
    ($($id:ident {field=$field:ident, with_input_name=$with_input_name:ident, with_values_name=$with_values_name:ident, set_max_size=$set_max_size:ident, value_type=$value_type:ty, instruction_name=$instruction_name:ident }),+$(,)?) => {
        pub struct Builder<Exec: StackState, $($id: StackState),+> {
            partial_state: PushState,
            _p: PhantomData<(Exec, $($id),+)>,
        }

        impl Default for Builder<SizeUnset, $(replace!($id | SizeUnset)),+> {
            fn default() -> Self {
                Builder {
                    partial_state: Default::default(),
                    _p: PhantomData
                }
            }
        }

        impl<Exec: Dataless, $($id: Dataless),+> Builder<Exec, $($id),+> {
            /// Sets the maximum stack size for all the stacks in this state.
            ///
            /// # Arguments
            ///
            /// * `max_stack_size` - A `usize` specifying the maximum stack size
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use push::push_vm::HasStack;
            /// use push::push_vm::push_state::{ Stack, HasStack, PushState, Builder };
            /// let mut state = Builder::new(PushState::default())
            ///     .with_max_stack_size(100)
            ///     .build();
            /// let bool_stack: &Stack<bool> = state.stack();
            /// assert_eq!(bool_stack.max_stack_size, 100);
            /// ```
            #[must_use]
            pub fn with_max_stack_size(
                mut self,
                max_size: usize,
            ) -> Builder<WithSize, $(replace!($id | WithSize)),*> {
                self.partial_state
                    .exec
                    .reserve(max_size - self.partial_state.exec().len());

                $(
                    self.partial_state.$field.set_max_stack_size(max_size);
                )+

                Builder {
                    partial_state: self.partial_state,
                    _p: PhantomData,
                }
            }
        }

        impl<$($id: StackState),+> Builder<WithoutData, $($id),+> {
            /// Sets the program you wish to execute.
            /// Note that the program will be executed in ascending order.
            ///
            /// # Arguments
            /// - `program` - The program you wish to execute
            #[must_use]
            pub fn with_program<P>(self, program: P) -> Builder<WithSizeAndData, $($id),+>
            where
                P: IntoIterator<Item = PushInstruction>,
                P::IntoIter: DoubleEndedIterator,
            {
                Builder {
                    partial_state: PushState {
                        exec: program.into_iter().rev().collect(),
                        ..self.partial_state
                    },
                    _p: PhantomData,
                }
            }
        }

        impl<$($id: StackState),+> Builder<WithData, $($id),+> {
            /// Finalize the build process, returning the fully constructed `PushState`
            /// value. For this to successfully build, all the input variables has to
            /// have been given values. Thus every input variable provided
            /// in the `Inputs` used when constructing the `Builder` must have had a
            /// corresponding `with_X_input()` call that specified the value for that
            /// variable.
            ///
            /// # Panics
            /// Panics if one or more of the variables provided in the `Inputs` wasn't
            /// then given a value during the build process.
            /*
             * Note that the `with_x_input()` functions ensure that the instruction for
             * that input variable will be in the same position in `self.input_instructions`
             * as the name is in `self.inputs.input_names`. This allows us to zip together
             * those two lists and know that we'll be pairing up instructions with the appropriate
             * names.
             */
            #[must_use]
            pub fn build(self) -> PushState {
                self.partial_state
            }
        }

        impl<Exec: StackState, $($id: StackState),+> Builder<Exec,$($id),+ > {
            $(
                /// Adds a input instruction to the current current state's set
                /// of instructions. The name for the input must have been included
                /// in the `Inputs` provided when the `Builder` was initially constructed.
                /// Here you provide the name and the boolean value for that
                /// input variable. That will create a new `PushInstruction::push_[type]()`
                /// instruction that will push the specified value onto the stack
                /// when performed.
                ///
                /// # Panics
                /// This panics if the `input_name` provided isn't included in the set of
                /// names in the `Inputs` object used in the construction of the `Builder`.
                #[must_use]
                pub fn $with_input_name(mut self, input_name: &str, input_value: $value_type) -> Self {
                    self.partial_state.input_instructions.insert(
                        VariableName::from(input_name),
                        PushInstruction::$instruction_name(input_value),
                    );
                    self
                }
            )*
        }

        builder!{@values | $($id # $with_values_name # $value_type),+}
        builder!{@set_size | $($id # $set_max_size # $value_type),+}
    };
    (@values $($id_bef:ident),* |) => {};
    (@values $($id_bef:ident),* | $id:ident # $name:ident # $value_type:ty $(,$($id_aft:ident # $name_aft:ident # $value_type_aft:ty),+)? ) => {
        impl<Exec: StackState, $($id_bef: StackState,)* $id: SizeSet $(, $($id_aft: StackState),+)?> Builder<Exec, $($id_bef,)* $id $(, $($id_aft),+)?> {
            /// Adds the given sequence of values to the stack for the state you're building.
            ///
            /// The first value in `values` will be the new top of the
            /// stack. If the stack was initially empty, the last value
            /// in `values` will be the new bottom of the stack.
            ///
            /// # Arguments
            ///
            /// * `values` - A `Vec` holding the values to add to the stack
            ///
            /// # Examples
            ///
            /// ```ignore
            /// use push::push_vm::push_state::{ Stack, HasStack, PushState, Builder };
            /// let mut state = Builder::new(PushState::default())
            ///     .with_int_values(vec![5, 8, 9])
            ///     .build();
            /// let int_stack: &Stack<PushInteger> = state.stack();
            /// assert_eq!(int_stack.size(), 3);
            /// // Now the top of the stack is 5, followed by 8, then 9 at the bottom.
            /// assert_eq!(int_stack.top().unwrap(), &5);
            /// ```
            #[must_use]
            pub fn $name(mut self, values: Vec<$value_type>) -> Builder<Exec, $($id_bef,)* WithData $(, $($id_aft),+)?> {
                self.partial_state.stack_mut::<$value_type>().extend(values);

                Builder {
                    partial_state: self.partial_state,
                    _p: PhantomData,
                }
            }
        }
        builder!{@values $($id_bef,)* $id | $($($id_aft # $name_aft # $value_type_aft),*)?}
    };
    (@set_size $($id_bef:ident),* |) => {};
    (@set_size $($id_bef:ident),* | $id:ident # $name:ident # $value_type:ty $(,$($id_aft:ident # $name_aft:ident # $value_type_aft:ty),+)? ) => {
        impl<Exec: StackState, $($id_bef: StackState,)*$id: Dataless $(,$($id_aft: StackState),+)?> Builder<Exec, $($id_bef,)* $id $(, $($id_aft),+)?> {
            /// Sets the maximum stack size for the stack in this state.
            ///
            /// # Arguments
            ///
            /// * `max_stack_size` - A `usize` specifying the maximum stack size
            #[must_use]
            pub fn $name(mut self, max_stack_size: usize) -> Builder<Exec, $($id_bef,)* WithoutData $(, $($id_aft),+)?> {
                self.partial_state.stack_mut::<$value_type>().set_max_stack_size(max_stack_size);

                Builder {
                    partial_state: self.partial_state,
                    _p: PhantomData,
                }
            }
        }
        builder!{@set_size $($id_bef,)* $id | $($($id_aft # $name_aft # $value_type_aft),*)?}
    };
}

use builder;
