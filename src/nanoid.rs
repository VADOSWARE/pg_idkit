use pgx::*;
use nanoid::nanoid;

#[pg_extern]
fn idkit_nanoid_generate() -> String {
    nanoid!()
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
    fn test_nanoid_len() {
        let generated = crate::nanoid::idkit_nanoid_generate();
        assert_eq!(generated.len(), 21);
    }
}
