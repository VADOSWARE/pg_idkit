# rust:1.90.0-alpine3.22 as of 2025/03/31
FROM rust:1.90.0-alpine3.22@sha256:b4b54b176a74db7e5c68fdfe6029be39a02ccbcfe72b6e5a3e18e2c61b57ae26

ARG CARGO_PGRX_VERSION=0.16.0
ENV CARGO_PGRX_VERSION=${CARGO_PGRX_VERSION}

RUN apk add --no-cache musl-dev openssl-dev perl make

##################
# Setup Postgres #
##################

# Install dependencies for building postgres, NodeJS, etc
RUN apk add postgresql-dev nodejs

# Add postgres user to wheel group
RUN addgroup --system idkit && \
    adduser --system --disabled-password --home /home/idkit --ingroup idkit --shell /bin/ash idkit && \
    mkdir -p /home/idkit && \
    chown idkit:idkit /home/idkit && \
    adduser idkit wheel

###############
# Setup Cargo #
###############

# Allow superuser group to write to cargo cache (idkit is now part of wheel)
RUN chmod g+w -R /usr/local/cargo && \
    chgrp wheel -R /usr/local/cargo

# Install development/build/testing deps
RUN su idkit -c "cargo install --locked just cargo-cache cargo-get cargo-edit"

# Install cargo-pgrx
# NOTE: cargo-pgrx has to be installed statically
RUN RUSTFLAGS="-Ctarget-feature=-crt-static" su idkit -c "cargo install --locked cargo-pgrx@$CARGO_PGRX_VERSION"

# Install postgres build deps used by pgrx
RUN apk add bison flex perl readline-dev zlib-dev make postgresql17-dev linux-headers

# Initialize cargo pgrx
#
# NOTE: pgrx should be reinitialized if cargo-pgrx or the default pg version changes
# at the project level
RUN su idkit -c "cargo pgrx init --pg17 download"
