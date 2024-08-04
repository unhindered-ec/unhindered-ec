use super::super::{Case, Cases};

// A collection of iterators on `Cases`.

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
    /// for c in cases.iter_mut() {
    ///     (*c).output *= 2;
    /// }
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
    ///     .map(|s| s.to_string())
    ///     .collect();
    /// let mut cases = Cases::from_inputs(inputs, |s| s.len());
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
