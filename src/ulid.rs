use std::str::FromStr;

use chrono::DateTime;
use pgrx::datum::TimestampWithTimeZone;
use pgrx::*;
use ulid::Ulid;
use uuid::Uuid;

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

/// Generate a ULID from an existing UUID
#[pg_extern]
fn idkit_ulid_from_uuid(uuid: pgrx::Uuid) -> String {
    let s = uuid.to_string();
    Ulid::from(Uuid::from_str(&s).or_pgrx_error(format!("failed to parse UUID [{s}]"))).to_string()
}

/// Generate a ULID from text of an existing UUID
#[pg_extern]
fn idkit_ulid_from_uuid_text(uuid: String) -> String {
    Ulid::from(Uuid::from_str(&uuid).or_pgrx_error(format!("failed to parse UUID text [{uuid}]")))
        .to_string()
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual ULID
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_ulid_extract_timestamptz(val: String) -> TimestampWithTimeZone {
    let ulid = Ulid::from_string(val.as_ref()).or_pgrx_error(format!("[{val}] is an invalid ULID"));
    naive_datetime_to_pg_timestamptz(
        DateTime::from_timestamp_millis(
            ulid.timestamp_ms()
                .try_into()
                .or_pgrx_error("failed to convert ulid timestamp milliseconds"),
        )
        .or_pgrx_error("failed to create timestamp from ULID [{val}]"),
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
    use uuid::Uuid;

    use crate::common::OrPgrxError;
    use crate::ulid::idkit_ulid_extract_timestamptz;
    use crate::ulid::idkit_ulid_from_uuid;
    use crate::ulid::idkit_ulid_from_uuid_text;
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
        assert_eq!(
            Utc::now().signed_duration_since(parsed).num_seconds(),
            0,
            "extracted, printed & re-parsed ulid timestamp is from recent past (within 1s)"
        );
    }

    /// Ensure an existing, hardcoded value works for extraction
    #[pg_test]
    fn test_ulid_extract_timestamptz_existing() {
        idkit_ulid_extract_timestamptz("01ARZ3NDEKTSV4RRFFQ69G5FAV".into());
    }

    /// Ensure a PG UUID can be turned into a ULID
    #[pg_test]
    fn test_ulid_from_uuid() {
        idkit_ulid_from_uuid(
            pgrx::Uuid::from_slice(Uuid::now_v7().as_bytes())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{e:?}")))
                .or_pgrx_error("failed to convert"),
        );
    }

    /// Ensure a textual UUID can be turned into a ULID
    #[pg_test]
    fn test_ulid_from_uuid_text() {
        idkit_ulid_from_uuid_text(Uuid::now_v7().to_string());
    }
}
