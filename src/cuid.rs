use chrono::DateTime;
use pgrx::*;

use crate::common::{naive_datetime_to_pg_timestamptz, OrPgrxError};

/// Generate a random cuid UUID
#[pg_extern]
fn idkit_cuid_generate() -> String {
    // Ignore the deprecated version here in case people are using cuid
    warning!(
        "cuid is deprecated in favor of cuid2, consider using cuid2 (also available in pg_idkit)"
    );
    #[allow(deprecated)]
    match cuid::cuid() {
        Err(e) => error!("failed to generate cuid: {}", e),
        Ok(id) => id,
    }
}

/// Generate a random cuid UUID, producing a Postgres text object
#[pg_extern]
fn idkit_cuid_generate_text() -> String {
    idkit_cuid_generate()
}

/// Retrieve a `timestamptz` from a given textual CUID
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_cuid_extract_timestamptz(val: String) -> pgrx::TimestampWithTimeZone {
    #[allow(deprecated)]
    if !cuid::is_cuid(&val) {
        pgrx::error!("value provided is not a valid CUID");
    }

    let millis: i64 = u128::from_str_radix(&val[1..9], 36)
        .or_pgrx_error("failed to base36 decode timestamp")
        .try_into()
        .or_pgrx_error("failed to convert u128 timestamp to i64");

    // Convert to a UTC timestamp
    let now = DateTime::from_timestamp_millis(millis)
        .or_pgrx_error("failed to parse timestamp from millis");

    naive_datetime_to_pg_timestamptz(now, format!("failed to convert timestamp for CUID [{val}]"))
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use chrono::{DateTime, Utc};
    use pgrx::*;

    use crate::cuid::{idkit_cuid_extract_timestamptz, idkit_cuid_generate};

    /// Ensure generated CUIDs are a well-known length
    #[pg_test]
    fn test_cuid_len() {
        let generated = idkit_cuid_generate();
        assert_eq!(generated.len(), 25);
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_cuid_extract_timestamptz() {
        let cuid = idkit_cuid_generate();
        let timestamp = idkit_cuid_extract_timestamptz(cuid.clone());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert_eq!(
            Utc::now().signed_duration_since(parsed).num_seconds(),
            0,
            "extracted, printed & re-parsed cuid timestamp is from recent past (within the same second)"
        );
    }
}
