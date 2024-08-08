//! `Cases` is a a collection of input-output pairs
//! used to test systems being evolved.

mod case;
mod iter;
mod with_target_fn;

pub use case::Case;
pub use with_target_fn::WithTargetFn;

/// Collection of [`Case`] input-output pairs, used for testing
/// evolved systems.
///
/// See also [`WithTargetFn`], which is often useful for constructing
/// `Cases` from a set of inputs and a target function.
///
/// # Examples
///
/// ```
/// # use push::evaluation::{Case, Cases, WithTargetFn};
/// #
/// let inputs = ["this", "and", "those"];
/// // Pair strings (inputs) with their lengths (outputs).
/// let cases = inputs.with_target_fn(|s| s.len());
///
/// assert!(cases.inputs().eq(&inputs));
/// assert!(cases.outputs().eq(&[4, 3, 5]));
/// ```
#[derive(Debug)]
pub struct Cases<Input, Output = Input> {
    cases: Vec<Case<Input, Output>>,
}

impl<Input, Output> Cases<Input, Output> {
    /// Create an empty set of cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let cases = Cases::<String, usize>::new();
    ///
    /// assert!(cases.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self { cases: Vec::new() }
    }

    /// Create a set of cases from a set of inputs and
    /// a function mapping inputs to outputs.
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let inputs = ["this", "and", "those"];
    /// let cases = Cases::from_inputs(inputs, |s| s.len());
    ///
    /// assert!(cases.inputs().eq(&inputs));
    /// assert!(cases.outputs().eq(&[4, 3, 5]));
    /// ```
    pub fn from_inputs(
        inputs: impl IntoIterator<Item = Input>,
        target_function: impl Fn(&Input) -> Output,
    ) -> Self {
        inputs
            .into_iter()
            .map(|input| {
                let output = target_function(&input);
                Case::new(input, output)
            })
            .collect()
    }

    /// Add a case to a set of cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Case, Cases};
    /// #
    /// let mut cases = Cases::new();
    /// let case = Case::new("Hello", 5);
    /// cases.add_case(case);
    ///
    /// assert!(cases.iter().eq(&[case]));
    /// ```
    pub fn add_case(&mut self, case: impl Into<Case<Input, Output>>) {
        self.cases.push(case.into());
    }

    /// Add a case to a set of cases, returning the set of cases for
    /// possible chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Case, Cases};
    /// #
    /// let first_case = Case::new("Hello", 5);
    /// let second_case = Case::new("People", 6);
    /// let cases = Cases::new().with_case(first_case).with_case(second_case);
    ///
    /// assert!(cases.iter().eq(&[first_case, second_case]));
    /// ```
    #[must_use]
    pub fn with_case(mut self, case: impl Into<Case<Input, Output>>) -> Self {
        self.add_case(case);
        self
    }

    /// Determine if this set of `Cases` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let cases = Cases::<String, usize>::new();
    ///
    /// assert!(cases.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cases.is_empty()
    }

    /// The number of cases in this collection.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let inputs = ["this", "and", "those"];
    /// let cases = Cases::from_inputs(inputs, |s| s.len());
    ///
    /// assert_eq!(cases.len(), 3);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.cases.len()
    }
}

/// Implement various iterators over `Cases`, inputs and outputs.
impl<Input, Output> Cases<Input, Output> {
    /// Creates an iterator over the set of cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Case, Cases, WithTargetFn};
    /// #
    /// let inputs = ["Hello", "People"];
    /// let cases = inputs.with_target_fn(|s| s.len());
    ///
    /// let mut iter = cases.iter();
    ///
    /// assert_eq!(iter.next(), Some(&Case::new("Hello", 5)));
    /// assert_eq!(iter.next(), Some(&Case::new("People", 6)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &Case<Input, Output>> {
        // std::slice::Iter<Case<Input, Output>> {
        self.cases.iter()
    }

    /// Creates a iterator over mutable instances of the cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Case, Cases};
    /// #
    /// let first_case = Case::new("Hello", 5);
    /// let mut second_case = Case::new("People", 6);
    /// let mut cases = Cases::new().with_case(first_case).with_case(second_case);
    ///
    /// cases.iter_mut().for_each(|c| c.output *= 2);
    ///
    /// let mut iter = cases.iter();
    ///
    /// assert_eq!(iter.next(), Some(&Case::new("Hello", 10)));
    /// assert_eq!(iter.next(), Some(&Case::new("People", 12)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Case<Input, Output>> {
        self.cases.iter_mut()
    }

    /// Creates an iterator over the inputs for this set of cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let inputs = ["this", "and", "those"];
    /// let cases = Cases::from_inputs(inputs, |s| s.len());
    ///
    /// assert!(cases.inputs().eq(&inputs));
    /// ```
    pub fn inputs(&self) -> impl Iterator<Item = &Input> {
        self.cases.iter().map(|c| &c.input)
    }

    /// Creates an iterator over mutable references to the inputs for this set
    /// of cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let inputs: Vec<String> = ["this", "and", "those"]
    ///     .iter()
    ///     .map(ToString::to_string)
    ///     .collect();
    /// let mut cases = Cases::from_inputs(inputs, String::len);
    ///
    /// for input in cases.inputs_mut() {
    ///     *input = input.to_uppercase();
    /// }
    ///
    /// assert!(cases.inputs().eq(&["THIS", "AND", "THOSE"]));
    /// assert!(cases.outputs().eq(&[4, 3, 5]));
    /// ```
    pub fn inputs_mut(&mut self) -> impl Iterator<Item = &mut Input> {
        self.cases.iter_mut().map(|c| &mut c.input)
    }

    /// Convert this set of cases into an iterator over the inputs. This
    /// takes ownership of the cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let inputs = ["this", "and", "those"];
    /// let cases = Cases::from_inputs(inputs, |s| s.len());
    ///
    /// assert!(cases.into_inputs().eq(["this", "and", "those"]));
    /// ```
    pub fn into_inputs(self) -> impl Iterator<Item = Input> {
        self.cases.into_iter().map(|c| c.input)
    }

    /// Creates an iterator over the outputs for this set of cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let inputs = ["this", "and", "those"];
    /// let cases = Cases::from_inputs(inputs, |s| s.len());
    ///
    /// assert!(cases.outputs().eq(&[4, 3, 5]));
    /// ```
    pub fn outputs(&self) -> impl Iterator<Item = &Output> {
        self.cases.iter().map(|c| &c.output)
    }

    /// Creates an iterator over mutable references to the outputs for this set
    /// of cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let inputs = ["this", "and", "those"];
    /// let mut cases = Cases::from_inputs(inputs, |s| s.len());
    ///
    /// for output in cases.outputs_mut() {
    ///     *output *= 2;
    /// }
    ///
    /// assert!(cases.inputs().eq(&["this", "and", "those"]));
    /// assert!(cases.outputs().eq(&[8, 6, 10]));
    /// ```
    pub fn outputs_mut(&mut self) -> impl Iterator<Item = &mut Output> {
        self.cases.iter_mut().map(|c| &mut c.output)
    }

    /// Convert this set of cases into an iterator over the outputs. This
    /// takes ownership of the cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let inputs = ["this", "and", "those"];
    /// let cases = Cases::from_inputs(inputs, |s| s.len());
    ///
    /// assert!(cases.into_outputs().eq([4, 3, 5]));
    /// ```
    pub fn into_outputs(self) -> impl Iterator<Item = Output> {
        self.cases.into_iter().map(|c| c.output)
    }
}

impl<Input, Output> Default for Cases<Input, Output> {
    /// Create an empty set of cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::Cases;
    /// #
    /// let cases = Cases::<String, usize>::default();
    ///
    /// assert!(cases.is_empty());
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Not;

    use super::{Case, Cases, WithTargetFn};
    use crate::list_into::vec_into;

    #[test]
    fn test_from_inputs() {
        let cases = Cases::from_inputs(0..10, |x| x * 2);
        assert_eq!(
            cases.into_iter().collect::<Vec<_>>(),
            (0..10).map(|x| Case::new(x, x * 2)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_from_inputs_different_types() {
        let cases = Cases::from_inputs(0..10, ToString::to_string);
        assert_eq!(
            cases.into_iter().collect::<Vec<_>>(),
            (0..10)
                .map(|x| Case::new(x, x.to_string()))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_add_case() {
        let mut cases = Cases::new();
        cases.add_case((1, 2));
        cases.add_case((3, 6));
        assert_eq!(
            cases.into_iter().collect::<Vec<_>>(),
            vec_into![(1, 2), (3, 6)]
        );
    }

    #[test]
    fn test_with_case() {
        let cases = Cases::new().with_case((1, 2)).with_case((3, 6));
        assert_eq!(
            cases.into_iter().collect::<Vec<_>>(),
            vec_into![(1, 2), (3, 6)]
        );
    }

    #[test]
    fn test_len() {
        let mut cases = Cases::default();
        assert!(cases.is_empty());
        assert_eq!(cases.len(), 0);
        cases.add_case((1, 2));
        assert!(cases.is_empty().not());
        assert_eq!(cases.len(), 1);
        cases.add_case((3, 6));
        assert!(cases.is_empty().not());
        assert_eq!(cases.len(), 2);
    }

    #[test]
    fn test_with_target() {
        let inputs = 0..10;
        let cases = inputs.with_target_fn(|x| x * 2);
        assert_eq!(
            cases.into_iter().collect::<Vec<_>>(),
            (0..10).map(|x| Case::new(x, x * 2)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_iter() {
        let cases = Cases::new()
            .with_case(("Hello", 5))
            .with_case(("People", 6));

        let mut iter = cases.iter();

        assert_eq!(iter.next(), Some(&Case::new("Hello", 5)));
        assert_eq!(iter.next(), Some(&Case::new("People", 6)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut cases = Cases::new()
            .with_case(("Hello", 5))
            .with_case(("People", 6));

        cases.iter_mut().for_each(|c| c.output *= 2);

        let mut iter = cases.iter();

        assert_eq!(iter.next(), Some(&Case::new("Hello", 10)));
        assert_eq!(iter.next(), Some(&Case::new("People", 12)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_inputs_mut() {
        let inputs: Vec<String> = ["this", "and", "those"]
            .iter()
            .map(ToString::to_string)
            .collect();
        let mut cases = Cases::from_inputs(inputs, String::len);

        for input in cases.inputs_mut() {
            *input = input.to_uppercase();
        }

        assert!(cases.inputs().eq(&["THIS", "AND", "THOSE"]));
        assert!(cases.outputs().eq(&[4, 3, 5]));
    }

    #[test]
    fn test_into_inputs() {
        let inputs = ["this", "and", "those"];
        let cases = Cases::from_inputs(inputs, |s| s.len());
        assert!(cases.into_inputs().eq(["this", "and", "those"]));
    }

    #[test]
    fn test_outputs_mut() {
        let inputs: Vec<String> = ["this", "and", "those"]
            .iter()
            .map(ToString::to_string)
            .collect();
        let mut cases = Cases::from_inputs(inputs, String::len);

        for output in cases.outputs_mut() {
            *output *= 2;
        }

        assert!(cases.inputs().eq(&["this", "and", "those"]));
        assert!(cases.outputs().eq(&[8, 6, 10]));
    }

    #[test]
    fn test_into_outputs() {
        let inputs = ["this", "and", "those"];
        let cases = Cases::from_inputs(inputs, |s| s.len());
        assert!(cases.into_outputs().eq([4, 3, 5]));
    }
}
