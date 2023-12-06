mod common;

mod cuid;
mod cuid2;
mod ksuid;
mod ksuid_ms;
mod nanoid;
mod pushid;
mod timeflake;
mod ulid;
mod uuid_v6;
mod uuid_v7;
mod xid;

use pgrx::pg_module_magic;

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
