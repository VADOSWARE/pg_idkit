use chrono::NaiveDateTime;
use pgrx::*;
use std::str::FromStr;
use svix_ksuid::{Ksuid, KsuidLike};

use crate::common::{naive_datetime_to_pg_timestamptz, OrPgrxError};

/// Generate a random KSUID (HEX-encoded), with millisecond precision
#[pg_extern]
fn idkit_ksuid_generate() -> String {
    Ksuid::new(None, None).to_string()
}

/// Generate a random KSUID, producing a Postgres text object (HEX-encoded)
#[pg_extern]
fn idkit_ksuid_generate_text() -> String {
    idkit_ksuid_generate()
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual KSUID
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_ksuid_extract_timestamptz(val: String) -> pgrx::TimestampWithTimeZone {
    let ksuid = Ksuid::from_str(val.as_ref()).or_pgrx_error(format!("[{val}] is an invalid KSUID"));
    naive_datetime_to_pg_timestamptz(
        NaiveDateTime::from_timestamp_opt(ksuid.timestamp_seconds(), 0)
            .or_pgrx_error("failed to create timestamp from KSUID [{val}]")
            .and_utc(),
        format!("failed to convert timestamp for KSUID [{val}]"),
    )
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use chrono::{DateTime, Utc};
    use pgrx::*;

    use crate::ksuid::idkit_ksuid_extract_timestamptz;
    use crate::ksuid::idkit_ksuid_generate;

    /// Basic length test (ksuids are always 27 characters)
    #[pg_test]
    fn test_ksuid_len() {
        assert_eq!(idkit_ksuid_generate().len(), 27);
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_ksuid_extract_timestamptz() {
        let timestamp = idkit_ksuid_extract_timestamptz(idkit_ksuid_generate());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert_eq!(
            Utc::now().signed_duration_since(parsed).num_seconds(),
            0,
            "extracted, printed & re-parsed ksuid timestamp is from recent past (within 1s)"
        );
    }

    /// Ensure an existing, hardcoded timestamp works for extraction
    #[pg_test]
    fn test_ksuid_extract_timestamptz_existing() {
        idkit_ksuid_extract_timestamptz("1srOrx2ZWZBpBUvZwXKQmoEYga2".into());
    }
}
