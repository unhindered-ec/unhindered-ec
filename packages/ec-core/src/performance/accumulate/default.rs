#[cfg(feature = "ordered_float")]
use ordered_float::OrderedFloat;

use super::{keep_results::KeepResults, saturating_sum::SaturatingSum, sum::Sum, widen::Widen};
use crate::performance::{error_value::ErrorValue, score_value::ScoreValue};

#[diagnostic::on_unimplemented(
    message = "No default accumulation strategy specified for type {Self}",
    label = "explicit accumulation strategy required here",
    note = "If you are trying to use Accumulate<{Self}>, use Accumulate<{Self}, MyStrategy> \
            instead,\nwhere MyStrategy specifies the strategy of accumulation."
)]
pub trait DefaultAccumulateStrategy {
    type Strategy;
}

macro_rules! default_to {
    ($t: ty => $d: ty) => {
        impl DefaultAccumulateStrategy for $t {
            type Strategy = $d;
        }
    };
    ($($t: ty => $d: ty),+$(,)?) => {
        $(default_to!($t => $d);)+
    }
}

default_to! {
    u8 => KeepResults<SaturatingSum>,
    u16 => KeepResults<SaturatingSum>,
    u32 => KeepResults<SaturatingSum>,
    u64 => KeepResults<SaturatingSum>,
    u128 => KeepResults<SaturatingSum>,
    usize => KeepResults<SaturatingSum>,

    i8 => KeepResults<Widen<i16, Sum>>,
    i16 => KeepResults<Widen<i32, Sum>>,
    i32 => KeepResults<Widen<i64, Sum>>,
    i64 => KeepResults<Widen<i128, Sum>>,
    isize => KeepResults<Sum>,

    f32 => KeepResults<Sum>,
    f64 => KeepResults<Sum>,

    ScoreValue<u8> => KeepResults<SaturatingSum>,
    ScoreValue<u16> => KeepResults<SaturatingSum>,
    ScoreValue<u32> => KeepResults<SaturatingSum>,
    ScoreValue<u64> => KeepResults<SaturatingSum>,
    ScoreValue<u128> => KeepResults<SaturatingSum>,
    ScoreValue<usize> => KeepResults<SaturatingSum>,

    ScoreValue<i8> => KeepResults<Widen<ScoreValue<i16>, Sum>>,
    ScoreValue<i16> => KeepResults<Widen<ScoreValue<i32>, Sum>>,
    ScoreValue<i32> => KeepResults<Widen<ScoreValue<i64>, Sum>>,
    ScoreValue<i64> => KeepResults<Widen<ScoreValue<i128>, Sum>>,
    ScoreValue<isize> => KeepResults<Sum>,

    ScoreValue<f32> => KeepResults<Sum>,
    ScoreValue<f64> => KeepResults<Sum>,

    ErrorValue<u8> => KeepResults<SaturatingSum>,
    ErrorValue<u16> => KeepResults<SaturatingSum>,
    ErrorValue<u32> => KeepResults<SaturatingSum>,
    ErrorValue<u64> => KeepResults<SaturatingSum>,
    ErrorValue<u128> => KeepResults<SaturatingSum>,
    ErrorValue<usize> => KeepResults<SaturatingSum>,

    ErrorValue<i8> => KeepResults<Widen<ErrorValue<i16>, Sum>>,
    ErrorValue<i16> => KeepResults<Widen<ErrorValue<i32>, Sum>>,
    ErrorValue<i32> => KeepResults<Widen<ErrorValue<i64>, Sum>>,
    ErrorValue<i64> => KeepResults<Widen<ErrorValue<i128>, Sum>>,
    ErrorValue<isize> => KeepResults<Sum>,

    ErrorValue<f32> => KeepResults<Sum>,
    ErrorValue<f64> => KeepResults<Sum>,
}

#[cfg(feature = "ordered_float")]
default_to! {
    OrderedFloat<f32> => KeepResults<Sum>,
    OrderedFloat<f64> => KeepResults<Sum>,
    ScoreValue<OrderedFloat<f32>> => KeepResults<Sum>,
    ScoreValue<OrderedFloat<f64>> => KeepResults<Sum>,
    ErrorValue<OrderedFloat<f32>> => KeepResults<Sum>,
    ErrorValue<OrderedFloat<f64>> => KeepResults<Sum>,
}
