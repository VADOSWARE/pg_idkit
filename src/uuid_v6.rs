use pgrx::*;
use uuidv6::{self, Node, RawUUIDv6, UUIDv6};

/// Generate a UUID v6
#[pg_extern]
fn idkit_uuidv6_generate() -> String {
    let node = Node::new();
    UUIDv6::new(&node).create()
}

/// Generate a UUID v6, producing a Postgres text object
#[pg_extern]
fn idkit_uuidv6_generate_text() -> String {
    idkit_uuidv6_generate()
}

/// Generate a UUID v6, producing a Postgres uuid object
#[pg_extern]
fn idkit_uuidv6_generate_uuid() -> pgrx::Uuid {
    let node = Node::new();

    pgrx::Uuid::from_slice(&RawUUIDv6::new(&node).create())
        .unwrap_or_else(|e| error!("{}", format!("failed to generate/parse uuidv6: {}", e)))
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
    fn test_uuidv6_len() {
        let generated = crate::uuid_v6::idkit_uuidv6_generate();
        assert_eq!(generated.len(), 36);
    }

    /// Check version integer in UUID string
    #[pg_test]
    fn test_uuidv6_version_int() {
        let generated = crate::uuid_v6::idkit_uuidv6_generate();
        let c9 = generated.chars().nth(14);
        assert!(c9.is_some());
        assert_eq!(c9.unwrap(), '6');
    }

    /// Basic length test for bytes
    #[pg_test]
    fn test_uuidv6_len_uuid() {
        let generated = crate::uuid_v6::idkit_uuidv6_generate_uuid();
        assert_eq!(generated.len(), 16);
    }
}
