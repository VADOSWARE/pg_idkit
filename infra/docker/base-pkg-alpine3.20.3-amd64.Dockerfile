FROM alpine:3.20.3@sha256:beefdbd8a1da6d2915566fde36db9db0b524eb737fc57cd1367effd16dc0d06d AS builder

# Allow for overriding rust toolcahin version
ARG RUST_TOOLCHAIN_VERSION=1.81
ENV RUST_TOOLCHAIN_VERSION=$RUST_TOOLCHAIN_VERSION

# Allow for overriding of PGRX PG version that is used
ARG PGRX_PG_VERSION=pg16
ENV PGRX_PG_VERSION=$PGRX_PG_VERSION

# Allow overriding features so that this file can be used to build
# different crate features. By default since this is a 16.2 base package
# we expect to build with crate feature 'pg16'
ARG CARGO_FEATURES=pg16
ENV CARGO_FEATURES=$CARGO_FEATURES

# Install OS deps
RUN apk add --no-cache \
    alpine-sdk \
    bash \
    bison \
    clang \
    clang-dev \
    clang-libs \
    coreutils \
    flex \
    icu-dev \
    linux-headers \
    musl-dev \
    openssl-dev \
    perl \
    readline \
    readline-dev \
    rustup \
    zlib-dev

# Install Rust & related deps
RUN rustup-init -y --profile minimal --default-toolchain $RUST_TOOLCHAIN_VERSION
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install just cargo-get

# Install pgrx
# (disabling the static C runtime is required since pgrx requires dynamic linking w/ libssl and libcrypto)
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo install --locked cargo-pgrx@0.12.5

# Copy in pg_idkit code
WORKDIR /pg_idkit
COPY . .

# Initialize pgrx
ENV PGRX_IGNORE_RUST_VERSIONS=y
RUN just pgrx-init

# Build the package
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" just build package
