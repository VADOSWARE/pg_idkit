use pgx::*;
use uuid7::uuid7;

#[pg_extern]
fn idkit_uuidv7_generate() -> String {
    uuid7().to_string()
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    /// Basic length test
    fn test_uuidv7_len() {
        let generated = crate::uuid_v7::idkit_uuidv7_generate();
        assert_eq!(generated.len(), 36);
    }

    #[pg_test]
    /// Check version integer in UUID string
    fn test_uuidv7_version_int() {
        let generated = crate::uuid_v7::idkit_uuidv7_generate();
        let c9 = generated.chars().nth(14);
        assert!(c9.is_some());
        assert_eq!(c9.unwrap(), '7');
    }
}
