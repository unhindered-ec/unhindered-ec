use std::num::Saturating;

// Justus suggested a macro like this to reduce the boilerplate below. Esitsu
// even thought it was a reasonable idea.

macro_rules! default_behavior {
    ($t: ty: $wrapper: ty) => {
        impl DefaultSummationBehavior for $t { type SummationWrapper = $wrapper; }
    };
    ($($t: ty: $wrapper: ty),*) => {
        $(default_behavior!($t: $wrapper);)*
    }
}

#[diagnostic::on_unimplemented(
    message = "No default summation behavior specified for type {Self}",
    label = "explicit summation type required here",
    note = "If you are trying to use TestResults<{Self}>, use TestResults<{Self}, MyTotalType> \
            instead,\nwhere MyTotalType is used for the total test result."
)]
/// Species the default type to use when summing a set of values.
pub trait DefaultSummationBehavior {
    type SummationWrapper;
}

pub trait SummationWrapper<T> {
    fn wrap(self) -> T;
}

impl<T> SummationWrapper<T> for T {
    fn wrap(self) -> T {
        self
    }
}

impl DefaultSummationBehavior for i8 {
    type SummationWrapper = i16;
}

impl SummationWrapper<i16> for i8 {
    fn wrap(self) -> i16 {
        self.into()
    }
}

impl DefaultSummationBehavior for i16 {
    type SummationWrapper = i32;
}

impl SummationWrapper<i32> for i16 {
    fn wrap(self) -> i32 {
        self.into()
    }
}

impl DefaultSummationBehavior for i32 {
    type SummationWrapper = i64;
}

impl SummationWrapper<i64> for i32 {
    fn wrap(self) -> i64 {
        self.into()
    }
}

impl DefaultSummationBehavior for i64 {
    type SummationWrapper = i128;
}

impl SummationWrapper<i128> for i64 {
    fn wrap(self) -> i128 {
        self.into()
    }
}

impl DefaultSummationBehavior for i128 {
    type SummationWrapper = i128;
}

impl<T> SummationWrapper<Saturating<T>> for T {
    fn wrap(self) -> Saturating<T> {
        Saturating(self)
    }
}

impl DefaultSummationBehavior for u8 {
    type SummationWrapper = Saturating<u8>;
}

impl DefaultSummationBehavior for u16 {
    type SummationWrapper = Saturating<u16>;
}

impl DefaultSummationBehavior for u32 {
    type SummationWrapper = Saturating<u32>;
}

impl DefaultSummationBehavior for u64 {
    type SummationWrapper = Saturating<u64>;
}

impl DefaultSummationBehavior for u128 {
    type SummationWrapper = Saturating<u128>;
}

impl DefaultSummationBehavior for f32 {
    type SummationWrapper = f32;
}

impl DefaultSummationBehavior for f64 {
    type SummationWrapper = f64;
}
