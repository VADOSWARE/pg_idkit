use chrono::NaiveDateTime;
use pgrx::pg_extern;
use std::io::{Error as IoError, ErrorKind};
use timeflake_rs::Timeflake;

use crate::common::{naive_datetime_to_pg_timestamptz, OrPgrxError};

/// Generate a random timeflake UUID
#[pg_extern]
fn idkit_timeflake_generate() -> String {
    Timeflake::random()
        .or_pgrx_error("failed to generate timeflake")
        .to_string()
}

/// Generate a random timeflake UUID, producing a Postgres text object
#[pg_extern]
fn idkit_timeflake_generate_text() -> String {
    idkit_timeflake_generate()
}

/// Generate a random timeflake UUID, producing a Postgres text object
#[pg_extern]
fn idkit_timeflake_generate_uuid() -> pgrx::Uuid {
    pgrx::Uuid::from_slice(
        Timeflake::random()
            .or_pgrx_error("failed to generate timeflake")
            .as_uuid()
            .as_bytes(),
    )
    .map_err(|e| IoError::new(ErrorKind::Other, e))
    .or_pgrx_error("failed to convert Timeflake to Postgres uuid type")
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual Timeflake
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_timeflake_extract_timestamptz(val: String) -> pgrx::TimestampWithTimeZone {
    let timeflake =
        Timeflake::parse(val.as_ref()).or_pgrx_error(format!("[{val}] is an invalid Timeflake"));
    naive_datetime_to_pg_timestamptz(
        NaiveDateTime::from_timestamp_millis(
            timeflake
                .timestamp
                .as_millis()
                .try_into()
                .or_pgrx_error("failed to convert timeflake timestamp milliseconds"),
        )
        .or_pgrx_error("failed to create timestamp from Timeflake [{val}]")
        .and_utc(),
        format!("failed to convert timestamp for Timeflake [{val}]"),
    )
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use chrono::{DateTime, Utc};
    use pgrx::datum::datetime_support::ToIsoString;
    use pgrx::pg_test;
    use uuid::Uuid;

    use crate::timeflake::idkit_timeflake_extract_timestamptz;
    use crate::timeflake::idkit_timeflake_generate_text;
    use crate::timeflake::idkit_timeflake_generate_uuid;

    /// Basic length test
    #[pg_test]
    fn test_timeflake_len() {
        assert_eq!(idkit_timeflake_generate_text().len(), 36);
    }

    /// Ensure UUIDs generated from a Timeflake are valid
    #[pg_test]
    fn test_timeflake_uuid_len() {
        let uuid = idkit_timeflake_generate_uuid().to_string();
        assert_eq!(uuid.len(), 36);
        assert!(Uuid::parse_str(&uuid.to_string()).is_ok())
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_timeflake_extract_timestamptz() {
        let timestamp = idkit_timeflake_extract_timestamptz(idkit_timeflake_generate_text());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert_eq!(
            Utc::now().signed_duration_since(parsed).num_seconds(),
            0,
            "extracted, printed & re-parsed timeflake timestamp is from recent past (within 1s)"
        );
    }

    /// Ensure an existing, hardcoded timestamp works for extraction
    #[pg_test]
    fn test_timeflake_extract_timestamptz_existing() {
        idkit_timeflake_extract_timestamptz("0004fbc6872f70fc9e27355a499e8b6d".into()); // base62
        idkit_timeflake_extract_timestamptz("016fa936bff0997a0a3c428548fee8c9".into()); // hex
    }
}
