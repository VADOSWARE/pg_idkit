use cuid2;
use pgrx::*;

/// Generate a random cuid2 UUID
#[pg_extern]
fn idkit_cuid2_generate() -> String {
    cuid2::create_id()
}

/// Generate a random cuid UUID, producing a Postgres text object
#[pg_extern]
fn idkit_cuid2_generate_text() -> String {
    idkit_cuid2_generate()
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
    fn test_cuid2_len() {
        let generated = crate::cuid2::idkit_cuid2_generate();
        assert_eq!(generated.len(), 24);
    }
}