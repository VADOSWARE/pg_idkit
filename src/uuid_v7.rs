use std::io::{Error as IoError, ErrorKind};
use std::str::FromStr;

use chrono::DateTime;
use pgrx::datum::TimestampWithTimeZone;
use pgrx::pg_extern;
use uuid::Uuid;

use crate::common::{naive_datetime_to_pg_timestamptz, OrPgrxError};

/// Generate a new UUIDv6
fn new_uuidv7() -> Uuid {
    Uuid::now_v7()
}

/// Generate a UUID v7
#[pg_extern]
fn idkit_uuidv7_generate() -> String {
    new_uuidv7().as_hyphenated().to_string()
}

/// Generate a UUID v7, producing a Postgres text object
#[pg_extern]
fn idkit_uuidv7_generate_text() -> String {
    idkit_uuidv7_generate()
}

/// Generate a UUID v7, producing a Postgres uuid object
#[pg_extern]
fn idkit_uuidv7_generate_uuid() -> pgrx::Uuid {
    pgrx::Uuid::from_slice(new_uuidv7().as_bytes())
        .map_err(|e| IoError::new(ErrorKind::Other, format!("{e:?}")))
        .or_pgrx_error("failed to convert UUIDv7 to Postgres uuid type")
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual UUIDv7
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_uuidv7_extract_timestamptz(val: String) -> TimestampWithTimeZone {
    let (secs, nanos) = Uuid::from_str(val.as_str())
        .or_pgrx_error(format!("[{val}] is an invalid UUIDv7"))
        .get_timestamp()
        .or_pgrx_error("failed to extract timestamp")
        .to_unix();
    if secs > i64::MAX as u64 {
        pgrx::error!(
            "value [{secs}] seconds is larger than the max signed 64bit integer [{}]",
            i64::MAX
        );
    }
    naive_datetime_to_pg_timestamptz(
        DateTime::from_timestamp(secs as i64, nanos)
            .or_pgrx_error("failed to create timestamp from UUIDV7 [{val}]"),
        format!("failed to convert timestamp for UUIDV7 [{val}]"),
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

    use crate::uuid_v7::idkit_uuidv7_extract_timestamptz;
    use crate::uuid_v7::idkit_uuidv7_generate;
    use crate::uuid_v7::idkit_uuidv7_generate_uuid;

    /// Basic length test
    #[pg_test]
    fn test_uuidv7_len() {
        let generated = crate::uuid_v7::idkit_uuidv7_generate();
        assert_eq!(generated.len(), 36);
    }

    /// Basic length test for bytes
    #[pg_test]
    fn test_uuidv7_len_uuid() {
        assert_eq!(idkit_uuidv7_generate_uuid().len(), 16);
    }

    /// Check version integer in UUID string
    #[pg_test]
    fn test_uuidv7_version_int() {
        let generated = crate::uuid_v7::idkit_uuidv7_generate();
        let c9 = generated.chars().nth(14);
        assert!(c9.is_some());
        assert_eq!(c9.unwrap(), '7');
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_uuidv7_extract_timestamptz() {
        let timestamp = idkit_uuidv7_extract_timestamptz(idkit_uuidv7_generate());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert_eq!(
            Utc::now().signed_duration_since(parsed).num_seconds(),
            0,
            "extracted, printed & re-parsed uuidv7 timestamp is from recent past (within 1s)"
        );
    }

    /// Ensure an existing, hardcoded timestamp works for extraction
    #[pg_test]
    fn test_uuidv7_extract_timestamptz_existing() {
        idkit_uuidv7_extract_timestamptz("016b0dd7-0cbb-691e-8548-9888e89d0527".into());
    }
}
