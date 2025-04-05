use std::{io::Write, string::FromUtf8Error};

pub trait HasStdout {
    type Stdout: Write;

    fn stdout(&mut self) -> &mut Self::Stdout;
    /// Return the contents of `stdout` as a `String`
    fn stdout_string(&mut self) -> Result<String, FromUtf8Error>;
}
