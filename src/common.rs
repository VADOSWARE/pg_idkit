/// A trait that encapsulates things that can be converted to some type
/// or to an error if conversion fails (ex. Result, Option, etc)
pub(crate) trait OrPgxError<T> {
    /// Convert the given type to a T, possibly failing
    /// and calling [`pgrx::error`], with a given prefix if an error is returned
    fn or_pgrx_error(self, prefix: impl AsRef<str>) -> T;
}

impl<T, E> OrPgxError<T> for Result<T, E>
where
    E: std::error::Error,
{
    fn or_pgrx_error(self, prefix: impl AsRef<str>) -> T {
        match self {
            Ok(v) => v,
            Err(e) => pgrx::error!("{}: {e}", prefix.as_ref()),
        }
    }
}

impl<T> OrPgxError<T> for Option<T> {
    fn or_pgrx_error(self, prefix: impl AsRef<str>) -> T {
        match self {
            Some(v) => v,
            None => pgrx::error!("{}", prefix.as_ref()),
        }
    }
}
