# `pg_idkit` - a tool kit for generating IDs in postgres

`pg_idkit` is a [Postgres][postgres] extension for generating IDs. It aims to be have just about every ID you'd normally think of using:

| Methodology               | function                   | Crate                                                   | Description                                              |
|---------------------------|----------------------------|---------------------------------------------------------|----------------------------------------------------------|
| [UUID v6][uuidv6]         | `idkit_uuidv6_generate`    | [`uuidv6`](https://crates.io/crates/uuidv6)             | UUID v6 ([RFC 4122][rfc-4122-update])                    |
| [UUID v7][uuidv7]         | `idkit_uuidv7_generate`    | [`uuid7`](https://crates.io/crates/uuid7)               | UUID v7 ([RFC 4122][rfc-4122-update])                    |
| [nanoid][nanoid]          | `idkit_nanoid_generate`    | [`nanoid`](https://crates.io/crates/nanoid)             | NanoID, developed by [Andrey Sitnik][github-ai]          |
| [ksuid][ksuid]            | `idkit_ksuid_generate`     | [`ksuid`](https://crates.io/crates/ksuid)               | developed by [Segment][segment]                          |
| [ulid][ulid]              | `idkit_ulid_generate`      | [`ulid`](https://crates.io/crates/ulid)                 | unique, lexicographically sortable identifiers           |
| [Timeflake][timeflake]    | `idkit_timeflake_generate` | [`timeflake-rs`](https://crates.io/crates/timeflake-rs) | Twitter's Snowflake + Instagram's ID + Firebase's PushID |
| [PushID][pushid]          | `idkit_pushid_generate`    | [`pushid`](https://crates.io/crates/pushid)             | Google Firebase's PushID                                 |
| [xid][xid]                | `idkit_xid_generate`       | [`xid`](https://crates.io/crates/xid)                   | XID                                                      |
| [cuid][cuid] (deprecated) | `idkit_cuid_generate`      | [`cuid`](https://crates.io/crates/cuid)                 | CUID                                                     |
| [cuid2][cuid2]            | `idkit_cuid2_generate`     | [`cuid2`](https://crates.io/crates/cuid2)               | CUID                                                     |

This Postgres extension is made possible thanks to [`pgrx`][pgrx].

## Prior Art

There are some other projects in the Postgres ecosystem that implement alternative UUID generation mechanisms.

Here are some you may or may not have heard of:

- [spa5k/uids-postgres](https://github.com/spa5k/uids-postgres)
- [`scoville/pgsql-ulid`](https://github.com/scoville/pgsql-ulid)
- [`pg-xid`](https://github.com/modfin/pg-xid)
- [`geckoboard/pgulid`](https://github.com/geckoboard/pgulid)
- [this gist by `fabiolimace` for generating UUIDv6](https://gist.github.com/fabiolimace/515a0440e3e40efeb234e12644a6a346)

## Installing `pg_idkit`

### Binary install

If running a custom version of locally/globally manually installed Postgres, you may download (and verify the checksum of) a shared library version from the [releases](/releases), and add it as one of your `shared_preload_libraries` in `postgresql.conf`.

Assuming you have downloaded the `pg_idkit-vX.X.X.so` file to `/etc/postgresql/extensions`, you might change the file like this:

`postgresql.conf`
```
shared_preload_libraries = '/etc/postgresql/extensions/pg_idkit-vX.X.X.so'
```

### Dockerfile

To use `pg_idkit` easily from a containerized environment, you can use the `pg_idkit` image, built from [`postgres`][docker-postgres]:

```console
docker run \
    --rm \
    -e POSTGRES_PASSWORD=replace_this \
    -p 5432 \
    --name pg_idkit \
    ghcr.io/vadosware/pg_idkit:0.1.0-pg15.5-alpine3.18-amd64
```

From another terminal, you can exec into the `pg_idkit` container and enable `pg_idkit`:

```console
➜ docker exec -it pg_idkit psql -U postgres
psql (15.5)
Type "help" for help.

postgres=# CREATE EXTENSION pg_idkit;
CREATE EXTENSION
postgres=# SELECT idkit_uuidv7_generate();
        idkit_uuidv7_generate
--------------------------------------
 018c106f-9304-79bb-b5be-4483b92b036c
(1 row)
```

> [!WARNING]
> Currently only amd64 (x86_64) images are present/supported (See [`pg_idkit` packages](https://github.com/VADOSWARE/pg_idkit/pkgs/container/pg_idkit)).
>
> Work to support more platforms is described in [issue #30](https://github.com/VADOSWARE/pg_idkit/issues/30)

[docker-postgres]: https://hub.docker.com/_/postgres

## Local Development

Here's how to get started working on `pg_idkit` locally.

### Prerequisites

To work on `pg_idkit`, you'll need the following:

- [Rust][rust] toolchain (via [`rustup`][rustup])
- [`cargo-pgrx`][cargo-pgrx] (see below for more details)
- [`just`][just] (see below for more details)
- (optional) [`git-crypt`][git-crypt] (for working with secrets)
- (optional) [direnv][direnv]
- (optional) [Docker][docker]
- (optional) [`cargo-watch`][cargo-watch]

#### `just`

This project uses a task runner similar to `make` called [`just`][just]. You can install it easily with `cargo`:

```console
cargo install --locked just
```

#### `cargo-pgrx`

Installing

```console
cargo install --locked cargo-pgrx
cargo pgrx init
```

## Building the project

To build the project:

```console
just build
```

To run the build continuously for quicker local development (assuming you have `cargo-watch` installed):

```console
just build-watch
```

### `pgrx` workflow

Note that you can use the `pgrx`-documented development flow as well (using `cargo pgrx`) as well, for example:

```console
cargo pgrx run pg15
```

## Run tests

To run the tests:

```console
just test
```

To run tests continuously for quicker local development (requires `cargo-watch`):

```console
just build-test-watch
```

## Starting a local Postgres instance with `pg_idkit` installed

Assuming you have Docker installed, to start a local Postgres instance first you must build a `postgres` docker image with `pg_idkit`:

```console
just image
```

Then start the container:

```console
just db-local
```

You may attach to the local DB with `psql` and execute commands:

```console
just db-local-psql
```

# Continuous Integration

To push up images that are used from continuous integration:

1. Get a personal access token from Github
2. Ensuring `DOCKER_LOGIN` is set (see instructions above)
3. Perform a login
   1. Manually via `echo $GH_PAT | docker login ghcr.io -u <username> --password-stdin`
   2. Automatically, via `just docker-login` which will use the `git-crypt` protected credentials (you must have run `git-crypt` unlock first)
4. Observe the docker login credentials generated in this local repo directory (`secrets/docker/config.json`)
5. Run `just build-ci-image push-ci-image`

# Packaging

## Setting up for Docker usage

Assuming you are using something like [`direnv`][direnv], use the following `.envrc` file:

```
# Use local docker auth file
export DOCKER_CONFIG=$(realpath secrets/docker)
```

**NOTE**, that is *not* a `.env` file, it is a `.envrc` file, with separate semantics

[a-chilton]: https://github.com/chilts
[a-feerasta]: https://github.com/alizain
[cargo-pgrx]: https://crates.io/crates/cargo-pgrx
[cargo-watch]: https://github.com/passcod/cargo-watch
[cuid2]: https://github.com/paralleldrive/cuid2
[cuid]: https://github.com/paralleldrive/cuid
[direnv]: https://direnv.net
[direnv]: https://direnv.net
[docker]: https://docs.docker.com/get-started/overview/
[e-elliott]: https://github.com/ericelliott
[git-crypt]: https://github.com/AGWA/git-crypt
[github-ai]: https://github.com/ai
[google]: https://google.com
[instagram]: instagram-engineering.com/
[it-cabrera]: https://darkghosthunter.medium.com/
[just]: https://github.com/casey/just
[ksuid]: https://github.com/segmentio/ksuid
[mongodb]: https://www.mongodb.com/blog/channel/engineering-blog
[nanoid]: https://www.npmjs.com/package/nanoid
[o-poitrey]: https://github.com/rs
[objectid]: https://www.mongodb.com/docs/manual/reference/method/ObjectId/
[oryx-pro]: https://system76.com/laptops/oryx
[p-pearcy]: https://github.com/ppearcy/elasticflake
[pg-docs-operator-classes]: https://www.postgresql.org/docs/current/indexes-opclass.html
[pgstattuple]: https://www.postgresql.org/docs/current/pgstattuple.html
[postgres]: https://postgresql.org
[pushid]: https://firebase.googleblog.com/2015/02/the-2120-ways-to-ensure-unique_68.html
[r-tallent]: https://github.com/richardtallent
[repo]: https://github.com/t3hmrman/pg_idkit
[rfc-4122-update]: https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04
[rust]: https://rust-lang.org
[rustup]: https://rust-lang.github.io/rustup
[segment]: https://segment.com/blog/engineering/
[sony]: https://github.com/sony
[t-pawlak]: https://github.com/T-PWK
[timeflake]: https://github.com/anthonynsimon/timeflake
[twitter-snowflake]: https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake
[twitter]: https://blog.twitter.com/engineering
[ulid]: https://github.com/ulid/spec
[uuidv1]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_1_(date-time_and_MAC_address)
[uuidv6]: https://www.ietf.org/archive/id/draft-peabody-dispatch-new-uuid-format-01.html
[uuidv7]: https://www.ietf.org/archive/id/draft-peabody-dispatch-new-uuid-format-01.html
[wiki-gregorian]: https://en.wikipedia.org/wiki/Gregorian_calendar
[wiki-mac-address]: https://en.wikipedia.org/wiki/MAC_address
[wiki-uuid]: https://en.wikipedia.org/wiki/Universally_unique_identifier
[xid]: https://github.com/rs/xid
