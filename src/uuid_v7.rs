use pgx::*;
use uuid7::uuid7;

/// Generate a UUID v7
#[pg_extern]
fn idkit_uuidv7_generate() -> String {
    uuid7().to_string()
}

/// Generate a UUID v7, producing a Postgres text object
#[pg_extern]
fn idkit_uuidv7_generate_text() -> String {
    idkit_uuidv7_generate()
}

/// Generate a UUID v7, producing a Postgres uuid object
#[pg_extern]
fn idkit_uuidv7_generate_uuid() -> pgx::Uuid {
    pgx::Uuid::from_slice(uuid7().as_bytes())
        .unwrap_or_else(|e| error!("{}", format!("failed to generate/parse uuidv7: {}", e)))
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    /// Basic length test
    #[pg_test]
    fn test_uuidv7_len() {
        let generated = crate::uuid_v7::idkit_uuidv7_generate();
        assert_eq!(generated.len(), 36);
    }

    /// Check version integer in UUID string
    #[pg_test]
    fn test_uuidv7_version_int() {
        let generated = crate::uuid_v7::idkit_uuidv7_generate();
        let c9 = generated.chars().nth(14);
        assert!(c9.is_some());
        assert_eq!(c9.unwrap(), '7');
    }
}
