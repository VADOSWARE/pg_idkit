# rust:1.69.0-alpine3.17 as of 2023/05/18
FROM rust:1.69.0-alpine3.17@sha256:3dd0bb6f134635fe40dd9c18bd9603f9d90ce3538ac25ae3e69b9b127137acf2 AS build

WORKDIR /app
COPY . .

RUN make build

# postgres:14.4-alpine as of 2022/07/31
FROM sha256:7d6403121b12c9d29c13c9873bc0f76da8ff51f6c89fae10b99dc890807e27ae

# SHAREDIR should be /usr/local/share/postgresql (pg_config --sharedir)
COPY --from=build /app/target/pg_idkit.so /usr/local/share/postgresql/extension/pg_idkit--$(VERSION).so
COPY --from=build /app/infra/pg/pg_idkit.control /usr/local/share/postgresql/extension/

RUN cat <<EOT >> /var/lib/postgresql/data/postgresql.com \
shared_preload_libraries='
