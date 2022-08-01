mod uuid_v6;
mod uuid_v7;
mod nanoid;
mod ksuid;
mod ulid;
mod timeflake;
mod pushid;
mod xid;
mod cuid;

use pgx::*;

pg_module_magic!();

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
