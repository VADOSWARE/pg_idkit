use chrono::NaiveDateTime;
use pgrx::*;

use crate::common::naive_datetime_to_pg_timestamptz;
use crate::common::OrPgrxError;
use crate::vendor::pushid::PushId;
use crate::vendor::pushid::PushIdGen;

/// Generate a random PushID UUID
#[pg_extern]
fn idkit_pushid_generate() -> String {
    PushId::new().get_id()
}

/// Generate a random PushID UUID, producing a Postgres text object
#[pg_extern]
fn idkit_pushid_generate_text() -> String {
    idkit_pushid_generate()
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual KSUID
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_pushid_extract_timestamptz(val: String) -> pgrx::TimestampWithTimeZone {
    let pushid =
        PushId::from_str(val.as_ref()).or_pgrx_error(format!("[{val}] is an invalid PushID"));
    naive_datetime_to_pg_timestamptz(
        NaiveDateTime::from_timestamp_opt(pushid.timestamp_seconds(), 0)
            .or_pgrx_error("failed to create timestamp from PushID [{val}]")
            .and_utc(),
        format!("failed to convert timestamp for PUSHID [{val}]"),
    )
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use chrono::{DateTime, Utc};
    use pgrx::pg_test;

    use crate::pushid::idkit_pushid_extract_timestamptz;
    use crate::pushid::idkit_pushid_generate;

    /// Basic length test
    #[pg_test]
    fn test_pushid_len() {
        let generated = crate::pushid::idkit_pushid_generate();
        assert_eq!(generated.len(), 20);
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_pushid_extract_timestamptz() {
        let timestamp = idkit_pushid_extract_timestamptz(idkit_pushid_generate());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert!(
            Utc::now().signed_duration_since(parsed).num_seconds() < 3,
            "extracted, printed & re-parsed pushid timestamp is from recent past (within 3s)"
        );
    }

    /// Ensure an existing, hardcoded timestamp works for extraction
    #[pg_test]
    fn test_pushid_extract_timestamptz_existing() {
        idkit_pushid_extract_timestamptz("1srOrx2ZWZBpBUvZwXKQmoEYga2".into());
    }
}
