# rust:1.69.0-slim-bullseye as of 2023/05/18
FROM rust:1.69.0-slim-bullseye@sha256:caba086dac32589a2c4e5ac43212eb2fa2eb1ddb49b19dbd976c89115af56f70

ENV CARGO_HOME=/usr/local/cargo
ENV CARGO_TARGET_DIR=/usr/local/build/target
ENV SCCACHE_DIR=/usr/local/sccache

# Install deps
RUN apt update && apt install -y libssl-dev git openssh-client pkg-config curl ca-certificates gnupg wget
RUN cargo install sccache

ENV CARGO_BUILD_RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

##################
# Setup Postgres #
##################

# Add postgresql.org PGP keys
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add

# Add postgres repo
RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt bullseye-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

# Install pg14
RUN apt -y update && apt -y upgrade && apt install -y postgresql-14

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

# Allow writing to cargo cache by idkit (now part of root group)
RUN chmod g+w -R /usr/local/cargo
RUN chmod g+w -R /usr/local/build

# Install development/build/testing deps
# NOTE: version of cargo-pgx must be handled speicfically
RUN su idkit -c "cargo install sccache cargo-cache cargo-pgx@0.7.4"

# Initialize pgx
# NOTE: pgx must be reinitialized if cargo-pgx changes
RUN su idkit -c "cargo pgx init --pg14 download"
