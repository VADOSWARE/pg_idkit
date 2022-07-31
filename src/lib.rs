use pgx::*;

pg_module_magic!();

#[pg_extern]
fn hello_pg_idkit() -> &'static str {
    "Hello, pg_idkit"
}

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
