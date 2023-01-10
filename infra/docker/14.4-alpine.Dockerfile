# rust:alpine as of 2022/07/31
FROM sha256:0ebddf3a3a92320ff15b6cd3c5603ad109d71dd241ebfbda5a3c79acd91fa7ef AS build

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
