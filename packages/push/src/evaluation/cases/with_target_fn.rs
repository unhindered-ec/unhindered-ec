use super::super::Cases;

/// Create a set of cases from a set of inputs (`self`) and a target
/// function that maps inputs to expected outputs.
///
/// # Examples
///
/// ```
/// # use push::evaluation::{Case, Cases, WithTargetFn};
/// #
/// let inputs = ["this", "and", "those"];
/// let cases = inputs.with_target_fn(|s| s.len());
///
/// assert!(cases.inputs().eq(&inputs));
/// assert!(cases.outputs().eq(&[4, 3, 5]));
/// ```
pub trait WithTargetFn<Input> {
    fn with_target_fn<Output, F>(self, target_fn: F) -> Cases<Input, Output>
    where
        F: Fn(&Input) -> Output;
}

impl<T, Input> WithTargetFn<Input> for T
where
    T: IntoIterator<Item = Input>,
{
    /// Create a set of cases from a set of inputs (`self`) and a target
    /// function that maps inputs to expected outputs.
    ///
    /// # Constraints
    ///
    /// `T` (the type of `self`) must implement `IntoIterator<Item =
    /// Input>`; this provides the mechanism for iterating over the
    /// inputs so we can create the associated outputs and thus the
    /// individual `Cases`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Cases, WithTargetFn};
    /// let inputs = ["this", "and", "those"];
    /// let cases = inputs.with_target_fn(|s| s.len());
    ///
    /// assert!(cases.inputs().eq(&inputs));
    /// assert!(cases.outputs().eq(&[4, 3, 5]));
    /// ```
    fn with_target_fn<Output, F>(self, target_fn: F) -> Cases<Input, Output>
    where
        F: Fn(&Input) -> Output,
    {
        Cases::from_inputs(self, target_fn)
    }
}

#[cfg(test)]
mod tests {
    use test_strategy::proptest;

    use crate::evaluation::WithTargetFn;

    #[proptest]
    fn with_identity(#[any] inputs: Vec<i32>) {
        let cases = inputs.into_iter().with_target_fn(i32::to_owned);
        assert_eq!(
            cases.inputs().collect::<Vec<_>>(),
            cases.outputs().collect::<Vec<_>>()
        );
    }

    #[proptest]
    fn with_string_length(#[any] inputs: Vec<String>) {
        let cases = inputs.into_iter().with_target_fn(String::len);
        for c in cases {
            assert_eq!(c.input.len(), c.output);
        }
    }
}
