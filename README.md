<h1 align="center">
  🐘 🪪 `pg_idkit`
</h1>

```
postgres=# CREATE EXTENSION pg_idkit;
CREATE EXTENSION

postgres=# SELECT idkit_uuidv7_generate();
        idkit_uuidv7_generate
--------------------------------------
 018c106f-9304-79bb-b5be-4483b92b036c
```

## Description

`pg_idkit` is a [Postgres][postgres] extension for generating many popular types of identifiers:

| Methodology               | Function                                    | Crate                                | Description                                              |
|---------------------------|---------------------------------------------|--------------------------------------|----------------------------------------------------------|
| [UUID v6][uuidv6]         | `idkit_uuidv6_generate()`                   | [`uuidv6`][crate-uuidv6]             | UUID v6 ([RFC 4122][rfc-4122-update])                    |
|                           | `idkit_uuidv6_generate_uuid()`              |                                      |                                                          |
|                           | `idkit_uuidv6_extract_timestamptz(TEXT)`    |                                      |                                                          |
| [UUID v7][uuidv7]         | `idkit_uuidv7_generate()`                   | [`uuid7`][crate-uuid7]               | UUID v7 ([RFC 4122][rfc-4122-update])                    |
|                           | `idkit_uuidv7_generate_uuid()`              |                                      |                                                          |
|                           | `idkit_uuidv7_extract_timestamptz(TEXT)`    |                                      |                                                          |
| [nanoid][nanoid]          | `idkit_nanoid_generate()`                   | [`nanoid`][crate-nanoid]             | NanoID, developed by [Andrey Sitnik][github-ai]          |
|                           | `idkit_nanoid_custom_generate_text()`       | [`nanoid`][crate-nanoid]             | NanoID with a custom length and alphabet                 |
| [ksuid][ksuid]            | `idkit_ksuid_generate()`                    | [`svix-ksuid`][crate-svix-ksuid]     | Created by [Segment][segment]                            |
|                           | `idkit_ksuid_extract_timestamptz(TEXT)`     |                                      |                                                          |
|                           | `idkit_ksuidms_generate()`                  | [`svix-ksuid`][crate-svix-ksuid]     | Same as `ksuid` but with millisecond precision           |
|                           | `idkit_ksuidms_extract_timestamptz(TEXT)`   |                                      |                                                          |
| [ulid][ulid]              | `idkit_ulid_generate()`                     | [`ulid`][crate-ulid]                 | Unique, lexicographically sortable identifiers           |
|                           | `idkit_ulid_extract_timestamptz(TEXT)`      |                                      |                                                          |
| [Timeflake][timeflake]    | `idkit_timeflake_generate()`                | [`timeflake-rs`][crate-timeflake-rs] | Twitter's Snowflake + Instagram's ID + Firebase's PushID |
|                           | `idkit_timeflake_extract_timestamptz(TEXT)` |                                      |                                                          |
| [PushID][pushid]          | `idkit_pushid_generate()`                   | [`pushid`][crate-pushid]             | Google Firebase's PushID                                 |
| [xid][xid]                | `idkit_xid_generate()`                      | [`xid`][crate-xid]                   | XID                                                      |
|                           | `idkit_xid_extract_timestamptz(TEXT)`       |                                      |                                                          |
| [cuid][cuid] (deprecated) | `idkit_cuid_generate()`                     | [`cuid`][crate-cuid]                 | CUID                                                     |
|                           | `idkit_cuid_extract_timestamptz(TEXT)`      |                                      |                                                          |
| [cuid2][cuid2]            | `idkit_cuid2_generate()`                    | [`cuid2`][crate-cuid2]               | CUID2                                                    |

This Postgres extension is made possible thanks to [`pgrx`][pgrx].

[crate-uuidv6]: https://crates.io/crates/uuidv6
[crate-uuid7]: https://crates.io/crates/uuid7
[crate-nanoid]: https://crates.io/crates/nanoid
[crate-svix-ksuid]: https://crates.io/crates/svix-ksuid
[crate-svix-ksuid]: https://crates.io/crates/svix-ksuid
[crate-ulid]: https://crates.io/crates/ulid
[crate-timeflake-rs]: https://crates.io/crates/timeflake-rs
[crate-pushid]: https://crates.io/crates/pushid
[crate-xid]: https://crates.io/crates/xid
[crate-cuid]: https://crates.io/crates/cuid
[crate-cuid2]: https://crates.io/crates/cuid2

## Quickstart

You can try out `pg_idkit` incredibly quickly by using `docker`, and a previously [released package of `pg_idkit`][released-packages]:

```console
docker run \
    --rm \
    -e POSTGRES_PASSWORD=replace_this \
    -p 5432 \
    --name pg_idkit \
    ghcr.io/vadosware/pg_idkit:0.2.4-pg17.0-alpine3.20.3-amd64
```

> [!WARNING]
> Currently only amd64 (x86_64) images are present/supported (See [`pg_idkit` packages][released-packages]).
>
> Work to support more platforms is described in [issue #30](https://github.com/VADOSWARE/pg_idkit/issues/30)

Once the postgres server is running, open another shell and connect to the dockerized Postgres instance running on port `5432`:

```console
➜ docker exec -it pg_idkit psql -U postgres
psql (17.0)
Type "help" for help.

postgres=# CREATE EXTENSION pg_idkit;
CREATE EXTENSION

postgres=# SELECT idkit_uuidv7_generate();
        idkit_uuidv7_generate
--------------------------------------
 018c106f-9304-79bb-b5be-4483b92b036c
(1 row)
```

## Installing `pg_idkit`

<details>
<summary>📃 From Source</summary>

### Source install

To build `pg_idkit` from source, clone this repository and run the following:

```console
cargo install cargo-get cargo-pgrx just
just package
```

After running these commands you should see the following directory structure in `target/release/pg_idkit-pg16`:

```
target/release/pg_idkit-pg16
├── home
│   └── <user>
│       └── .pgrx
│           └── 17.0
│               └── pgrx-install
│                   ├── lib
│                   │   └── postgresql
│                   │       └── pg_idkit.so
│                   └── share
│                       └── postgresql
│                           └── extension
│                               ├── pg_idkit--0.2.4.sql
│                               └── pg_idkit.control
└── usr
    ├── lib
    │   └── postgresql
    │       └── pg_idkit.so
    └── share
        └── postgresql
            └── extension
                └── pg_idkit.control

24 directories, 8 files
```

As the installation of the extension into a specific version of postgres uses your local installation of pgrx-managed Postgres by default (normally at `$HOME/.pgrx`), `cargo pgrx package` reproduces the directory structure in `target/release`. You can safely ignore the shorter `usr/lib`/`user/share` tree.

In the example above, the [files you need for a Postgres extension][pg-ext-files] are:

- `target/release/home/<user>/.pgrx/17.0/pgrx-install/lib/postgresql/pg_idkit.so`
- `target/release/home/<user>/.pgrx/17.0/pgrx-install/share/postgresql/extension/pg_idkit--0.2.4.sql`
- `target/release/home/<user>/.pgrx/17.0/pgrx-install/share/postgresql/extension/pg_idkit.control`

Install these files in the relevant folders for your Postgres installation -- note that exactly where these files should go can can differ across linux distributions and containerized environments.

</details>

<details>
<summary>💽 From Binary</summary>

### Binary install

If running a custom version of locally/globally manually installed Postgres, you may download (and verify the checksum of) a shared library version from the [releases](/releases), and add it as one of your `shared_preload_libraries` in `postgresql.conf`.

Assuming you have downloaded the `pg_idkit-vX.X.X.so` file to `/etc/postgresql/extensions`, you might change the file like this:

`postgresql.conf`
```
shared_preload_libraries = '/etc/postgresql/extensions/pg_idkit-vX.X.X.so'
```

Once your postgres instance is started up, you should be able to `CREATE EXTENSION`:

```
postgres=# CREATE EXTENSION pg_idkit;
CREATE EXTENSION
postgres=# SELECT idkit_uuidv7_generate();
        idkit_uuidv7_generate
--------------------------------------
 018c106f-9304-79bb-b5be-4483b92b036c
```

</details>

<details>
<summary>🐳 Dockerfile</summary>

### Dockerfile

To use `pg_idkit` easily from a containerized environment, you can use the `pg_idkit` image, built from [`postgres`][docker-postgres]:

```console
docker run \
    --rm \
    -e POSTGRES_PASSWORD=replace_this \
    -p 5432 \
    --name pg_idkit \
    ghcr.io/vadosware/pg_idkit:0.2.4-pg17.0-alpine3.18-amd64
```

From another terminal, you can exec into the `pg_idkit` container and enable `pg_idkit`:

```console
➜ docker exec -it pg_idkit psql -U postgres
psql (17.0)
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
> Currently only amd64 (x86_64) images are present/supported (See [`pg_idkit` packages][released-packages]).
>
> Work to support more platforms is described in [issue #30](https://github.com/VADOSWARE/pg_idkit/issues/30)

[docker-postgres]: https://hub.docker.com/_/postgres

</details>

<details>
<summary>📦 Debian (RPM)</summary>

### RPM install

RPMs are produced upon [every official release](/releases) of `pg_idkit`.

Grab a released version of the RPM (or build one yourself by running `just build-rpm` after [setting up local development][guide-localdev]).

For example, with an RPM named `pg_idkit-0.2.4-pg17.x86_64.rpm`, you should be able to run:

```
dnf install pg_idkit-0.2.4-pg17.x86_64.rpm
```

</details>

## Prior Art

There are some other projects in the Postgres ecosystem that implement alternative UUID generation mechanisms.

Here are some you may or may not have heard of:

- [spa5k/uids-postgres](https://github.com/spa5k/uids-postgres)
- [`scoville/pgsql-ulid`](https://github.com/scoville/pgsql-ulid)
- [`pg-xid`](https://github.com/modfin/pg-xid)
- [`geckoboard/pgulid`](https://github.com/geckoboard/pgulid)
- [this gist by `fabiolimace` for generating UUIDv6](https://gist.github.com/fabiolimace/515a0440e3e40efeb234e12644a6a346)

## Setting up for local development

Interested in contributing on the project? Set up your local development environment w/ [`docs/local-development.md`][guide-localdev].

## Contributing

Contributions are welcome!

If you find a bug or an impovement that should be included in `pg_idkit`, [create an issue](https://github.com/vadosware/pg_idkit/issues).

If you'd like to contribute code, get started by:

1. Reading the [local development guide][guide-localdev]
2. Creating an issue (if necessary) to explain the new feature/bugfix/etc
3. Forking this repository
4. Creating a feature/bugfix/etc branch  (we expect [conventional commits][conventional-commits], i.e. `feat: new awesome feature`)
5. Opening a Pull Request to this repository

[a-chilton]: https://github.com/chilts
[a-feerasta]: https://github.com/alizain
[cargo-get]: https://crates.io/crates/cargo-get
[cargo-pgrx]: https://crates.io/crates/cargo-pgrx
[cargo-watch]: https://github.com/passcod/cargo-watch
[cuid2]: https://github.com/paralleldrive/cuid2
[cuid]: https://github.com/paralleldrive/cuid
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
[pg-ext-files]: https://www.postgresql.org/docs/current/extend-extensions.html#EXTEND-EXTENSIONS-FILES
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
[released-packages]: https://github.com/VADOSWARE/pg_idkit/pkgs/container/pg_idkit
[guide-localdev]: ./docs/local-development.md
