# `pg_idkit` - a tool kit for generating IDs in postgres

`pg_idkit` is a [Postgres][postgres] extension for generating IDs. It aims to be have just about every ID you'd normally think of using:

| Methodology                            | function                 | Crate                                                   | Description                                              |
|----------------------------------------|--------------------------|---------------------------------------------------------|----------------------------------------------------------|
| [nanoid][nanoid]                       | `idkit_gen_nanoid`       | [`nanoid`](https://crates.io/crates/nanoid)             | NanoID, develepod by [Andrey Sitnik][github-ai]          |
| [ksuid][ksuid]                         | `idkit_gen_ksuid`        | [`ksuid`](https://crates.io/crates/ksuid)               | developed by [Segment][segment]                          |
| [ulid][ulid]                           | `idkit_gen_ulid`         | [`ulid`](https://crates.io/crates/ulid)                 | unique, lexicographically sortable identifiers           |
| [Twitter Snowflake][twitter-snowflake] | `idkit_gen_tw_snowflake` | [`rs-snowflake`](https://crates.io/crates/rs-snowflake) | Twitter Snowflake                                        |
| [Timeflake][twitter-snowflake]         | `idkit_gen_timeflake`    | [`timeflake-rs`](https://crates.io/crates/timeflake-rs) | Twitter's Snowflake + Instagram's ID + Firebase's PushID |
| [SonyFlake][sonyflake]                 | `idkit_gen_sonyflake`    | [`sonyflake-rs`](https://crates.io/crates/sonyflake-rs) | SonyFlake                                                |
| [PushID][pushid]                       | `idkit_gen_pushid`       | [`pushid`](https://crates.io/crates/pushid)             | Google Firebase's PushID                                 |
| [xid][xid]                             | `idkit_gen_xid`          | [`xid`](https://crates.io/crates/xid)                   | XID                                                      |
| [cuid][cuid]                           | `idkit_gen_cuid`         | [`cuid`](https://crates.io/crates/cuid)                 | CUID                                                     |
| [UUID v6][uuidv6]                      | `idkit_gen_uuidv6`       | [`uuidv6`](https://crates.io/crates/uuidv6)             | UUID v6 ([RFC 4122][rfc-4122-update])                    |
| [UUID v7][uuidv7]                      | `idkit_gen_uuidv7`       | [`uuid7`](https://crates.io/crates/uuid7)               | UUID v7 ([RFC 4122][rfc-4122-update])                    |

This Postgres extension is made possible thanks to [`pgx`][pgx].

# Installing `pg_idkit`

## Binary install

If running a custom version of locally/globally manually installed Postgres, you may download (and verify the checksum of) a shared library versionfrom the [releases](/releases), and add it as one of your `shared_preload_libraries` in `postgresql.conf`.

Assuming you have downloaded the `pg_idkit-vX.X.X.so` file to `/etc/postgresql/extensions`, you might change the file like this:

`postgresql.conf`
```
shared_preload_libraries = '/etc/postgresql/extensions/pg_idkit-vX.X.X.so'
```

## Dockerfile

To build `pg_idkit` into a Postgres instance you can use a `Dockerfile` like the following:

```dockerfile
TODO
```

# Local Development

Here's how to get started working on `pg_idkit` locally.

## Prerequisites

To work on `pg_idkit`, you'll need the following:

- [Rust][rust] toolchain ([`rustup`][rustup])
- (optional) [Docker][docker]
- [`pgx`][pgx] and it's toolchain (the rust subcommand)

## Building the project

To build the project:

```console
make build
```

## Run tests

To run the tests:

```console
make test
```

Note that you can use the `pgx`-documented development flow as well (using `cargo pgx`) as well!

## Starting a local Postgres instance with `pg_idkit` installed

Assuming you have Docker installed, to start a local Postgres instance first you must build a `postgres` docker image with `pg_idkit`:

```console
make image
```

Then start the container:

```console
make db-local
```

You may attach to the local DB with `psql` and execute commands:

```console
make db-local-psql
```

[pgx]: https://github.com/tcdi/pgx
[github-ai]: https://github.com/ai
[rfc-4122-update]: https://www.ietf.org/archive/id/draft-peabody-dispatch-new-uuid-format-01.html
