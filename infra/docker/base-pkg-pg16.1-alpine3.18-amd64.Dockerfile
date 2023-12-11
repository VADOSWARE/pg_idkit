FROM postgres:16.1-alpine3.18@sha256:b788d196db04847b17df664f4ae18062e712328d225b9ff75d4d7cd91a16c374 AS builder

# Allow for overriding rust toolcahin version
ARG RUST_TOOLCHAIN_VERSION=1.74
ENV RUST_TOOLCHAIN_VERSION=$RUST_TOOLCHAIN_VERSION

# Allow for overriding of PGRX PG version that is used
ARG PGRX_PG_VERSION=pg16
ENV PGRX_PG_VERSION=$PGRX_PG_VERSION

# Allow overriding features so that this file can be used to build
# different crate features. By default since this is a 16.1 base package
# we expect to build with crate feature 'pg16'
ARG CARGO_FEATURES=pg16
ENV CARGO_FEATURES=$CARGO_FEATURES

ARG USER
ENV USER=$USER

# Install OS deps
RUN apk add --no-cache \
    alpine-sdk \
    coreutils \
    clang \
    clang-dev \
    clang-libs \
    musl-dev \
    openssl-dev \
    rustup

# Install Rust & related deps
RUN rustup-init -y --profile minimal --default-toolchain $RUST_TOOLCHAIN_VERSION
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install just cargo-get

# Install pgrx
# (disabling the static C runtime is required since pgrx requires dynamic linking w/ libssl and libcrypto)
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo install --locked cargo-pgrx@0.11.2

# Copy in pg_idkit code
WORKDIR /pg_idkit
COPY . .

# Initialize pgrx
ENV PGRX_IGNORE_RUST_VERSIONS=y
ENV PKG_PG_CONFIG_PATH=/usr/local/bin/pg_config
RUN cargo pgrx init --pg16=$PKG_PG_CONFIG_PATH

# Build the package
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" just build package
