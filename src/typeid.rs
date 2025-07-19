use std::str::FromStr;

use anyhow::{anyhow, ensure, Context, Result};
use chrono::DateTime;
use pgrx::datum::TimestampWithTimeZone;
use pgrx::*;
use type_safe_id::{DynamicType, TypeSafeId};
use uuid::Uuid;

use crate::common::{naive_datetime_to_pg_timestamptz, OrPgrxError};

/// A wrapper class for UUIDs that are v7s
#[derive(Debug)]
struct Uuidv7(Uuid);

impl Uuidv7 {
    /// Convert a Uuidv7 into it's underlying Uuid
    fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl FromStr for Uuidv7 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let uuid = Uuid::from_str(s).map_err(|e| anyhow!("invalid UUID [{s}]: {e}"))?;
        ensure!(
            matches!(uuid.get_version(), Some(uuid::Version::SortRand)),
            "invalid non-v7 UUID"
        );
        Ok(Self(uuid))
    }
}

/// Generate a random typeid (type safe ID) with UUIDv7, given a type prefix
///
/// In order to avoid failing, this always returns an `unknown` type if the
/// provided prefix is invalid
#[pg_extern]
fn idkit_typeid_generate(prefix: &str) -> String {
    typeid_generate(prefix).or_else_pgrx_error(|| format!("failed to generate"))
}

fn typeid_generate(prefix: &str) -> Result<String> {
    let ty =
        DynamicType::new(prefix).with_context(|| format!("invalid typeid prefix [{prefix}]"))?;
    Ok(TypeSafeId::from_type_and_uuid(ty, Uuid::now_v7()).to_string())
}

/// Generate a random typeid, producing a Postgres text object (HEX-encoded)
#[pg_extern]
fn idkit_typeid_generate_text(prefix: &str) -> String {
    typeid_generate_text(prefix).or_else_pgrx_error(|| format!("failed to generate"))
}

fn typeid_generate_text(prefix: &str) -> Result<String> {
    typeid_generate(prefix)
}

/// Generate a TypeID with a given UUIDv7
///
/// If given an invalid UUIDv7, a new one will be created.
#[pg_extern]
fn idkit_typeid_from_uuid_v7(prefix: &str, uuid: &str) -> String {
    typeid_from_uuid_v7(prefix, uuid)
        .or_else_pgrx_error(|| format!("failed to generate from uuid v7"))
}

fn typeid_from_uuid_v7(prefix: &str, uuid: &str) -> Result<String> {
    let ty = DynamicType::new(prefix).with_context(|| format!("invalid type prefix"))?;
    let uuid = Uuidv7::from_str(uuid).with_context(|| format!("invalid UUID v7 provided"))?;
    Ok(TypeSafeId::from_type_and_uuid(ty, uuid.into_uuid()).to_string())
}

/// Retrieve a `timestamptz` (with millisecond precision) from a given textual typeid
///
/// # Panics
///
/// This function panics (with a [`pgrx::error`]) when the timezone can't be created
#[pg_extern]
fn idkit_typeid_extract_timestamptz(val: String) -> TimestampWithTimeZone {
    typeid_extract_timestamptz(val).or_else_pgrx_error(|| format!("failed to extract timestamp"))
}

fn typeid_extract_timestamptz(val: String) -> Result<TimestampWithTimeZone> {
    let uuid = TypeSafeId::<DynamicType>::from_str(val.as_ref())
        .with_context(|| format!("[{val}] is an invalid typeid"))?
        .uuid();
    ensure_uuidv7(uuid)?;
    let (secs, nsecs) = uuid
        .get_timestamp()
        .with_context(|| format!("invalid uuid, failed to extract timestamp"))?
        .to_unix();
    Ok(naive_datetime_to_pg_timestamptz(
        DateTime::from_timestamp(
            secs.try_into()
                .with_context(|| format!("seconds overflow failed to convert [{secs}] into i64"))?,
            nsecs,
        )
        .with_context(|| format!("failed to create timestamp from typeid [{val}]"))?,
        format!("failed to convert timestamp for typeid [{val}]"),
    ))
}

/// Ensure a given UUID is a uuidv7
fn ensure_uuidv7(u: Uuid) -> Result<()> {
    ensure!(
        matches!(u.get_version(), Some(uuid::Version::SortRand)),
        "[{u}] typeid contains non-v7 uuid"
    );
    Ok(())
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use chrono::{DateTime, Utc};
    use pgrx::prelude::ToIsoString;
    use pgrx::*;

    use crate::typeid::{
        idkit_typeid_extract_timestamptz, idkit_typeid_from_uuid_v7, idkit_typeid_generate,
        idkit_typeid_generate_text, typeid_from_uuid_v7,
    };

    /// Basic length test (a typeid with prefix 'text' should be X characters)
    #[pg_test]
    fn test_typeid_len() {
        let id = idkit_typeid_generate("test");
        assert_eq!(id.len(), 31);
        assert!(id.starts_with("test_"));
        let id_text = idkit_typeid_generate_text("test");
        assert_eq!(id_text.len(), 31);
        assert!(id_text.starts_with("test_"));
    }

    /// Ensure an existing, hardcoded timestamp works for extraction
    #[pg_test]
    fn test_typeid_from_uuid() {
        let id = idkit_typeid_from_uuid_v7("test", &uuid::Uuid::now_v7().to_string());
        assert!(id.starts_with("test_"));
        assert!(typeid_from_uuid_v7("test", &uuid::Uuid::new_v4().to_string()).is_err());
    }

    /// Ensure timestamps extracted from generated typeids are valid
    #[pg_test]
    fn test_typeid_extract_timestamptz() {
        let id = idkit_typeid_generate("test");
        assert!(id.starts_with("test"));
        let timestamp = idkit_typeid_extract_timestamptz(id);
        let parsed: DateTime<Utc> = DateTime::parse_from_rfc3339(&timestamp.to_iso_string())
            .expect("extracted timestamp as ISO string parsed to UTC DateTime")
            .into();
        assert_eq!(
            Utc::now().signed_duration_since(parsed).num_seconds(),
            0,
            "extracted, printed & re-parsed typeid timestamp is from recent past (within 1s)"
        );
    }

    /// Ensure an existing, hardcoded timestamp works for extraction
    #[pg_test]
    fn test_typeid_extract_timestamptz_existing() {
        idkit_typeid_extract_timestamptz("test_01k0gryxwkep0tcf8r1sc6ydv9".into());
    }
}
