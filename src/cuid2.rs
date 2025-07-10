use pgrx::*;

/// Generate a random cuid2 UUID
#[pg_extern]
fn idkit_cuid2_generate() -> String {
    cuid2::create_id()
}

/// Generate a custom cuid2 UUID with specified length
#[pg_extern]
fn idkit_cuid2_generate_with_len(length: i32) -> Result<String, String> {
    if length < 2 {
        return Err("CUID2 length must be at least 2".to_string());
    }
    if length > u16::MAX as i32 {
        return Err(format!("CUID2 length must be at most {}", u16::MAX));
    }
    
    let constructor = cuid2::CuidConstructor::new().with_length(length as u16);
    
    // Note: Custom fingerprint support is not currently implemented because 
    // the cuid2 library expects function pointers for fingerprinters, not 
    // string values. This would require either:
    // 1. Implementing CUID2 generation logic directly to use custom fingerprints
    // 2. Using a different library that supports string-based fingerprints
    // 3. Creating a wrapper that modifies the generated ID based on the fingerprint
    
    Ok(constructor.create_id())
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

    /// Test custom length generation
    #[pg_test]
    fn test_cuid2_custom_length() {
        let generated = crate::cuid2::idkit_cuid2_generate_with_len(16).unwrap();
        assert_eq!(generated.len(), 16);
        
        let generated_32 = crate::cuid2::idkit_cuid2_generate_with_len(32).unwrap();
        assert_eq!(generated_32.len(), 32);
    }

    /// Test invalid length handling
    #[pg_test]
    fn test_cuid2_invalid_length() {
        let result = crate::cuid2::idkit_cuid2_generate_with_len(1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be at least 2"));
    }

    /// Test that generated IDs are valid CUID2s
    #[pg_test]
    fn test_cuid2_validity() {
        let generated = crate::cuid2::idkit_cuid2_generate();
        assert!(cuid2::is_cuid2(&generated));

        let generated_custom = crate::cuid2::idkit_cuid2_generate_with_len(16).unwrap();
        assert!(cuid2::is_cuid2(&generated_custom));
    }
}
