use pgx::*;
use sonyflake::Sonyflake;

/// Generate a random sonyflake UUID (hex encoded)
#[pg_extern]
fn idkit_sonyflake_generate() -> String {
    let generated = Sonyflake::new();
    if let Err(e) = generated {
        error!("{}", format!("failed to generate sonyflake: {}", e));
    }

    let next_id = generated.unwrap().next_id();
    if let Err(e) = next_id {
        error!("{}", format!("failed to generate sonyflake: {}", e));
    }

    format!("{:x}", next_id.unwrap())
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
    fn test_sonyflake_len() {
        let generated = crate::sonyflake::idkit_sonyflake_generate();
        assert_eq!(generated.len(), 15);
    }
}
