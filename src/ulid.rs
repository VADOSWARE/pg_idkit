use pgx::*;
use ulid::Ulid;

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

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    /// Basic length test
    #[pg_test]
    fn test_ulid_len() {
        let generated = crate::ulid::idkit_ulid_generate();
        assert_eq!(generated.len(), 26);
    }
}
