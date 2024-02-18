#[derive(Debug)]
pub struct Case<Input, Output = Input> {
    pub input: Input,
    pub output: Output,
}

#[derive(Debug)]
pub struct Cases<Input, Output = Input> {
    cases: Vec<Case<Input, Output>>,
}

impl<Input, Output> Cases<Input, Output> {
    #[must_use]
    pub const fn new() -> Self {
        Self { cases: Vec::new() }
    }

    pub fn from_inputs(
        inputs: impl Iterator<Item = Input>,
        target_function: impl Fn(&Input) -> Output,
    ) -> Self {
        inputs
            .map(|input| Case {
                output: target_function(&input),
                input,
            })
            .collect()
    }

    // TODO: Add `from` that selects randomly from some cases
    // TODO: Add `from` that gets cases from an external source
    //    Maybe outside of this type?

    pub fn add_case(&mut self, input: Input, output: Output) {
        self.cases.push(Case { input, output });
    }

    pub fn iter(&self) -> std::slice::Iter<Case<Input, Output>> {
        self.cases.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Case<Input, Output>> {
        self.cases.iter_mut()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cases.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.cases.len()
    }
}

impl<Input, Output> Default for Cases<Input, Output> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Input, Output, C> FromIterator<C> for Cases<Input, Output>
where
    C: Into<Case<Input, Output>>,
{
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        Self {
            cases: iter.into_iter().map(Into::into).collect(),
        }
    }
}

impl<Input, Output> IntoIterator for Cases<Input, Output> {
    type Item = Case<Input, Output>;
    type IntoIter = std::vec::IntoIter<Case<Input, Output>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cases.into_iter()
    }
}

impl<'a, Input, Output> IntoIterator for &'a Cases<Input, Output> {
    type Item = &'a Case<Input, Output>;
    type IntoIter = std::slice::Iter<'a, Case<Input, Output>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cases.iter()
    }
}

impl<'a, Input, Output> IntoIterator for &'a mut Cases<Input, Output> {
    type Item = &'a mut Case<Input, Output>;
    type IntoIter = std::slice::IterMut<'a, Case<Input, Output>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cases.iter_mut()
    }
}
