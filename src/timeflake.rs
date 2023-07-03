use pgrx::*;
use timeflake_rs::Timeflake;

/// Generate a random timeflake UUID
#[pg_extern]
fn idkit_timeflake_generate() -> String {
    let generated = Timeflake::random();

    if let Err(e) = generated {
        error!("{}", format!("failed to generate timeflake: {}", e));
    }

    generated.unwrap().to_string()
}

/// Generate a random timeflake UUID, producing a Postgres text object
#[pg_extern]
fn idkit_timeflake_generate_text() -> String {
    idkit_timeflake_generate()
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
    fn test_timeflake_len() {
        let generated = crate::timeflake::idkit_timeflake_generate();
        assert_eq!(generated.len(), 36);
    }
}
