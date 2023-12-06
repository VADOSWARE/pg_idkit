use chrono::{Datelike, Timelike};

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

/// Convert a naive datetime to a Postgres (PGRX) timestamptz
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
pub(crate) fn naive_datetime_to_pg_timestamptz(
    t: impl Datelike + Timelike,
    msg: impl AsRef<str>,
) -> pgrx::TimestampWithTimeZone {
    pgrx::TimestampWithTimeZone::with_timezone(
        t.year(),
        t.month()
            .try_into()
            .or_pgrx_error("failed to convert months"),
        t.day().try_into().or_pgrx_error("failed to convert days"),
        t.hour().try_into().or_pgrx_error("failed to convert hours"),
        t.minute()
            .try_into()
            .or_pgrx_error("failed to convert minutes"),
        t.second()
            .try_into()
            .or_pgrx_error("failed to convert seconds"),
        "utc",
    )
    .or_pgrx_error(msg.as_ref())
}
