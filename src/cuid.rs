use std::io;

use chrono::{Datelike, NaiveDateTime, Timelike};
use cuid;
use pgrx::*;

use crate::common::OrPgxError;

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
/// If the value is not a valid CUID, this function will throw an error
#[pg_extern]
fn idkit_cuid_extract_timestamptz(val: String) -> pgrx::TimestampWithTimeZone {
    #[allow(deprecated)]
    if !cuid::is_cuid(&val) {
        pgrx::error!("value provided is not a valid CUID");
    }

    // Get the base64 epoch milliseconds
    let epoch_millis = base36::decode(&val[1..9])
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{e:?}")))
        .or_pgrx_error("failed to base64 decode cuid timestamp");
    let epoch_millis_len = epoch_millis.len();

    if epoch_millis_len > 8 {
        pgrx::error!("unexpected length of bytes [{}]", epoch_millis.len());
    };

    // Copy in bytes from epoch conversion, copy to i64
    let mut be_bytes = [0; 8];
    be_bytes[8 - epoch_millis_len..].copy_from_slice(&epoch_millis);
    let millis_i64 = i64::from_be_bytes(be_bytes);

    // Convert to a UTC timestamp
    let now = NaiveDateTime::from_timestamp_millis(millis_i64)
        .or_pgrx_error("failed to parse timestamp from millis")
        .and_utc();

    pgrx::TimestampWithTimeZone::with_timezone(
        now.year(),
        now.month()
            .try_into()
            .or_pgrx_error("failed to convert months"),
        now.day().try_into().or_pgrx_error("failed to convert days"),
        now.hour()
            .try_into()
            .or_pgrx_error("failed to convert hours"),
        now.minute()
            .try_into()
            .or_pgrx_error("failed to convert minutes"),
        now.second()
            .try_into()
            .or_pgrx_error("failed to convert seconds"),
        "utc",
    )
    .or_pgrx_error("failed to convert timestamp")
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
    fn test_len() {
        let generated = idkit_cuid_generate();
        assert_eq!(generated.len(), 25);
    }

    /// Ensure timestamps extracted from CUIDs are valid
    #[pg_test]
    fn test_extract_timestamptz() {
        let cuid = idkit_cuid_generate();
        let timestamp = idkit_cuid_extract_timestamptz(cuid.clone());
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert!(
            Utc::now().signed_duration_since(parsed).num_seconds() < 3,
            "extracted, printed & re-parsed cuid timestamp is from recent past (within 3s)"
        );
    }
}
