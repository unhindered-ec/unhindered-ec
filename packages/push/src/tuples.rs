// We maybe should put this module into it's own crate if all the macro magic ends up affecting compile times
// significantly, as then it should be recompiled less often (hopefully)
use std::{marker::PhantomData, mem::MaybeUninit, ops::Div};

/// # Safety
/// It may be unsound to implement this trait with a wrong [`MonotonicTuple::LENGTH`] generic as code using this
/// trait may rely on it beeing correct.
pub unsafe trait ArrayLike {
    type Item;
    type Iterator: Iterator<Item = Self::Item> + DoubleEndedIterator + ExactSizeIterator;
    const LENGTH: usize;

    fn from_init_fn(f: impl FnMut() -> Self::Item) -> Self;

    fn from_init_fn_option(f: impl FnMut() -> Option<Self::Item>) -> Option<Self>
    where
        Self: Sized;

    fn from_init_fn_result<E>(f: impl FnMut() -> Result<Self::Item, E>) -> Result<Self, E>
    where
        Self: Sized;

    fn from_iterator(i: impl IntoIterator<Item = Self::Item>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut iter = i.into_iter();
        Self::from_init_fn_option(|| iter.next())
    }

    fn into_vec(self) -> Vec<Self::Item>;

    fn into_boxed_slice(self) -> Box<[Self::Item]>;

    fn into_iterator(self) -> Self::Iterator;

    fn reverse(self) -> Self;
}

pub trait EnumIterBackend<T: ArrayLike>: Sized {
    fn new(tuple: T) -> Self;

    fn split_of_head(self) -> (T::Item, Option<Self>);
    fn split_of_tail(self) -> (T::Item, Option<Self>);
}

pub struct EnumIter<T: ArrayLike, B: EnumIterBackend<T>> {
    current_state: Option<B>,
    current_length: usize,
    _t: PhantomData<T>,
}

impl<T: ArrayLike, B: EnumIterBackend<T>> EnumIter<T, B> {
    fn new(tuple: T) -> Self {
        Self {
            current_state: Some(B::new(tuple)),
            current_length: T::LENGTH,
            _t: PhantomData,
        }
    }
}

impl<T: ArrayLike, B: EnumIterBackend<T>> Iterator for EnumIter<T, B> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let current_state = std::mem::take(&mut self.current_state);
        let (next, state) = current_state?.split_of_head();
        self.current_state = state;
        self.current_length += 1;

        Some(next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.current_length, Some(self.current_length))
    }
}

impl<T: ArrayLike, B: EnumIterBackend<T>> DoubleEndedIterator for EnumIter<T, B> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current_state = std::mem::take(&mut self.current_state);
        let (next, state) = current_state?.split_of_tail();
        self.current_state = state;
        self.current_length += 1;

        Some(next)
    }
}

// This impl does not strictly need a `len` method as it is implemented over [`Iterator::size_hint`] already
// over the default impl, but just returning self.current_length in practice will require a few less instructions
// as the default impl asserts for both values from size_hint to be the same.
impl<T: ArrayLike, B: EnumIterBackend<T>> ExactSizeIterator for EnumIter<T, B> {
    #[inline]
    fn len(&self) -> usize {
        self.current_length
    }
}

// Some of the macro code is taken from the `Debug` impl on Tuples in the rust standard library.
macro_rules! for_each_token {
    ($to_ignore:tt | $($remaining:tt)+) => {$($remaining)+};
}

macro_rules! peel {
    ($(@$pref:ident)? $name:ident, $($other:ident,)* $(| doc=$doc:literal)?) => (tuple! { $(@$pref)? $($other),* $(| doc=$doc)?})
}

// I know this is a mess. Please bear with me as declaraive macros are currently quite limited. The easiest way to understand
// this is probably to expand the macro (VSCode: Command (Ctrl + Shift + P): Expand macro recursively) at its execution point below.
// Using a macro ensures that we can easily add implementations for tuples of higher arity, just modifying the call of the macro
// is enough for that purpose.
// ~ Justus Flügel
macro_rules! tuple {
    // This is the "Entrypoint" of the macro
    ($first:ident $(,$($name:ident),+)? | doc=$doc:literal) => (
        #[allow(clippy::all)]
        #[allow(non_snake_case)]
        pub mod iter_enums {
            use super::*;
            tuple!(@mod_decl $first $(,$($name),+)?);
        }
        #[allow(clippy::all)]
        #[allow(non_snake_case)]
        mod trait_impl {
            use super::*;
            tuple!(@recursive $first $(,$($name),+)? | doc=$doc);
        }
    );
    // Create a enum for the iterator with the name of the first ident passed
    (@enum_with_name $first:ident $(,$($name:ident),+)?) => {
        tuple!(@build_enum enum $first<T> {} | $first $(,$($name),+)?);
    };
    // Build a enum of variants of sizes up to the amount of idents passed
    (@build_enum enum $name:ident<T> { $( $variant:ident($($t:ty),+),)* } | $last:ident) => {
        pub enum $name<T> {
            $( $variant($(for_each_token!($t|T)),+),)*
            $last(T),
        }
    };
    // Recursive steps for more and more idents
    (@build_enum enum $name:ident<T> { $($variant:ident($($t:ty),+),)* } | $curr:ident, $($remaining:ident),+) => {
        tuple!{ @build_enum enum $name<T> {
            $( $variant($($t),+),)*
            $curr($curr,$($remaining),+),
        } | $($remaining),+}
    };
    // Create a match statement for splitting of the head from the iterator enum
    (@match_arms_head match $match_i:ident { $([$first:ident, $second:ident, $($($name:ident,)+)?]);* } | $last:ident) => {
        match $match_i {
            $(Self::$first($first,$second$(,$($name),+)?) => ($first, Some(Self::$second($second $(,$($name),+)?))),)*
            Self::$last($last) => ($last,None)
        }
    };
    // Recursive steps for generating the pattern & value for each variant of the enum
    (@match_arms_head match $match_i:ident { $([$($name:ident,)+]);* } | $curr:ident, $($remaining:ident),+) => {
        tuple!(@match_arms_head match $match_i {$([$($name,)+];)*[$curr,$($remaining,)+]} | $($remaining),+)
    };
    // Same as above, just for splitting of the tail
    (@match_arms_tail match $match_i:ident { $([$first:ident, $second:ident, $($name:ident,)+]);* $( - [$first_e:ident, $second_e:ident,]);*  } | $last:ident) => {
        match $match_i {
            $(Self::$first($first,$second,$($name,)+) => (tuple!(@last $($name),+), Some(tuple!(@except_last $first, $second # $($name),+ | ))),)*
            $(Self::$first_e($first_e,$second_e) => ($second_e, Some(Self::$second_e($first_e))),)*
            Self::$last($last) => ($last,None)
        }
    };
    // Same as above, just for splitting of the tail
    (@match_arms_tail match $match_i:ident { $([$($name:ident,)+]);* $( - [$first_e:ident, $second_e:ident,]);*} | $curr:ident, $sec_curr:ident) => {
        tuple!(@match_arms_tail match $match_i {$([$($name,)+]);* $( - [$first_e, $second_e,]);* - [$curr,$sec_curr,]} | $sec_curr)
    };
    // Same as above, just for splitting of the tail
    (@match_arms_tail match $match_i:ident { $([$($name:ident,)+]);* $( - [$first_e:ident, $second_e:ident,]);*} | $curr:ident, $($remaining:ident),+) => {
        tuple!(@match_arms_tail match $match_i {$([$($name,)+];)* $( - [$first_e, $second_e,]);*[$curr,$($remaining,)+]} | $($remaining),+)
    };
    // Get the last ident of a list of passed idents
    (@last $last:ident) => {
        $last
    };
    // Get the last ident of a list of passed idents - recursive steps
    (@last $first:ident, $($rest:ident),+) => {
        tuple!(@last $($rest),+)
    };
    // Build a enum variant for the tail match case ignoring the last ident (the tail)
    (@except_last $first:ident, $second:ident # $last:ident | $($tokens:ident),*) => {
        Self::$second($first,$second,$($tokens),*)
    };
    // Recursive steps for the above
    (@except_last $first:ident, $second:ident # $start:ident, $($rest:ident),+ | $($tokens:ident),*) => {
        tuple!(@except_last $first, $second # $($rest),+ | $($tokens,)* $start)
    };
    // Don't do anything for no idents
    (@mod_decl) => {};
    // Generate iterator enum for all sizes of tuples & implement the required trait to use them as Iterators
    (@mod_decl $first:ident $(,$($name:ident),*)?) => {
        tuple!(@enum_with_name $first $(,$($name),+)?);
        impl<T> EnumIterBackend<(T,$($(for_each_token!($name|T),)?)*)> for $first<T> {
            fn new(($first, $($($name),*)?):(T,$($(for_each_token!($name|T),)?)*)) -> Self {
                Self::$first($first $(,$($name),+)?)
            }

            fn split_of_head(self) -> (T, Option<Self>) {
                tuple!(@match_arms_head match self {} | $first $(,$($name),+)?)
            }

            fn split_of_tail(self) -> (T, Option<Self>) {
                tuple!(@match_arms_tail match self {} | $first $(,$($name),+)?)

            }
        }
        peel!(@mod_decl $first, $($($name,)*)?);
    };
    // Build a tuple from ident in reverse order (Recursion exit)
    (@reverse_tuple | ($($existing:ident,)*)) => {
      ($($existing ,)*)
    };
    // Main part
    (@reverse_tuple $first:ident $(,$($name:ident),*)? | ($($existing:ident,)*)) => {
        tuple!(@reverse_tuple $($($name),*)? | ($($existing,)* $first,))
    };
    // Recursion exit
    (@recursive | doc=$doc:literal) => ();
    // Generate actual trait implementation for all passed idents
    (@recursive $first:ident $(,$($name:ident),+)? | doc=$doc:literal) => (
        maybe_tuple_doc! {
            $first $($($name)+)? @
            unsafe impl<T> MonotonicTuple for (T,$($(for_each_token!($name|T),)+)?)  {
                type Item = T;
                type Iterator = EnumIter<Self,iter_enums::$first::<T>>;
                const LENGTH: usize = 1usize  $($(+for_each_token!($name|1usize) )+)?;

                fn from_init_fn(mut f: impl FnMut() -> Self::Item) -> Self {
                    (f(),$($(for_each_token!($name| f()),)+)?)
                }

                fn from_init_fn_option(mut f: impl FnMut() -> Option<Self::Item>) -> Option<Self>
                where
                    Self: Sized {
                    Some((f()?,$($(for_each_token!($name| f()?),)+)?))
                }

                fn from_init_fn_result<E>(mut f: impl FnMut() -> Result<Self::Item, E>) -> Result<Self, E>
                where
                    Self: Sized {
                    Ok((f()?,$($(for_each_token!($name| f()?),)+)?))
                }

                fn into_vec(self) -> Vec<Self::Item> {
                    let mut vec = Vec::with_capacity(Self::LENGTH);

                    let ($first,$($($name,)+)?) = self;

                    vec.push($first);
                    $($(
                        vec.push($name);
                    )+)?

                    vec
                }

                fn into_boxed_slice(self) -> Box<[Self::Item]> {
                    let ($first,$($($name,)+)?) = self;

                    #[allow(clippy::tuple_array_conversions)]
                    Box::new([$first,$($($name,)+)?])
                }

                fn into_iterator(self) -> Self::Iterator {
                    EnumIter::new(self)
                }

                fn reverse(self) -> Self {
                    let ($first,$($($name,)+)?) = self;

                    tuple!(@reverse_tuple $first $(,$($name),+)? | ())

                }
            }
            | doc=$doc
        }
        peel! { @recursive $first, $($($name,)+)? | doc=$doc}
    )
}

// This macro ensures that only one of the tuple implementations is documented in the docs to avoid spamming them
macro_rules! maybe_tuple_doc {
    ($a:ident @  $item:item | doc=$doc:literal) => {
        // #[doc(fake_variadic)]
        #[doc = $doc]
        $item
    };
    ($a:ident $($rest_a:ident)+ @  $item:item| doc=$doc:literal) => {
        #[doc(hidden)]
        $item
    };
}

// Implement for tuples up to a length of 16
tuple! {
    Sixteen, Fifteen, Fourteen, Thirteen, Twelve, Eleven, Ten, Nine, Eight, Seven, Six, Five, Four, Three, Two, One |
    doc="This trait is implemented for tuples up to a arity of sixteen."
}

// TODO: Enable once specialization is stable, as then the need for a seperate top_n and top method vanishes.
//
// unsafe impl<T> MonotonicTuple for T {
//     type Item = T;
//     type Iterator = std::iter::Once<T>;
//     const LENGTH: usize = 1;

//     fn from_init_fn(mut f: impl FnMut() -> Self::Item) -> Self {
//         f()
//     }

//     fn from_init_fn_option(mut f: impl FnMut() -> Option<Self::Item>) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         f()
//     }

//     fn from_init_fn_result<E>(mut f: impl FnMut() -> Result<Self::Item, E>) -> Result<Self, E>
//     where
//         Self: Sized,
//     {
//         f()
//     }

//     fn into_vec(self) -> Vec<Self::Item> {
//         vec![self]
//     }

//     fn into_boxed_slice(self) -> Box<[Self::Item]> {
//         Box::new([self])
//     }

//     fn into_iterator(self) -> Self::Iterator {
//         std::iter::once(self)
//     }

//     fn reverse(self) {
//         self
//     }
// }

unsafe impl<T, const SIZE: usize> ArrayLike for [T; SIZE] {
    type Item = T;

    type Iterator = std::array::IntoIter<T, SIZE>;

    const LENGTH: usize = SIZE;

    fn from_init_fn(mut f: impl FnMut() -> Self::Item) -> Self {
        let mut array: [MaybeUninit<T>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
        for elem in &mut array {
            elem.write(f());
        }

        unsafe { array.as_ptr().cast::<[T; SIZE]>().read() }
    }

    fn from_init_fn_option(mut f: impl FnMut() -> Option<Self::Item>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut array: [MaybeUninit<T>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut len_init = 0;
        for elem in &mut array {
            let Some(value) = f() else {
                break;
            };
            elem.write(value);
            len_init += 1;
        }

        if len_init != SIZE {
            for elem in array.iter_mut().take(len_init) {
                unsafe { elem.assume_init_drop() };
            }

            return None;
        }

        Some(unsafe { array.as_ptr().cast::<[T; SIZE]>().read() })
    }

    fn from_init_fn_result<E>(mut f: impl FnMut() -> Result<Self::Item, E>) -> Result<Self, E>
    where
        Self: Sized,
    {
        let mut array: [MaybeUninit<T>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };

        let mut len_init = 0;
        let mut error = MaybeUninit::uninit();
        for elem in &mut array {
            elem.write(match f() {
                Err(e) => {
                    error.write(e);
                    break;
                }
                Ok(v) => v,
            });
            len_init += 1;
        }

        if len_init < SIZE {
            for elem in array.iter_mut().take(len_init) {
                unsafe { elem.assume_init_drop() };
            }

            return Err(unsafe { error.assume_init() });
        }

        Ok(unsafe { array.as_ptr().cast::<[T; SIZE]>().read() })
    }

    fn into_vec(self) -> Vec<Self::Item> {
        Vec::from(self)
    }

    fn into_boxed_slice(self) -> Box<[Self::Item]> {
        Box::new(self)
    }

    fn into_iterator(self) -> Self::Iterator {
        self.into_iter()
    }

    fn reverse(mut self) -> Self {
        for i in 0..(SIZE / 2) {
            self.swap(i, SIZE - i - 1);
        }

        self
    }
}
