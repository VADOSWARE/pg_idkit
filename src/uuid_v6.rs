use std::io::{Error as IoError, ErrorKind};
use std::str::FromStr;

use chrono::NaiveDateTime;
use getrandom::getrandom;
use pgrx::pg_extern;
use uuid::Uuid;

use crate::common::{naive_datetime_to_pg_timestamptz, OrPgrxError};

/// Generate a new UUIDv6
fn new_uuidv6() -> Uuid {
    let mut buf = [0u8; 6];
    getrandom(&mut buf).or_pgrx_error("failed to get random bytes for building uuidv6");
    Uuid::now_v6(&buf)
}

/// Generate a UUID v6
#[pg_extern]
fn idkit_uuidv6_generate() -> String {
    new_uuidv6().as_hyphenated().to_string()
}

/// Generate a UUID v6, producing a Postgres text object
#[pg_extern]
fn idkit_uuidv6_generate_text() -> String {
    idkit_uuidv6_generate()
}

/// Generate a UUID v6, producing a Postgres uuid object
#[pg_extern]
fn idkit_uuidv6_generate_uuid() -> pgrx::Uuid {
    pgrx::Uuid::from_slice(new_uuidv6().as_bytes())
        .map_err(|e| IoError::new(ErrorKind::Other, e))
        .or_pgrx_error("failed to convert UUIDv6 to Postgres uuid type")
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual UUIDv6
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_uuidv6_extract_timestamptz(val: String) -> pgrx::TimestampWithTimeZone {
    let (secs, nanos) = Uuid::from_str(val.as_str())
        .or_pgrx_error(format!("[{val}] is an invalid UUIDv6"))
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
        NaiveDateTime::from_timestamp_opt(secs as i64, nanos)
            .or_pgrx_error("failed to create timestamp from UUIDV6 [{val}]")
            .and_utc(),
        format!("failed to convert timestamp for UUIDV6 [{val}]"),
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

    use crate::uuid_v6::idkit_uuidv6_extract_timestamptz;
    use crate::uuid_v6::idkit_uuidv6_generate;
    use crate::uuid_v6::idkit_uuidv6_generate_uuid;

    /// Basic length test
    #[pg_test]
    fn test_uuidv6_len() {
        assert_eq!(idkit_uuidv6_generate().len(), 36);
    }

    /// Basic length test for bytes
    #[pg_test]
    fn test_uuidv6_len_uuid() {
        assert_eq!(idkit_uuidv6_generate_uuid().len(), 16);
    }

    /// Check version integer in UUID string
    #[pg_test]
    fn test_uuidv6_version_int() {
        let generated = idkit_uuidv6_generate();
        let c9 = generated.chars().nth(14);
        assert!(c9.is_some());
        assert_eq!(c9.unwrap(), '6');
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_uuidv6_extract_timestamptz() {
        let timestamp = idkit_uuidv6_extract_timestamptz(idkit_uuidv6_generate());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert!(
            Utc::now().signed_duration_since(parsed).num_seconds() < 3,
            "extracted, printed & re-parsed uuidv6 timestamp is from recent past (within 3s)"
        );
    }

    /// Ensure an existing, hardcoded timestamp works for extraction
    #[pg_test]
    fn test_uuidv6_extract_timestamptz_existing() {
        idkit_uuidv6_extract_timestamptz("016b0dd7-0cbb-691e-8548-9888e89d0527".into());
    }
}
