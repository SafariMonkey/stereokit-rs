// Dirty is must_use and produced when
// emit_error is called, so its existence
// should be equivalent to an error having
// been emitted.
#[must_use]
pub(crate) struct Dirty;
pub(crate) trait UnwrapDirty<T> {
    fn unwrap_or_abort(self) -> T;
}
impl<T> UnwrapDirty<T> for Result<T, Dirty> {
    fn unwrap_or_abort(self) -> T {
        proc_macro_error::abort_if_dirty();
        match self {
            Ok(v) => v,
            Err(_) => unreachable!("abort_if_dirty should have aborted"),
        }
    }
}
