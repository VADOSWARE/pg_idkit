#
# NOTE: you must have the base packaging layer built for this image to work
# you can build this from scratch with `just build-base-pkg-image`
#
# `cargo pgrx init` is run in the base-pkg, so it contains the versions of pg that will be used.
#
FROM ghcr.io/vadosware/pg_idkit/base-pkg:0.1.x-alpine3.21.3-amd64 AS builder

ARG USER
ENV USER=$USER

ARG PGRX_PG_VERSION=pg15
ENV PGRX_PG_VERSION=$PGRX_PG_VERSION

ARG PKG_PG_VERSION=15.13
ENV PKG_PG_VERSION=$PKG_PG_VERSION

ENV PKG_TARBALL_SUFFIX="-musl"

# Re-run the build with the latest code
WORKDIR /pg_idkit
COPY . .
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" just build package

FROM postgres:15.13-alpine3.21@sha256:f7de0e2497b9a3b027d41377606f94bb0140a034ed303f6de690aa77637bfbc9

ARG PGRX_PG_VERSION=pg15
ARG PGIDKIT_VERSION
ARG PGIDKIT_REVISION

# Install packaged pg_idkit for system postgres
COPY --from=builder /pg_idkit/pg_idkit-*-musl.tar.gz /tmp
RUN tar -C /usr/local --strip-components=1 -xvf /tmp/pg_idkit-*-musl.tar.gz

LABEL org.opencontainers.image.authors="Victor Adossi <vados@vadosware.io>"
LABEL org.opencontainers.image.description="A distribution of the base postgres image, with pg_idkit pre-installed."
LABEL org.opencontainers.image.documentation="https://github.com/VADOSWARE/pg_idkit#readme"
LABEL org.opencontainers.image.licenses="Apache-2.0"
LABEL org.opencontainers.image.revision=$PGIDKIT_REVISION
LABEL org.opencontainers.image.source="https://github.com/VADOSWARE/pg_idkit"
LABEL org.opencontainers.image.title="Postgres + pg_idkit"
LABEL org.opencontainers.image.url="https://github.com/VADOSWARE/pg_idkit"
LABEL org.opencontainers.image.vendor="VADOSWARE"
LABEL org.opencontainers.image.version=v${PGIDKIT_VERSION}
