#
# NOTE: you must have the base packaging layer built for this image to work
# you can build this from scratch with `just build-base-pkg-image`
#
FROM ghcr.io/vadosware/pg_idkit/base-pkg:0.1.x-pg15.5-alpine3.18-amd64 AS builder

ARG USER
ENV USER=$USER

# Re-run the build with the latest code
WORKDIR /pg_idkit
COPY . .
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" just build package

FROM postgres:15.5-alpine3.18@sha256:a57387207806d947c842f1be9f358e37b05442bf8b5ed19b1a69af281be930e7

# NOTE: PGRX_PG_VERSION is defined via base-pkg:pg15.5-alpine3.18-amd64
ARG PGRX_PG_VERSION=pg15
ARG PGIDKIT_REVISION

# Copy relevant built-files
COPY --from=builder /pg_idkit/target/release/pg_idkit-${PGRX_PG_VERSION}/usr /usr

# expected @ /usr/local/share/postgresql/extension
# actually @ ./local/share/postgresql/extension/pg_idkit.control

ARG PGIDKIT_VERSION
ARG PGIDKIT_REVISION

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
