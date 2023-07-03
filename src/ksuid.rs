use ksuid::Ksuid;
use pgrx::*;

/// Generate a random KSUID (HEX-encoded)
#[pg_extern]
fn idkit_ksuid_generate() -> String {
    Ksuid::generate().to_hex()
}

/// Generate a random KSUID, producing a Postgres text object (HEX-encoded)
#[pg_extern]
fn idkit_ksuid_generate_text() -> String {
    idkit_ksuid_generate()
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::*;

    /// Basic length test
    #[pg_test]
    fn test_ksuid_len() {
        let generated = crate::ksuid::idkit_ksuid_generate();
        assert_eq!(generated.len(), 40);
    }
}
