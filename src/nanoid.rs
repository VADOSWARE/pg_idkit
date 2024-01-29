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

/// Generate a nanoid, using a configurable alphabet, producing a Postgres text object
#[pg_extern]
fn idkit_nanoid_custom_generate_text(len: i64, alphabet: String) -> String {
    let len = match usize::try_from(std::cmp::max(len, 0)) {
        Ok(v) => v,
        Err(e) => {
            pgrx::error!("invalid length, cannot convert to platform-specific quantity: {e}")
        }
    };
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
}
