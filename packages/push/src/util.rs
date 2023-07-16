// When this code was suggested (by MizardX@Twitch) they included the
// `inline(always)` annotation. Clippy is then fussy about this, because
// it's often overused by people who haven't done the testing
// necessary to figure out if it's actually needed. My guess is
// that it is actually a Good Thing, and that we should bring
// it back (with an `allow` annotation to make Clippy happy),
// but it would be good to have the testing to back it up.
// #[inline(always)]
pub fn pop2<T>(stack: &mut Vec<T>) -> Option<(T, T)> {
    if stack.len() >= 2 {
        let x = stack.pop()?;
        let y = stack.pop()?;
        Some((x, y))
    } else {
        None
    }
}
