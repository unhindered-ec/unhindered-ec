//! A collection of implementations converting `Cases` into various iterators
//! and constructing them from iterators.

use super::super::{Case, Cases};

impl<Input, Output> IntoIterator for Cases<Input, Output> {
    type Item = Case<Input, Output>;
    type IntoIter = std::vec::IntoIter<Case<Input, Output>>;

    /// Converts a set of `Cases` into an iterator over the individual cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Case, Cases, WithTargetFn};
    /// #
    /// let inputs = ["Hello", "People"];
    /// let cases = inputs.with_target_fn(|s| s.len());
    ///
    /// let mut iter = cases.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(Case::new("Hello", 5)));
    /// assert_eq!(iter.next(), Some(Case::new("People", 6)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.cases.into_iter()
    }
}

impl<'a, Input, Output> IntoIterator for &'a Cases<Input, Output> {
    type Item = &'a Case<Input, Output>;
    type IntoIter = std::slice::Iter<'a, Case<Input, Output>>;

    /// Converts a reference to a set of `Cases` into an iterator over the
    /// individual cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Case, Cases};
    /// #
    /// let cases = Cases::new()
    ///     .with_case(Case::new("Hello", 5))
    ///     .with_case(Case::new("People", 6));
    ///
    /// let mut iter = (&cases).into_iter();
    ///
    /// assert_eq!(iter.next(), Some(&Case::new("Hello", 5)));
    /// assert_eq!(iter.next(), Some(&Case::new("People", 6)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.cases.iter()
    }
}

impl<'a, Input, Output> IntoIterator for &'a mut Cases<Input, Output> {
    type Item = &'a mut Case<Input, Output>;
    type IntoIter = std::slice::IterMut<'a, Case<Input, Output>>;

    /// Returns an iterator over mutable references to the cases in the
    /// `Cases` struct.
    ///
    /// This allows the `Cases` struct to be used mutably in a `for` loop or
    /// other iterator-based operations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Case, Cases};
    /// #
    /// let mut cases = Cases::new()
    ///     .with_case(Case::new("Hello", 5))
    ///     .with_case(Case::new("People", 6));
    ///
    /// for c in &mut cases {
    ///     (*c).output *= 2
    /// }
    ///
    /// let mut iter = cases.into_iter();
    ///
    /// assert_eq!(iter.next(), Some(Case::new("Hello", 10)));
    /// assert_eq!(iter.next(), Some(Case::new("People", 12)));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.cases.iter_mut()
    }
}

impl<Input, Output, C> FromIterator<C> for Cases<Input, Output>
where
    C: Into<Case<Input, Output>>,
{
    /// Create a set of cases from an iterator over items that can
    /// be converted into individual cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use push::evaluation::{Case, Cases};
    /// #
    /// let items = [("Hello", 5), ("People", 6)];
    /// let cases = Cases::from_iter(items);
    ///
    /// assert!(cases.inputs().eq(&["Hello", "People"]));
    /// assert!(cases.outputs().eq(&[5, 6]));
    /// ```
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        Self {
            cases: iter.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use test_strategy::proptest;

    use crate::evaluation::{Case, Cases};

    #[proptest]
    fn from_iterator(#[any] pairs: Vec<(String, i32)>) {
        let cases = pairs.clone().into_iter().collect::<Cases<_, _>>();

        assert_eq!(cases.len(), pairs.len());

        assert!(
            zip(pairs, cases).all(|((s, x), Case { input, output })| s == input && x == output)
        );
    }
}
