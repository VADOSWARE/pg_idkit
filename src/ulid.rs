use chrono::NaiveDateTime;
use pgrx::*;
use ulid::Ulid;

use crate::common::{naive_datetime_to_pg_timestamptz, OrPgrxError};

/// Generate a ULID
#[pg_extern]
fn idkit_ulid_generate() -> String {
    Ulid::new().to_string()
}

/// Generate a ULID, producing a Postgres text object
#[pg_extern]
fn idkit_ulid_generate_text() -> String {
    idkit_ulid_generate()
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual ULID
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_ulid_extract_timestamptz(val: String) -> pgrx::TimestampWithTimeZone {
    let ulid = Ulid::from_string(val.as_ref()).or_pgrx_error(format!("[{val}] is an invalid ULID"));
    naive_datetime_to_pg_timestamptz(
        NaiveDateTime::from_timestamp_millis(
            ulid.timestamp_ms()
                .try_into()
                .or_pgrx_error("failed to convert ulid timestamp milliseconds"),
        )
        .or_pgrx_error("failed to create timestamp from ULID [{val}]")
        .and_utc(),
        format!("failed to convert timestamp for ULID [{val}]"),
    )
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use chrono::{DateTime, Utc};
    use pgrx::datum::datetime_support::ToIsoString;
    use pgrx::pg_test;

    use crate::ulid::idkit_ulid_extract_timestamptz;
    use crate::ulid::idkit_ulid_generate_text;

    /// Basic length test
    #[pg_test]
    fn test_ulid_len() {
        let generated = crate::ulid::idkit_ulid_generate();
        assert_eq!(generated.len(), 26);
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_ulid_extract_timestamptz() {
        let timestamp = idkit_ulid_extract_timestamptz(idkit_ulid_generate_text());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert!(
            Utc::now().signed_duration_since(parsed).num_seconds() < 3,
            "extracted, printed & re-parsed ulid timestamp is from recent past (within 3s)"
        );
    }

    /// Ensure an existing, hardcoded timestamp works for extraction
    #[pg_test]
    fn test_ulid_extract_timestamptz_existing() {
        idkit_ulid_extract_timestamptz("01ARZ3NDEKTSV4RRFFQ69G5FAV".into());
    }
}
