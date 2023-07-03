use pgrx::*;
use nanoid::nanoid;

/// Generate a nanoid
#[pg_extern]
fn idkit_nanoid_generate() -> String {
    nanoid!()
}

/// Generate a nanoid, producing a Postgres text object
#[pg_extern]
fn idkit_nanoid_generate_text() -> String {
    idkit_nanoid_generate()
}

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::*;

    #[pg_test]
    /// Basic length test
    fn test_nanoid_len() {
        let generated = crate::nanoid::idkit_nanoid_generate();
        assert_eq!(generated.len(), 21);
    }
}
