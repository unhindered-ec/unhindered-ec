pub mod ec;
pub mod scorer;

/// A individual in the ec system
///
/// This consists of a genome describing the individual and test results for
/// evaluating the Individual
///
/// Also see [`EcIndividual`](ec::EcIndividual)
///
/// # Example[^ec-linear-usage]
/// ```
/// # use ec_core::{test_results::Score, individual::Individual};
/// # use ec_linear::genome::bitstring::Bitstring;
/// #
/// # #[allow(dead_code)]
/// struct ScoredIndividual {
///     genome: Bitstring,
///     score: Score<u32>,
/// }
///
/// impl Individual for ScoredIndividual {
///     type Genome = Bitstring;
///     type TestResults = Score<u32>;
///
///     fn genome(&self) -> &Bitstring {
///         &self.genome
///     }
///
///     fn test_results(&self) -> &Score<u32> {
///         &self.score
///     }
/// }
/// ```
/// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
///     dependency of this package to demonstrate some concepts which need
///     concrete implementations. If you want to replicate this example, make
///     sure [`ec-linear`][ec-linear] is installed.
///
/// [ec-linear]: #
pub trait Individual {
    /// The type of the genome of this individual
    type Genome;
    /// The type of the evaluation results of this individual
    type TestResults;

    /// Get the genome of this individual
    ///
    /// # Example[^ec-linear-usage]
    /// ```
    /// # use ec_core::{test_results::Score, individual::Individual};
    /// # use ec_linear::genome::bitstring::Bitstring;
    /// #
    /// # struct ScoredIndividual {
    /// #     genome: Bitstring,
    /// #     score: Score<u32>,
    /// # }
    /// #
    /// # impl Individual for ScoredIndividual {
    /// #     type Genome = Bitstring;
    /// #     type TestResults = Score<u32>;
    /// #
    /// #     fn genome(&self) -> &Bitstring {
    /// #         &self.genome
    /// #     }
    /// #
    /// #     fn test_results(&self) -> &Score<u32> {
    /// #         &self.score
    /// #     }
    /// # }
    /// #
    /// let my_individual = ScoredIndividual {
    ///     genome: [true, false, true].into_iter().collect::<Bitstring>(),
    ///     score: Score(2),
    /// };
    ///
    /// assert_eq!(
    ///     my_individual.genome(),
    ///     &[true, false, true].into_iter().collect::<Bitstring>()
    /// );
    /// ```
    /// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
    ///     dependency of this package to demonstrate some concepts which need
    ///     concrete implementations. If you want to replicate this example,
    /// make     sure [`ec-linear`][ec-linear] is installed.
    ///
    /// [ec-linear]: #
    fn genome(&self) -> &Self::Genome;

    /// Get the evaluation results of this individual
    ///
    /// # Example[^ec-linear-usage]
    /// ```
    /// # use ec_core::{test_results::Score, individual::Individual};
    /// # use ec_linear::genome::bitstring::Bitstring;
    /// #
    /// # struct ScoredIndividual {
    /// #     genome: Bitstring,
    /// #     score: Score<u32>,
    /// # }
    /// #
    /// # impl Individual for ScoredIndividual {
    /// #     type Genome = Bitstring;
    /// #     type TestResults = Score<u32>;
    /// #
    /// #     fn genome(&self) -> &Bitstring {
    /// #         &self.genome
    /// #     }
    /// #
    /// #     fn test_results(&self) -> &Score<u32> {
    /// #         &self.score
    /// #     }
    /// # }
    /// #
    /// let my_individual = ScoredIndividual {
    ///     genome: [true, false, true].into_iter().collect::<Bitstring>(),
    ///     score: Score(2),
    /// };
    ///
    /// assert_eq!(my_individual.test_results(), &2);
    /// ```
    /// [^ec-linear-usage]: Note that this example uses [`ec-linear`][ec-linear] which is not a
    ///     dependency of this package to demonstrate some concepts which need
    ///     concrete implementations. If you want to replicate this example,
    ///     make sure [`ec-linear`][ec-linear] is installed.
    ///
    /// [ec-linear]: #
    fn test_results(&self) -> &Self::TestResults;
}

static_assertions::assert_obj_safe!(Individual<Genome = (), TestResults = ()>);
