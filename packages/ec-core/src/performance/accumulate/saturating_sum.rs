use std::convert::Infallible;

use super::{strategy::AccumulateStrategy, total::TotalResult};
use crate::performance::{error_value::ErrorValue, score_value::ScoreValue};

#[derive(Debug, Clone, Copy)]
pub struct SaturatingSum;

macro_rules! impl_accumulate_strategy {
    (signed $t:ty) => {
        impl AccumulateStrategy<$t> for SaturatingSum {
            type Error = Infallible;
            type State = $t;

            fn initialize() -> Self::State {
                Self::State::default()
            }

            fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
            where
                I: Iterator<Item = $t>,
            {
                for item in iter {
                    *state = state.saturating_add(item);
                }

                Ok(())
            }
        }

        impl TotalResult<$t> for SaturatingSum {
            type TotalRef<'a> = $t;
            type Total = $t;

            fn total(state: &Self::State) -> Self::TotalRef<'_> {
                *state
            }

            fn into_total(state: Self::State) -> Self::Total {
                state
            }
        }
    };
    (signedw $t:ty) => {
        impl AccumulateStrategy<$t> for SaturatingSum {
            type Error = Infallible;
            type State = $t;

            fn initialize() -> Self::State {
                Self::State::default()
            }

            fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
            where
                I: Iterator<Item = $t>,
            {
                for item in iter {
                    state.0 = state.0.saturating_add(item.0);
                }

                Ok(())
            }
        }

        impl TotalResult<$t> for SaturatingSum {
            type TotalRef<'a> = $t;
            type Total = $t;

            fn total(state: &Self::State) -> Self::TotalRef<'_> {
                *state
            }

            fn into_total(state: Self::State) -> Self::Total {
                state
            }
        }
    };
    (unsigned $t:ty) => {
        impl AccumulateStrategy<$t> for SaturatingSum {
            type Error = Infallible;
            type State = $t;

            fn initialize() -> Self::State {
                Self::State::default()
            }

            fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
            where
                I: Iterator<Item = $t>,
            {
                use std::num::Saturating;
                let Saturating(sum) = iter.map(|x| Saturating(x)).sum();

                *state = state.saturating_add(sum);

                Ok(())
            }
        }

        impl TotalResult<$t> for SaturatingSum {
            type TotalRef<'a> = $t;
            type Total = $t;

            fn total(state: &Self::State) -> Self::TotalRef<'_> {
                *state
            }

            fn into_total(state: Self::State) -> Self::Total {
                state
            }
        }
    };
    (unsigned2 $t:ty) => {
        impl AccumulateStrategy<$t> for SaturatingSum {
            type Error = Infallible;
            type State = $t;

            fn initialize() -> Self::State {
                Self::State::default()
            }

            fn accululate_into<I>(state: &mut Self::State, iter: I) -> Result<(), Self::Error>
            where
                I: Iterator<Item = $t>,
            {
                use std::num::Saturating;
                let Saturating(sum) = iter.map(|x| Saturating(x.0)).sum();

                state.0 = state.0.saturating_add(sum);

                Ok(())
            }
        }

        impl TotalResult<$t> for SaturatingSum {
            type TotalRef<'a> = $t;
            type Total = $t;

            fn total(state: &Self::State) -> Self::TotalRef<'_> {
                *state
            }

            fn into_total(state: Self::State) -> Self::Total {
                state
            }
        }
    };
    ($s: tt $($t: ty),+$(,)?) => {
        $(impl_accumulate_strategy!($s $t);)+
    };
}

impl_accumulate_strategy!(signed i8, i16, i32, i64, i128, isize);
impl_accumulate_strategy!(unsigned u8, u16, u32, u64, u128, usize);
impl_accumulate_strategy!(signedw ScoreValue<i8>, ScoreValue<i16>, ScoreValue<i32>, ScoreValue<i64>, ScoreValue<i128>, ScoreValue<isize>);
impl_accumulate_strategy!(unsigned2 ScoreValue<u8>, ScoreValue<u16>, ScoreValue<u32>, ScoreValue<u64>, ScoreValue<u128>, ScoreValue<usize>);
impl_accumulate_strategy!(signedw ErrorValue<i8>, ErrorValue<i16>, ErrorValue<i32>, ErrorValue<i64>, ErrorValue<i128>, ErrorValue<isize>);
impl_accumulate_strategy!(unsigned2 ErrorValue<u8>, ErrorValue<u16>, ErrorValue<u32>, ErrorValue<u64>, ErrorValue<u128>, ErrorValue<usize>);
