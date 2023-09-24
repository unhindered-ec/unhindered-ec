pub trait TryRecover<T> {
    type Error;

    /// # Errors
    ///
    /// `x.try_recover()` returns an error if `x` is not a `Recoverable` error type.
    fn try_recover(self) -> Result<T, Self::Error>;
}
