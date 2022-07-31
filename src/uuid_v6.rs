pub mod uuid_v6 {
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

    #[cfg(any(test, feature = "pg_test"))]
    #[pg_schema]
    mod tests {
        use pgx::*;

        #[pg_test]
        fn test_hello_pg_idkit() {
            assert_eq!("Hello, pg_idkit", crate::hello_pg_idkit());
        }

    }

    #[cfg(test)]
    pub mod pg_test {
        pub fn setup(_options: Vec<&str>) {
            // perform one-off initialization when the pg_test framework starts
        }

        pub fn postgresql_conf_options() -> Vec<&'static str> {
            // return any postgresql.conf settings that are required for your tests
            vec![]
        }
    }

}
