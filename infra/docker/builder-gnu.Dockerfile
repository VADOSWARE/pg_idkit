FROM rust:1.99.1-slim-bookworm@sha256:f2c004e3d2d8b547e8cf7ffbc29205385f4e71d23e53e4e383065db5e9f822cd

ARG RUST_TOOLCHAIN_VERSION=1.99.1
RUN rustup toolchain install --profile minimal $RUST_TOOLCHAIN_VERSION && \
    rustup default $RUST_TOOLCHAIN_VERSION

ARG CARGO_PGRX_VERSION=0.20.1
ENV CARGO_PGRX_VERSION=${CARGO_PGRX_VERSION}

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

##################
# Setup Postgres #
##################

# Add postgresql.org PGP keys
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add

# Add postgres repo
RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt bookworm-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

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

# Install & Initialize pgrx
# Install development/build/testing deps
RUN su idkit -c "cargo install --locked just cargo-cache cargo-get cargo-edit cargo-pgrx@$CARGO_PGRX_VERSION"

# Initialize cargo pgrx
#
# NOTE: pgrx shoudl be reinitialized if cargo-pgrx or the default pg version changes
# at the project level
RUN su idkit -c "cargo pgrx init --pg17 download"
