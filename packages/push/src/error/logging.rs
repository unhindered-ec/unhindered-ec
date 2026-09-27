use std::error::Error;

use miette::{Diagnostic, MietteDiagnostic, Report, Severity};

pub trait PrintError {
    #[must_use]
    fn print_error(self) -> Self;
}

impl<T, E> PrintError for Result<T, E>
where
    E: Error + Diagnostic + 'static,
{
    fn print_error(self) -> Self {
        if let Err(error) = &self {
            let severity_string = match error.severity() {
                Some(Severity::Advice) => "INFO: ",
                Some(Severity::Error) => "FATAL: ",
                Some(Severity::Warning) => "RECOVERABLE: ",
                None => "",
            };
            eprintln!(
                "{severity_string}{}",
                std::fmt::from_fn(|f| {
                    Report::new(MietteDiagnostic::new(String::new()))
                        .handler()
                        .debug(error, f)
                })
            );
        }
        self
    }
}
