use pgx::*;
use ::xid::{new as generate_xid};

/// Generate a random xid UUID
#[pg_extern]
fn idkit_xid_generate() -> String {
    generate_xid().to_string()
}

/// Generate a random xid UUID, producing a Postgres text object
#[pg_extern]
fn idkit_xid_generate_text() -> String {
    idkit_xid_generate()
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
    fn test_xid_len() {
        let generated = crate::xid::idkit_xid_generate();
        assert_eq!(generated.len(), 20);
    }
}
