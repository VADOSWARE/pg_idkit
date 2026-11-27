ARG POSTGRES_IMAGE=postgres:19.0-alpine3.24@sha256:cac45fb483caee3668946088a1c0532db89d5285c28dbfeac889b4068296e06f
FROM $POSTGRES_IMAGE AS builder

RUN apk add --no-cache build-base clang-dev llvm-dev linux-headers openssl-dev pkgconf rustup
ENV PATH="/root/.cargo/bin:$PATH"
ENV RUSTFLAGS="-Ctarget-feature=-crt-static"
RUN rustup-init -y --profile minimal --default-toolchain 1.99.1 && \
    cargo install --locked cargo-pgrx --version 0.20.1

WORKDIR /pg_idkit
COPY . .
RUN cargo pgrx package --pg-config /usr/local/bin/pg_config --no-default-features --features pg19 --out-dir /stage

FROM $POSTGRES_IMAGE
COPY --from=builder /stage/usr/local/ /usr/local/

ARG PGIDKIT_VERSION
ARG PGIDKIT_REVISION

LABEL org.opencontainers.image.authors="Victor Adossi <vados@vadosware.io>"
LABEL org.opencontainers.image.description="A distribution of the base postgres image, with pg_idkit pre-installed."
LABEL org.opencontainers.image.documentation="https://github.com/VADOSWARE/pg_idkit#readme"
LABEL org.opencontainers.image.licenses="Apache-2.0"
LABEL org.opencontainers.image.revision=${PGIDKIT_REVISION}
LABEL org.opencontainers.image.source="https://github.com/VADOSWARE/pg_idkit"
LABEL org.opencontainers.image.title="Postgres + pg_idkit"
LABEL org.opencontainers.image.url="https://github.com/VADOSWARE/pg_idkit"
LABEL org.opencontainers.image.vendor="VADOSWARE"
LABEL org.opencontainers.image.version=v${PGIDKIT_VERSION}
