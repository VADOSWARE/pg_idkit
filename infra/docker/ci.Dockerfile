FROM rust:1.6.3-slim-bullseye@sha256:4d6b7664f5292cdfbeaa7eb9f1f4eae01aa289a49e4f043cdf6f4f63d0cf8ca8

# Install deps
RUN apt update && apt install -y libssl-dev git openssh-client pkg-config
RUN cargo install sccache

ENV CARGO_HOME=/usr/local/cargo
ENV CARGO_TARGET_DIR=/usr/local/build/target
ENV SCCACHE_DIR=/usr/local/sccache
ENV CARGO_BUILD_RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

# Add postgres repo
RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt bullseye-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

# Add postgresql.org PGP keys
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add

# Install NodeJS
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -

# Install dependencies for building postgres, NodeJS, etc
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates \
    git build-essential libpq-dev \
    postgresql postgresql-server-dev-14 \
    curl libreadline6-dev zlib1g-dev libclang-dev \
    pkg-config cmake nodejs


# Add postgres user to root group
RUN useradd --user-group --system --create-home --no-log-init idkit && \
    usermod -aG sudo idkit && \
    chown -R idkit /home/idkit && \
    addgroup idkit root

# Allow writing to postgres extensions folder
RUN chmod g+w -R /usr/share/postgresql/**/extension && \
    chmod g+w -R /usr/lib/postgresql/**/lib

      ###############
      # Setup Cargo #
      ###############

# Install development/build/testing deps
RUN su idkit -c "cargo install sccache cargo-cache cargo-pgx"

# Initialize pgx
RUN su idkit -c "cargo pgx init --pg15 $(which pg_config)"
