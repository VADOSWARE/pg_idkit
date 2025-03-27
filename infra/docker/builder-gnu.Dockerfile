# rust:1.85.1-slim-bullseye as of 2025/03/27
FROM rust:1.85.1-slim-bullseye@sha256:1eca9af45f393ac4669b9b63659529638359575f6268cbd4e6543ddc46c53803

ARG CARGO_PGRX_VERSION=0.12.5
ENV CARGO_PGRX_VERSION=${CARGO_PGRX_VERSION}

ENV CARGO_HOME=/usr/local/cargo
ENV CARGO_TARGET_DIR=/usr/local/build/target
ENV SCCACHE_DIR=/usr/local/sccache
# Disable cargo incremental builds since sccache can't support them
ENV CARGO_INCREMENTAL=0

# Install deps
#
# NOTE: some deps are used by cargo-pgrx at build/test time (ex. bison)
RUN apt update && \
    apt install -y \
    libssl-dev \
    git \
    openssh-client \
    pkg-config \
    curl \
    ca-certificates \
    gnupg \
    wget \
    bison \
    flex
RUN cargo install sccache --locked

ENV CARGO_BUILD_RUSTC_WRAPPER=/usr/local/cargo/bin/sccache

##################
# Setup Postgres #
##################

# Add postgresql.org PGP keys
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add

# Add postgres repo
RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt bullseye-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

# Install pg15
RUN apt -y update && apt -y upgrade && apt install -y postgresql-15

# Install NodeJS
RUN set -uex; \
    apt-get update; \
    apt-get install -y ca-certificates curl gnupg; \
    mkdir -p /etc/apt/keyrings; \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
     | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg; \
    NODE_MAJOR=20; \
    echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" \
     > /etc/apt/sources.list.d/nodesource.list; \
    apt-get update; \
    apt-get install nodejs -y;

# Install dependencies for building postgres, NodeJS, etc
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates \
    git build-essential libpq-dev \
    postgresql-server-dev-all \
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

RUN mkdir /usr/local/sccache
RUN chmod g+w -R /usr/local/sccache

# Install & Initialize pgrx
# Install development/build/testing deps
RUN su idkit -c "cargo install just sccache cargo-cache cargo-get cargo-edit cargo-pgrx@$CARGO_PGRX_VERSION"

# Initialize cargo pgrx
#
# NOTE: pgrx shoudl be reinitialized if cargo-pgrx or the default pg version changes
# at the project level
RUN su idkit -c "cargo pgrx init --pg17 download"
