mod and;
mod map;
mod repeat_with;
mod then;

pub use and::*;
// derive macro
#[doc(hidden)]
pub use ec_macros::Composable;
pub use map::*;
pub use repeat_with::*;
pub use then::*;

/// Create new [`Operator`]'s by composing preexisting [`Operator`]'s!
///
/// This is usually the prefered method for interacting with operators,
/// similarly how one would use composable iterators over defining new iterator
/// types all the time.
///
/// # Example
/// ```
/// # use ec_core::operator::{constant::Constant, identity::Identity, Composable, Operator};
/// # use rand::rng;
/// #
/// let my_operator = Constant::new(false)
///     .apply_n_times::<5>()
///     .map(Identity)
///     .map(Constant::new(5));
///
/// let Ok(my_result) = my_operator.apply((), &mut rng());
/// assert_eq!(my_result, [5; 5]);
/// ```
///
/// [`Operator`]: super::Operator
#[doc(hidden)] // makes the re-export in super the main way to consume the api.
pub trait Composable {
    /// Apply the [`Operator`] `Op` to the output of `Self`.
    ///
    /// This is eqivalent to writing [`Then::new(self, op)`](Then).
    ///
    /// # Example
    ///
    /// ```
    /// # use ec_core::operator::{
    /// #     constant::Constant,
    /// #     identity::Identity,
    /// #     composable::Then,
    /// #     Composable
    /// # };
    /// #
    /// let my_composed_operator = Constant::new(5).then(Identity);
    /// let my_constructed_operator = Then::new(Constant::new(5), Identity);
    ///
    /// assert_eq!(my_composed_operator, my_constructed_operator);
    /// ```
    ///
    /// [`Operator`]: super::Operator
    fn then<Op>(self, op: Op) -> Then<Self, Op>
    where
        Self: Sized,
    {
        Then::new(self, op)
    }

    /// Apply the [`Operator`] `Op` to each individual element in the collection
    /// that is the output of `Self`.
    ///
    /// This is eqivalent to writing [`Then::new(self, Map::new(op))`](Map) or
    /// [`self.then(Map::new(op))`](Map).
    ///
    /// # Example
    ///
    /// ```
    /// # use ec_core::operator::{
    /// #     constant::Constant,
    /// #     identity::Identity,
    /// #     composable::{Then, Map},
    /// #     Composable
    /// # };
    /// #
    /// let my_composed_operator = Constant::new(5).apply_twice().map(Identity);
    /// let my_constructed_operator = Then::new(Constant::new(5).apply_twice(), Map::new(Identity));
    ///
    /// assert_eq!(my_composed_operator, my_constructed_operator);
    /// ```
    ///
    /// [`Operator`]: super::Operator
    fn map<Op>(self, op: Op) -> Then<Self, Map<Op>>
    where
        Self: Sized,
    {
        Then::new(self, Map::new(op))
    }

    /// Create a tuple of values by applying two [`Operator`]'s in parallel,
    /// `Self` and `Op`.
    ///
    /// This is eqivalent to writing [`And::new(self, op)`](And).
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{
    /// #     constant::Constant,
    /// #     composable::And,
    /// #     Composable,
    /// #     Operator
    /// # };
    /// # use rand::rng;
    /// #
    /// let my_composed_operator = Constant::new(5).and(Constant::new(6));
    /// let my_constructed_operator = And::new(Constant::new(5), Constant::new(6));
    ///
    /// assert_eq!(my_composed_operator, my_constructed_operator);
    ///
    /// let Ok(my_value) = my_composed_operator.apply((), &mut rng());
    /// assert_eq!(my_value, (5, 6));
    /// ```
    ///
    /// [`Operator`]: super::Operator
    fn and<Op>(self, op: Op) -> And<Self, Op>
    where
        Self: Sized,
    {
        And::new(self, op)
    }

    /// Create a array of values by applying the [`Operator`] `self` to the
    /// inputs twice in parallel.
    ///
    /// This is equivalent to writing [`RepeatWith::<_,
    /// 2>::new(self)`](RepeatWith).
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{
    /// #     constant::Constant,
    /// #     composable::RepeatWith,
    /// #     Composable,
    /// #     Operator
    /// # };
    /// # use rand::rng;
    /// #
    /// let my_composed_operator = Constant::new(5).apply_twice();
    /// let my_constructed_operator = RepeatWith::<_, 2>::new(Constant::new(5));
    ///
    /// assert_eq!(my_composed_operator, my_constructed_operator);
    ///
    /// let Ok(my_value) = my_composed_operator.apply((), &mut rng());
    /// assert_eq!(my_value, [5, 5]);
    /// ```
    ///
    /// [`Operator`]: super::Operator
    fn apply_twice(self) -> RepeatWith<Self, 2>
    where
        Self: Sized,
    {
        self.apply_n_times::<2>()
    }

    /// Create a array of values by applying the [`Operator`] `self` to the
    /// inputs `N` times in parallel.
    ///
    /// This is equivalent to writing [`RepeatWith::<_,
    /// N>::new(self)`](RepeatWith).
    ///
    /// # Example
    /// ```
    /// # use ec_core::operator::{
    /// #     constant::Constant,
    /// #     composable::RepeatWith,
    /// #     Composable,
    /// #     Operator
    /// # };
    /// # use rand::rng;
    /// #
    /// let my_composed_operator = Constant::new(5).apply_n_times::<5>();
    /// let my_constructed_operator = RepeatWith::<_, 5>::new(Constant::new(5));
    ///
    /// assert_eq!(my_composed_operator, my_constructed_operator);
    ///
    /// let Ok(my_value) = my_composed_operator.apply((), &mut rng());
    /// assert_eq!(my_value, [5, 5, 5, 5, 5]);
    /// ```
    ///
    /// [`Operator`]: super::Operator
    fn apply_n_times<const N: usize>(self) -> RepeatWith<Self, N>
    where
        Self: Sized,
    {
        RepeatWith::new(self)
    }

    /// Construct a new operator from `Self` and a context, providing `Self` as
    /// a value.
    ///
    /// Notably this operates on the operator instead of on operator values.
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     operator::{
    /// #         genome_scorer::GenomeScorer,
    /// #         constant::Constant,
    /// #         Composable,
    /// #         Operator
    /// #     },
    /// #     individual::{
    /// #         scorer::FnScorer,
    /// #         ec::EcIndividual,
    /// #     }
    /// # };
    /// # use rand::rng;
    /// #
    /// let my_individual_generator =
    ///     Constant::new(5).wrap::<GenomeScorer<_, _>>(FnScorer(|x: &i32| *x));
    ///
    /// let Ok(my_individual) = my_individual_generator.apply(&[10], &mut rng());
    ///
    /// assert_eq!(my_individual, EcIndividual::new(5, 5));
    /// ```
    fn wrap<T>(self, context: T::Context) -> T
    where
        T: Wrappable<Self>,
        Self: Sized,
    {
        T::construct(self, context)
    }
}

static_assertions::assert_obj_safe!(Composable);

/// A [`Operator`] which can be constructed by wrapping another [`Operator`]
/// with additional context.
///
/// # Example
/// ```
/// # use ec_core::{
/// #     operator::{
/// #         genome_scorer::GenomeScorer,
/// #         constant::Constant,
/// #         composable::Wrappable,
/// #         Operator,
/// #     },
/// #     individual::{
/// #         scorer::FnScorer,
/// #         ec::EcIndividual,
/// #     }
/// # };
/// # use rand::rng;
/// #
/// let my_individual_generator = GenomeScorer::construct(Constant::new(5), FnScorer(|x: &i32| *x));
///
/// let Ok(my_individual) = my_individual_generator.apply(&[10], &mut rng());
///
/// assert_eq!(my_individual, EcIndividual::new(5, 5));
/// ```
///
/// [`Operator`]: super::Operator
pub trait Wrappable<T> {
    type Context;

    /// Construct a new instance of this [`Operator`]
    ///
    /// # Example
    /// ```
    /// # use ec_core::{
    /// #     operator::{
    /// #         genome_scorer::GenomeScorer,
    /// #         constant::Constant,
    /// #         composable::Wrappable,
    /// #         Operator,
    /// #     },
    /// #     individual::{
    /// #         scorer::FnScorer,
    /// #         ec::EcIndividual,
    /// #     }
    /// # };
    /// # use rand::rng;
    /// #
    /// let my_individual_generator = GenomeScorer::construct(Constant::new(5), FnScorer(|x: &i32| *x));
    /// #
    /// # let Ok(my_individual) = my_individual_generator.apply(&[10], &mut rng());
    /// #
    /// # assert_eq!(my_individual, EcIndividual::new(5, 5));
    /// ```
    ///
    /// [`Operator`]: super::Operator
    fn construct(wrapped: T, context: Self::Context) -> Self;
}
