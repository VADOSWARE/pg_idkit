## Local Development

Here's how to get started working on `pg_idkit` locally.

### Prerequisites

To work on `pg_idkit`, you'll need the following:

- [Rust][rust] toolchain (via [`rustup`][rustup])
- [`cargo-pgrx`][cargo-pgrx] (see below for more details)
- [`cargo-get`][cargo-get]
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

The PGRX project is the magic behind this extension, exposing an easy-to-program interface in Rust for Postgres extensions.

To install it:

```console
cargo install --locked cargo-pgrx@0.11.0
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
