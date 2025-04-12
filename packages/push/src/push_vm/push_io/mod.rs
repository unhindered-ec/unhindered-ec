use std::io::Write;

pub trait HasStdout {
    type Stdout: Write;

    /// Return a mutable reference to this instance's associated
    /// `Stdout` type.
    ///
    /// The associated `Stdout` type must implement the `std::io::Write` trait.
    /// This allows callers to then call writing functions (like the `write!`
    /// macro) to add content to this `Stdout` value, similar to using
    /// `print!()` and `println!()` for the normal `stdout`.
    fn stdout(&mut self) -> &mut Self::Stdout;
}
