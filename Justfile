cargo := env_var_or_default("CARGO", "cargo")
cargo_get := env_var_or_default("CARGO_GET", "cargo-get")
cargo_watch := env_var_or_default("CARGO_WATCH", "cargo-watch")
docker := env_var_or_default("DOCKER", "docker")
git := env_var_or_default("GIT", "git")
just := env_var_or_default("JUST", just_executable())

changelog_path := "CHANGELOG"

default:
    {{just}} --list

###########
# Tooling #
###########

_check-installed-version tool msg:
    #!/usr/bin/env -S bash -euo pipefail
    if [ -z "$(command -v {{tool}})" ]; then
      echo "{{msg}}";
      exit 1;
    fi

@_check-tool-cargo:
    {{just}} _check-installed-version {{cargo}} "'cargo' not available, please install the Rust toolchain (see: https://github.com/rust-lang/cargo/)";

@_check-tool-cargo-watch:
    {{just}} _check-installed-version {{cargo_watch}} "'cargo-watch' not available, please install cargo-watch (https://github.com/passcod/cargo-watch)"

@_check-tool-cargo-get:
    {{just}} _check-installed-version {{cargo_get}} "'cargo-get' not available, please install cargo-get (https://crates.io/crates/cargo-get)"

#########
# Build #
#########

version := `grep 'version' Cargo.toml | head -n 1 | sed -rn 's/version\s*=\s*(.*)/\1/p'`

# NOTE: we can't use this as the official version getter until
# see: https://github.com/nicolaiunrein/cargo-get/issues/14
@get-version: _check-tool-cargo-get
    cargo get package.version

print-version:
    #!/usr/bin/env -S bash -euo pipefail
    echo -n `{{just}} get-version`

changelog:
    {{git}} cliff --unreleased --tag=$(VERSION) --prepend=$(CHANGELOG_FILE_PATH)

build:
    {{cargo}} build

build-watch: _check-tool-cargo _check-tool-cargo-watch
    {{cargo_watch}} -x "build $(CARGO_BUILD_FLAGS)" --watch src

build-test-watch: _check-tool-cargo _check-tool-cargo-watch
    {{cargo_watch}} -x "test $(CARGO_BUILD_FLAGS)" --watch src

package:
    {{cargo}} pgrx package

test:
    {{cargo}} test
    {{cargo}} pgrx test

##########
# Docker #
##########

pg_image_version := env_var_or_default("POSTGRES_IMAGE_VERSION", "15.4")
pg_image_tag := env_var_or_default("POSGRES_IMAGE_VERSION", pg_image_version + "-alpine3.18")

pgkit_image_name := env_var_or_default("PGKIT_IMAGE_NAME", "postgres")
pgkit_image_tag := env_var_or_default("PGKIT_IMAGE_TAG", pg_image_version + "-pg_idkit=" + version)
pgkit_image_name_full := env_var_or_default("PGKIT_IMAGE_NAME_FULL", pgkit_image_name + ":" + pgkit_image_tag)
pgkit_dockerfile_path := env_var_or_default("PGKIT_DOCKERFILE_PATH", "infra" / "docker" / pgkit_image_tag + ".Dockerfile")

ci_dockerfile_path := env_var_or_default("CI_DOCKERFILE_PATH", "infra" / "docker" / "ci.Dockerfile")
ci_image_name := env_var_or_default("CI_IMAGE_NAME", "ghcr.io/vadosware/pg_idkit/builder")
ci_image_tag := env_var_or_default("CI_IMAGE_TAG", "0.x.x")
ci_image_name_full := env_var_or_default("CI_IMAGE_NAME_FULL", ci_image_name + ":" + ci_image_tag)

docker_password_path := env_var_or_default("DOCKER_PASSWORD_PATH", "secrets/docker/password.secret")
docker_username_path := env_var_or_default("DOCKER_USERNAME_PATH", "secrets/docker/username.secret")
docker_image_registry := env_var_or_default("DOCKER_IMAGE_REGISTRY", "ghcr.io/vadosware/pg_idkit")
docker_config_dir := env_var_or_default("DOCKER_CONFIG", "secrets/docker")

# Ensure that that a given file is present
_ensure-file file:
    #!/usr/bin/env -S bash -euo pipefail
    @if [ ! -f "{{file}}" ]; then
      echo "[error] file [{{file}}] is required, but missing";
      exit 1;
    fi;

# Log in with docker using local credentials
docker-login:
    {{just}} ensure-file {{docker_password_path}}
    {{just}} ensure-file {{docker_username_path}}
    cat {{docker_password_path}} | {{docker}} login {{docker_image_registry}} -u `cat {{docker_username_path}}` --password-stdin
    cp {{docker_config_dir}}/config.json {{docker_config_dir}}/.dockerconfigjson

# Build the docker image for pg_idkit
image:
    {{docker}} build -f {{pgkit_dockerfile_path}} -t {{pgkit_image_name_full}}

# Build the docker image used in CI
build-ci-image:
    {{docker}} build -f {{ci_dockerfile_path}} -t {{ci_image_name_full}} .

# Push the docker image used in CI (to GitHub Container Registry)
push-ci-image:
    {{docker}} push {{ci_image_name_full}}
