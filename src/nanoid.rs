use std::convert::TryFrom;

use nanoid::nanoid;
use pgrx::*;

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

/// Generate a nanoid, using a configurable alphabet, producing a Postgres text object.
/// Non-positive lengths produce an empty string.
#[pg_extern]
fn idkit_nanoid_custom_generate_text(len: i64, alphabet: String) -> String {
    let len = match usize::try_from(std::cmp::max(len, 0)) {
        Ok(v) => v,
        Err(e) => {
            pgrx::error!("invalid length, cannot convert to platform-specific quantity: {e}")
        }
    };
    // nanoid 0.4 does not terminate when asked to generate an empty ID.
    if len == 0 {
        return String::new();
    }
    let alphabet = alphabet.chars().collect::<Vec<char>>();
    nanoid!(len, &alphabet)
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

    #[pg_test]
    /// Test a custom alphabet
    fn test_nanoid_custom_alphabet() {
        let generated = crate::nanoid::idkit_nanoid_custom_generate_text(21, "abc".into());
        assert_eq!(generated.len(), 21);
        assert!(generated.chars().all(|c| ('a'..='c').contains(&c)));
    }

    #[pg_test]
    /// Test a custom len
    fn test_nanoid_custom_len() {
        let generated = crate::nanoid::idkit_nanoid_custom_generate_text(10, "abc".into());
        assert_eq!(generated.len(), 10);
        assert!(generated.chars().all(|c| ('a'..='c').contains(&c)));
    }

    #[pg_test]
    fn test_nanoid_zero_len() {
        assert_eq!(
            crate::nanoid::idkit_nanoid_custom_generate_text(0, "abc".into()),
            ""
        );
    }

    #[pg_test]
    fn test_nanoid_negative_len() {
        for len in [-1, i64::MIN] {
            assert_eq!(
                crate::nanoid::idkit_nanoid_custom_generate_text(len, "abc".into()),
                ""
            );
        }
    }
}
