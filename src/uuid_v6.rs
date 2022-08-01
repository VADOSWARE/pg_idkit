use pgx::*;
use uuidv6::{Node, UUIDv6};

#[pg_extern]
fn idkit_uuidv6_generate() -> String {
    let node = Node::new();
    UUIDv6::new(&node).create()
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
    fn test_uuidv6_len() {
        let generated = crate::uuid_v6::idkit_uuidv6_generate();
        assert_eq!(generated.len(), 36);
    }

    #[pg_test]
    /// Check version integer in UUID string
    fn test_uuidv6_version_int() {
        let generated = crate::uuid_v6::idkit_uuidv6_generate();
        let c9 = generated.chars().nth(14);
        assert!(c9.is_some());
        assert_eq!(c9.unwrap(), '6');
    }

}
