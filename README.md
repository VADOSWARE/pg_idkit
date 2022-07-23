# `pg_idkit` - a tool kit for generating IDs in postgres

`pg_idkit` is a [Postgres][postgres] extension for generating IDs. It aims to be have just about every ID you'd normally think of using:

| Methodology      | function                             | Description                                    |
|------------------|--------------------------------------|------------------------------------------------|
| [UUIDv1][uuidv1] | `gen_uuid_v1`/`gen_uuid_v1_with_mac` | time-based, with random/non-random hmac        |
| [UUIDv4][uuidv4] | `gen_uuid_v4`                        |                                                |
| [UUIDv6][uuidv6] | `gen_uuid_v6`                        | time ordered, with sort-friendly byte ordering |
| [UUIDv7][uuidv7] | `gen_uuid_v7`                        | time ordered, with sort-friendly byte ordering |
| [UUIDv8][uuidv8] | `gen_uuid_v8`                        | time ordered, with sort-friendly byte ordering |
| [ksuid][ksuid]   | `gen_ksuid`                          | developed by [Segment][segment]                |
| [ulid][ulid]     | `gen_ulid`                           | unique, lexicographically sortable identifiers |

This Postgres extension is made possible by [`pgx`][pgx].

# How to install `pg_idkit`

## PostgreSQL Extension Network (PGXN)

`pg_idkit` can be retrieved from the PostgreSQL Extension Network:

```console
TODO
```

## Dockerfile

To build PGXN into a Postgres instance you can use a `Dockerfile` like the following:

```dockerfile
TODO
```

## Custom build

If running a custom version of locally/globally manually installed Postgres, you may download (and verify the checksum of) a shared library versionfrom the [releases](/releases), and add it as one of your `shared_preload_libraries` in `postgresql.conf`.

Assuming you have downloaded the `pg_idkit-vX.X.X.so` file to `/etc/postgresql/extensions`, you might change the file like this:

`postgresql.conf`
```
shared_preload_libraries = '/etc/postgresql/extensions/pg_idkit-vX.X.X.so'
```

# Local Development

Here's how to get started working on `pg_idkit` locally.

## Prerequisites

To work on `pg_idkit`, you'll need the following:

- [Rust][rust] toolchain ([`rustup`][rustup])
- (optional) [Docker][docker]

## Building the project

To build the project:

```console
make build
```

## Starting a local Postgres instance with `pg_idkit` installed

Assuming you have Docker installed, to start a local Postgres instance with `pg_idkit`:

```console
make db-local
```

You may attach to the local DB with `psql`:

```console
make db-local-psql
```

[pgx]: https://github.com/tcdi/pgx
