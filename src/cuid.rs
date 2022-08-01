use pgx::*;
use cuid;

/// Generate a random cuid UUID
#[pg_extern]
fn idkit_cuid_generate() -> String {
    let generated = cuid::cuid();
    if let Err(e) = generated {
        error!("{}", format!("failed to generate sonyflake: {}", e));
    }

    generated.unwrap()
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
    fn test_cuid_len() {
        let generated = crate::cuid::idkit_cuid_generate();
        assert_eq!(generated.len(), 25);
    }
}
