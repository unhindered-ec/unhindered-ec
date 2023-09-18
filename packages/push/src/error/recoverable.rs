use std::convert::Infallible;

use super::try_recover::TryRecover;

/// Error
///
/// - `state`: The state of the system _before_ attempting to perform
///     the instruction that generated this error
/// - `error`: The cause of this error
/// - `error_kind`: Whether this error is `Fatal` (i.e., whether program execution
///     should terminate immediately) or `Recoverable` (i.e., this instruction
///     should just be skipped and the program execution continues with the
///     next instruction).
#[derive(Debug)]
pub struct RecoverableError<S, E> {
    // Without the `Box` the size of this Error ended up being 156 bytes
    // with a `PushState` and a `PushInstructionError`. That led to a Clippy
    // warning (https://rust-lang.github.io/rust-clippy/master/index.html#/result_large_err)
    // our `Error` was then larger than the 128 byte limit. They recommended boxing
    // the big piece (the state in our case), and doing that brought the size down to
    // 40 bytes. Since `Error`s are only constructed through `::fatal()` or `::recoverable()`,
    // we'd nicely encapsulated this and only had to make changes in those two places to
    // get things working.
    pub(super) state: Box<S>,
    pub(super) error: E,
}

impl<S, E> RecoverableError<S, E> {
    pub fn into_state(self) -> S {
        *self.state
    }
}
