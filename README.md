# `pg_idkit` - a tool kit for generating IDs in postgres

`pg_idkit` is a [Postgres][postgres] extension for generating IDs. It aims to be have just about every ID you'd normally think of using:

| Methodology            | function                   | Crate                                                   | Description                                              |
|------------------------|----------------------------|---------------------------------------------------------|----------------------------------------------------------|
| [UUID v6][uuidv6]      | `idkit_uuidv6_generate`    | [`uuidv6`](https://crates.io/crates/uuidv6)             | UUID v6 ([RFC 4122][rfc-4122-update])                    |
| [UUID v7][uuidv7]      | `idkit_uuidv7_generate`    | [`uuid7`](https://crates.io/crates/uuid7)               | UUID v7 ([RFC 4122][rfc-4122-update])                    |
| [nanoid][nanoid]       | `idkit_nanoid_generate`    | [`nanoid`](https://crates.io/crates/nanoid)             | NanoID, develepod by [Andrey Sitnik][github-ai]          |
| [ksuid][ksuid]         | `idkit_ksuid_generate`     | [`ksuid`](https://crates.io/crates/ksuid)               | developed by [Segment][segment]                          |
| [ulid][ulid]           | `idkit_ulid_generate`      | [`ulid`](https://crates.io/crates/ulid)                 | unique, lexicographically sortable identifiers           |
| [Timeflake][timeflake] | `idkit_timeflake_generate` | [`timeflake-rs`](https://crates.io/crates/timeflake-rs) | Twitter's Snowflake + Instagram's ID + Firebase's PushID |
| [PushID][pushid]       | `idkit_pushid_generate`    | [`pushid`](https://crates.io/crates/pushid)             | Google Firebase's PushID                                 |
| [xid][xid]             | `idkit_xid_generate`       | [`xid`](https://crates.io/crates/xid)                   | XID                                                      |
| [cuid][cuid]           | `idkit_cuid_generate`      | [`cuid`](https://crates.io/crates/cuid)                 | CUID                                                     |

This Postgres extension is made possible thanks to [`pgx`][pgx].

## Prior Art

There are some other projects in the Postgres ecosystem that implement alternative UUID generation mechanisms. 

Here are some you may or may not have heard of:

- [spa5k/uids-postgres](https://github.com/spa5k/uids-postgres)
- [`scoville/pgsql-ulid`](https://github.com/scoville/pgsql-ulid)
- [`pg-xid`](https://github.com/modfin/pg-xid)
- [`geckoboard/pgulid`](https://github.com/geckoboard/pgulid)
- [this gist by `fabiolimace` for generating UUIDv6](https://gist.github.com/fabiolimace/515a0440e3e40efeb234e12644a6a346)

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
- [`pgx`][pgx] and it's toolchain (the rust subcommand)
- (optional) [Docker][docker]
- (optional) [`cargo-watch`][cargo-watch]

## Setting up local environment

Assuming you are using something like [`direnv`][direnv], use the following `.envrc` file: 

```
# Use local docker auth file
export DOCKER_CONFIG=$(realpath infra/docker)
```

**NOTE**, that is *not* a `.env` file, it is a `.envrc` file, with separate semantics

[direnv]: https://direnv.net

## Building the project

To build the project:

```console
make build
```

To run the build continuously for quicker local development (assuming you have `cargo-watch` installed):

```console
make build-watch
```

### `pgx` workflow

Note that you can use the `pgx`-documented development flow as well (using `cargo pgx`) as well, for example:

```console
cargo pgx run pg14
```

## Run tests

To run the tests:

```console
make test
```

To run tests continuously for quicker local development (requires `cargo-watch`):

```console
make build-test-watch
```

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

## Continuous Integration

To push up images that are used from continuous integration:

1. Get a personal access token from Github
2. Ensuring `DOCKER_LOGIN` is set (see instructions above), perform a login (`echo $GH_PAT | docker login ghcr.io -u <username> --password-stdin`)
3. Observe the docker login credentials generated in this local repo directory (`infra/docker/config.json`)
4. Run `make build-ci-image push-ci-image`

[pgx]: https://github.com/tcdi/pgx
[github-ai]: https://github.com/ai
[rfc-4122-update]: https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04
[cargo-watch]: https://github.com/passcod/cargo-watch
[uuidv1]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_1_(date-time_and_MAC_address)
[ksuid]: https://github.com/segmentio/ksuid
[ulid]: https://github.com/ulid/spec
[pushid]: https://firebase.googleblog.com/2015/02/the-2120-ways-to-ensure-unique_68.html
[cuid]: https://github.com/ericelliott/cuid
[xid]: https://github.com/rs/xid
[objectid]: https://www.mongodb.com/docs/manual/reference/method/ObjectId/
[nanoid]: https://www.npmjs.com/package/nanoid
[wiki-uuid]: https://en.wikipedia.org/wiki/Universally_unique_identifier
[twitter]: https://blog.twitter.com/engineering
[wiki-mac-address]: https://en.wikipedia.org/wiki/MAC_address
[instagram]: instagram-engineering.com/
[p-pearcy]: https://github.com/ppearcy/elasticflake
[segment]: https://segment.com/blog/engineering/
[r-tallent]: https://github.com/richardtallent
[a-chilton]: https://github.com/chilts
[it-cabrera]: https://darkghosthunter.medium.com/
[sony]: https://github.com/sony
[t-pawlak]: https://github.com/T-PWK
[a-feerasta]: https://github.com/alizain
[google]: https://google.com
[o-poitrey]: https://github.com/rs
[mongodb]: https://www.mongodb.com/blog/channel/engineering-blog
[e-elliott]: https://github.com/ericelliott
[wiki-gregorian]: https://en.wikipedia.org/wiki/Gregorian_calendar
[rust]: https://rust-lang.org
[pgx]: https://github.com/tcdi/pgx
[pg-docs-operator-classes]: https://www.postgresql.org/docs/current/indexes-opclass.html
[repo]: https://github.com/t3hmrman/pg_idkit
[oryx-pro]: https://system76.com/laptops/oryx
[pgstattuple]: https://www.postgresql.org/docs/current/pgstattuple.html
[uuidv6]: https://www.ietf.org/archive/id/draft-peabody-dispatch-new-uuid-format-01.html
[uuidv7]: https://www.ietf.org/archive/id/draft-peabody-dispatch-new-uuid-format-01.html
[twitter-snowflake]: https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake
[timeflake]: https://github.com/anthonynsimon/timeflake
[postgres]: https://postgresql.org
[rustup]: https://rust-lang.github.io/rustup
[docker]: https://docs.docker.com/get-started/overview/
