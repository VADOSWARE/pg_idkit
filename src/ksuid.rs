use pgx::*;
use ksuid::Ksuid;

/// Generate a random KSUID (HEX-encoded)
#[pg_extern]
fn idkit_ksuid_generate() -> String {
    Ksuid::generate().to_hex()
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
    fn test_ksuid_len() {
        let generated = crate::ksuid::idkit_ksuid_generate();
        assert_eq!(generated.len(), 40);
    }
}
