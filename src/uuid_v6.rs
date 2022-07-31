use pgx::*;
// use svix_ksuid::{KsuidLike, KsuidMs};

#[pg_extern]
fn hello_pg_idkit() -> &'static str {
    "Hello, pg_idkit"
}

// #[pg_extern]
// pub(crate) fn generate_ksuid() -> String {
//     let ksuid_string = KsuidMs::new(None, None);
//     ksuid_string.to_string()
// }

// #[pg_extern]
// pub(crate) fn generate_ksuid_bytes() -> Vec<u8> {
//     let ksuid_bytes = KsuidMs::new(None, None);
//     ksuid_bytes.bytes().to_vec()
// }

//////////
// Test //
//////////

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;
    use crate::uuid_v6::pg_test;

    #[pg_test]
    fn test_hello_pg_idkit() {
        assert_eq!("Hello, pg_idkit", crate::uuid_v6::hello_pg_idkit());
    }

}
