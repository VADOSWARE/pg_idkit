FROM postgres:15.5-alpine3.18@sha256:a57387207806d947c842f1be9f358e37b05442bf8b5ed19b1a69af281be930e7 AS builder

ARG RUST_TOOLCHAIN_VERSION=1.74
ARG PGRX_PG_VERSION=pg15

# Install OS deps
RUN apk add --no-cache \
    alpine-sdk \
    clang \
    clang-dev \
    clang-libs \
    musl-dev \
    openssl-dev \
    rustup

# Install Rust & related deps
RUN rustup-init -y --profile minimal --default-toolchain $RUST_TOOLCHAIN_VERSION \
    && source "$HOME/.cargo/env" \
    && cargo install cargo-binstall
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo binstall -y just cargo-get

# Install pgrx
# (disabling the static C runtime is required since pgrx requires dynamic linking w/ libssl and libcrypto)
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo install cargo-pgrx

# Copy in pg_idkit code
WORKDIR /pg_idkit
COPY . .

# Perform the build and packaging of pg_idkit
ENV PGRX_IGNORE_RUST_VERSIONS=y
ENV PKG_PG_CONFIG_PATH=/usr/local/bin/pg_config
RUN cargo pgrx init --pg15=$PKG_PG_CONFIG_PATH
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" just build package
