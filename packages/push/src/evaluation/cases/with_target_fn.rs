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
    use crate::evaluation::WithTargetFn;

    #[test]
    fn empty_inputs() {
        let inputs: [i32; 0] = [];
        let cases = inputs.into_iter().with_target_fn(|x| x * 2);

        assert!(cases.is_empty());
    }

    #[test]
    fn string_length() {
        let inputs = ["Hello", "to", "all", "the", "people"];
        let cases = inputs.into_iter().with_target_fn(|s| s.len());

        assert!(cases.inputs().eq(&inputs));
        assert!(cases.outputs().eq(&[5, 2, 3, 3, 6]));
    }
}
