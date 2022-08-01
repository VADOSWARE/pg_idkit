use pgx::*;
use pushid::PushId;

/// Generate a random pushid UUID (hex encoded)
#[pg_extern]
fn idkit_pushid_generate() -> String {
    PushId::new().get_id()
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
    fn test_pushid_len() {
        let generated = crate::pushid::idkit_pushid_generate();
        assert_eq!(generated.len(), 15);
    }
}
