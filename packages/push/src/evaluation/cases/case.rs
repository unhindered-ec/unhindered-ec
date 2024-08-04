/// Simple pair of input and (expected) output used
/// for testing evolved systems.
///
/// See also [`Cases`](crate::evaluation::Cases).
///
/// # Examples
///
/// ```
/// # use push::evaluation::Case;
/// #
/// let case = Case::new("Hello", 5);
///
/// assert_eq!(case.input, "Hello");
/// assert_eq!(case.output, 5);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Case<Input, Output = Input> {
    pub input: Input,
    pub output: Output,
}

/// Create a `Case` from an input-output pair
///
/// # Examples
///
/// ```
/// # use push::evaluation::Case;
/// #
/// let case = Case::from(("this", 4));
///
/// assert_eq!(case.input, "this");
/// assert_eq!(case.output, 4);
/// ```
impl<Input, Output> From<(Input, Output)> for Case<Input, Output> {
    fn from((input, output): (Input, Output)) -> Self {
        Self { input, output }
    }
}

/// Convert a `Case` into an input-output pair.
///
/// # Examples
///
/// ```
/// # use push::evaluation::Case;
/// #
/// let case = Case::new(true, 17);
/// let (x, y): (bool, i32) = case.into();
///
/// assert_eq!(x, true);
/// assert_eq!(y, 17);
/// ```
impl<Input, Output> From<Case<Input, Output>> for (Input, Output) {
    fn from(case: Case<Input, Output>) -> (Input, Output) {
        (case.input, case.output)
    }
}

impl<Input, Output> Case<Input, Output> {
    /// Construct new `Case` from input and output.
    pub const fn new(input: Input, output: Output) -> Self {
        Self { input, output }
    }
}
