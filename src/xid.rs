use std::str::FromStr;
use std::time::UNIX_EPOCH;

use chrono::DateTime;
use pgrx::datum::TimestampWithTimeZone;
use pgrx::pg_extern;
use xid::{new as generate_xid, Id as Xid};

use crate::common::{naive_datetime_to_pg_timestamptz, OrPgrxError};

/// Generate a random xid UUID
#[pg_extern]
fn idkit_xid_generate() -> String {
    generate_xid().to_string()
}

/// Generate a random xid UUID, producing a Postgres text object
#[pg_extern]
fn idkit_xid_generate_text() -> String {
    idkit_xid_generate()
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual XID
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_xid_extract_timestamptz(val: String) -> TimestampWithTimeZone {
    let xid = Xid::from_str(val.as_ref()).or_pgrx_error(format!("[{val}] is an invalid XID"));
    naive_datetime_to_pg_timestamptz(
        DateTime::from_timestamp_millis(
            xid.time()
                .duration_since(UNIX_EPOCH)
                .or_pgrx_error("failed to convert XID type to timestamp milliseconds")
                .as_millis()
                .try_into()
                .or_pgrx_error("failed to convert unix timestamp milliseconds"),
        )
        .or_pgrx_error("failed to create timestamp from XID [{val}]"),
        format!("failed to convert timestamp for XID [{val}]"),
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

    use crate::xid::idkit_xid_extract_timestamptz;
    use crate::xid::idkit_xid_generate_text;

    /// Basic length test
    #[pg_test]
    fn test_xid_len() {
        let generated = crate::xid::idkit_xid_generate();
        assert_eq!(generated.len(), 20);
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_xid_extract_timestamptz() {
        let timestamp = idkit_xid_extract_timestamptz(idkit_xid_generate_text());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert_eq!(
            Utc::now().signed_duration_since(parsed).num_seconds(),
            0,
            "extracted, printed & re-parsed xid timestamp is from recent past (within 1s)"
        );
    }

    /// Ensure an existing, hardcoded value works for extraction
    #[pg_test]
    fn test_xid_extract_timestamptz_existing() {
        idkit_xid_extract_timestamptz("bva9lbqn1bt68k8mj62g".into());
    }
}
