use std::{io::Write, string::FromUtf8Error};

pub trait HasStdout {
    type Stdout: Write;

    fn stdout(&mut self) -> &mut Self::Stdout;

    /// Return the contents of `stdout` as a `String`
    ///
    /// # Errors
    ///
    /// Returns a `FromUtf8Error` if there is a problem converting
    /// the contents of `Self::Stdout` into a `String`.
    fn stdout_string(&mut self) -> Result<String, FromUtf8Error>;
}
