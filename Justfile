# Choose sell based on platform
shell := if os() == "macos" { "zsh" } else { "bash" }

docker := env_var_or_default("DOCKER", "docker")
git := env_var_or_default("GIT", "git")
tar := env_var_or_default("TAR", "tar")
strip := env_var_or_default("STRIP", "strip")
just := env_var_or_default("JUST", just_executable())

cargo := env_var_or_default("CARGO", "cargo")
cargo_get := env_var_or_default("CARGO_GET", "cargo-get")
cargo_generate_rpm := env_var_or_default("CARGO_GENERATE_RPM", "cargo-generate-rpm")
cargo_watch := env_var_or_default("CARGO_WATCH", "cargo-watch")
cargo_profile := env_var_or_default("CARGO_PROFILE", "")
cargo_profile_arg := if cargo_profile != "" {
  "--profile " + cargo_profile
} else {
  ""
}
cargo_features := env_var_or_default("CARGO_FEATURES", "")
cargo_features_arg := if cargo_features != "" {
  "--no-default-features --features " + cargo_features
} else {
  ""
}

changelog_file_path := absolute_path(justfile_directory() / "CHANGELOG")

pkg_pg_version := env_var_or_default("PKG_PG_VERSION", "17.5")
pkg_pg_config_path := env_var_or_default("PKG_PG_CONFIG_PATH", "~/.pgrx/" + pkg_pg_version + "/pgrx-install/bin/pg_config")
pkg_tarball_suffix := env_var_or_default("PKG_TARBALL_SUFFIX", "")

pgrx_pg_version := env_var_or_default("PGRX_PG_VERSION", "pg17")
pgrx_pkg_path_prefix := env_var_or_default("PGRX_PKG_PATH_PREFIX", "target")
# If /root, 'home' does not appear in the generated prefix
pkg_user_dir_prefix := if docker_build_user == "root" { docker_build_user } else { "home/" + docker_build_user }
pgrx_pkg_output_dir := pgrx_pkg_path_prefix / "release" / "pg_idkit-" + pgrx_pg_version / pkg_user_dir_prefix / ".pgrx" / pkg_pg_version / "pgrx-install"

docker_build_user := env_var_or_default('DOCKER_BUILD_USER', "root")

@_default:
    {{just}} --list

###########
# Tooling #
###########

_check-installed-version tool msg:
    #!/usr/bin/env -S {{shell}} -euo pipefail
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

@_check-tool-strip:
    {{just}} _check-installed-version {{strip}} "'strip' not available, please install strip (https://www.man7.org/linux/man-pages/man1/strip.1.html)"

@_check-tool-cargo-generate-rpm:
    {{just}} _check-installed-version {{cargo_generate_rpm}} "'cargo-generate-rpm' not available, please install cargo-generate-rpm (https://crates.io/crates/cargo-generate-rpm)"

#########
# Build #
#########

version := env_var_or_default("VERSION", "0.3.1")

# Print the current version (according to the script)
[group('meta')]
@get-version: _check-tool-cargo-get
    echo -n `cargo get package.version`

# Print the current revision (according to the script)
[group('meta')]
@get-revision: _check-tool-cargo-get
    echo -n `git rev-parse --short HEAD`

############
# Metadata #
############

# Print the current version
[group('meta')]
print-version:
    #!/usr/bin/env -S {{shell}} -euo pipefail
    echo -n `{{just}} get-version`

# Print the current revision
[group('meta')]
print-revision:
    #!/usr/bin/env -S {{shell}} -euo pipefail
    echo -n `{{just}} get-revision`

# Print package output directory
[group('meta')]
print-pkg-output-dir:
    echo -n {{pgrx_pkg_output_dir}}

# Set the version on the package
[group('release')]
set-version version:
    {{cargo}} set-version {{version}}

###############
# Development #
###############

# Initialize pgrx
[group('setup')]
pgrx-init:
    #!/usr/bin/env -S {{shell}} -euo pipefail
    if [ ! -d "{{pkg_pg_config_path}}" ]; then
      echo "failed to find pgrx init dir [{{pkg_pg_config_path}}], running pgrx init...";
      {{cargo}} pgrx init --{{pgrx_pg_version}}=download
    fi

# Initialize all pgrx versions
[group('setup')]
pgrx-init-all:
    #!/usr/bin/env -S {{shell}} -euo pipefail
    {{cargo}} pgrx init

# Perform all required setup for the project
[group('setup')]
setup:
    {{just}} pgrx-init

# Lint the project
[group('dev')]
lint:
    {{cargo}} clippy {{cargo_features_arg}} {{cargo_profile_arg}} --all-targets

# Build the pg_idkit project
[group('dev')]
build:
    {{cargo}} build {{cargo_features_arg}} {{cargo_profile_arg}}

# Build the project continuously
[group('dev')]
build-watch: _check-tool-cargo _check-tool-cargo-watch
    {{cargo_watch}} -x "build $(CARGO_BUILD_FLAGS)" --watch src

# Build the release version
[group('dev')]
build-release:
    {{cargo}} build --release {{cargo_features_arg}}

# Build tests continuously
[group('dev')]
build-test-watch: _check-tool-cargo _check-tool-cargo-watch
    {{cargo_watch}} -x "test $(CARGO_BUILD_FLAGS)" --watch src

# Run tests
[group('test')]
test:
    {{cargo}} test {{cargo_profile_arg}}
    {{cargo}} pgrx test

###########
# Release #
###########

# Build the package
[group('release')]
build-package:
    PGRX_IGNORE_RUST_VERSIONS=y {{cargo}} pgrx package --pg-config {{pkg_pg_config_path}} -vvv

# Package the project
[group('release')]
package: build-package
    mkdir -p pkg/pg_idkit-$({{just}} print-version)
    cp -r $({{just}} print-pkg-output-dir)/* pkg/pg_idkit-$({{just}} print-version)
    {{tar}} -C pkg -cvf pg_idkit-$(just print-version){{pkg_tarball_suffix}}.tar.gz  pg_idkit-$({{just}} print-version)

# Generate changelog
[group('release')]
changelog:
    {{git}} cliff --unreleased --tag={{version}} --prepend={{changelog_file_path}}

##########
# Docker #
##########

container_img_arch := env_var_or_default("CONTAINER_IMAGE_ARCH", "amd64")

pg_image_version := env_var_or_default("POSTGRES_IMAGE_VERSION", "17.5")
pg_os_image_version := env_var_or_default("POSTGRES_OS_IMAGE_VERSION", "alpine3.21.3")

pgidkit_image_name := env_var_or_default("PGIDKIT_IMAGE_NAME", "ghcr.io/vadosware/pg_idkit")
pgidkit_image_tag := env_var_or_default("POSGRES_IMAGE_VERSION", version + "-" + "pg" + pg_image_version + "-" + pg_os_image_version + "-" + container_img_arch)
pgidkit_image_tag_suffix := env_var_or_default("PGIDKIT_IMAGE_TAG_SUFFIX", "")
pgidkit_image_name_full := env_var_or_default("PGIDKIT_IMAGE_NAME_FULL", pgidkit_image_name + ":" + pgidkit_image_tag + pgidkit_image_tag_suffix)
pgidkit_dockerfile_path := env_var_or_default("PGIDKIT_DOCKERFILE_PATH", "infra" / "docker" / pgidkit_image_tag + ".Dockerfile")

docker_password_path := env_var_or_default("DOCKER_PASSWORD_PATH", "secrets/docker/password.secret")
docker_username_path := env_var_or_default("DOCKER_USERNAME_PATH", "secrets/docker/username.secret")
docker_image_registry := env_var_or_default("DOCKER_IMAGE_REGISTRY", "ghcr.io/vadosware/pg_idkit")
docker_config_dir := env_var_or_default("DOCKER_CONFIG", "secrets/docker")

img_dockerfile_path := "infra" / "docker" / "pg_idkit-pg" + pg_image_version + "-" + pg_os_image_version + "-" + container_img_arch + ".Dockerfile"

# Ensure that that a given file is present
_ensure-file file:
    #!/usr/bin/env -S {{shell}} -euo pipefail
    if [ ! -f "{{file}}" ] ; then
      echo "[error] file [{{file}}] is required, but missing";
      exit 1;
    fi;

# Log in with docker using local credentials
[group('package')]
docker-login:
    {{just}} _ensure-file {{docker_password_path}}
    {{just}} _ensure-file {{docker_username_path}}
    cat {{docker_password_path}} | {{docker}} login {{docker_image_registry}} -u `cat {{docker_username_path}}` --password-stdin
    cp {{docker_config_dir}}/config.json {{docker_config_dir}}/.dockerconfigjson

docker_platform_arg := env_var_or_default("DOCKER_PLATFORM_ARG", "")
docker_progress_arg := env_var_or_default("DOCKER_PROGRESS_ARG", "")

##########################
# Docker Image - builder #
##########################
#
# This image is used as a cache for speeding up CI builds,
# and for performing builds when building release artifacts
#

builder_type := env_var_or_default("BUILDER_TYPE", "gnu") # alternatively, 'musl'

builder_dockerfile_path := env_var_or_default("BUILDER_DOCKERFILE_PATH", "infra" / "docker" / "builder-" + builder_type + ".Dockerfile")
builder_image_name := env_var_or_default("BUILDER_IMAGE_NAME", "ghcr.io/vadosware/pg_idkit/builder-" + builder_type)
builder_image_tag := env_var_or_default("BUILDER_IMAGE_TAG", "0.1.x")
builder_image_name_full := env_var_or_default("BUILDER_IMAGE_NAME_FULL", builder_image_name + ":" + builder_image_tag)

builder_image_arg_cargo_pgrx_version := env_var_or_default("BUILDER_IMAGE_ARG_CARGO_PGRX_VERSION", "0.15.0")

# Build the docker image used in BUILDER
[group('package')]
build-builder-image:
    {{docker}} build \
      -f {{builder_dockerfile_path}} \
      -t {{builder_image_name_full}} \
      --build-arg CARGO_PGRX_VERSION={{builder_image_arg_cargo_pgrx_version}} \
      .

# Push the docker image used in BUILDER (to GitHub Container Registry)
[group('package')]
push-builder-image:
    {{docker}} push {{builder_image_name_full}}

###########################
# Docker Image - base-pkg #
###########################
#
# This image is used as a base for packaging flows, usually while building
# the end-user facing Docker image that is contains Postgres & pg_idkit
#

# Determine the Dockerfile to use when building the packaging utility base image
base_pkg_dockerfile_path := "infra/docker/base-pkg-" + pg_os_image_version + "-" + container_img_arch + ".Dockerfile"
base_pkg_image_name := env_var_or_default("PKG_IMAGE_NAME", "ghcr.io/vadosware/pg_idkit/base-pkg")
base_pkg_version := env_var_or_default("PKG_IMAGE_NAME", "0.1.x")
base_pkg_image_tag := env_var_or_default("POSGRES_IMAGE_VERSION", base_pkg_version + "-" + pg_os_image_version + "-" + container_img_arch)
base_pkg_image_name_full := env_var_or_default("PKG_IMAGE_NAME_FULL", base_pkg_image_name + ":" + base_pkg_image_tag)

# Build the base image for packaging
[group('package')]
build-base-pkg-image:
      {{docker}} build --build-arg USER={{docker_build_user}} -f {{base_pkg_dockerfile_path}} . -t {{base_pkg_image_name_full}};

# Push the base image for packaging
[group('package')]
push-base-pkg-image:
      {{docker}} push {{base_pkg_image_name_full}}

###########################
# Docker Image - pg_idkit #
###########################
#
# This image is the pg_idkit image itself, normally built FROM
# a image of base-pkg
#

# Build the docker image for pg_idkit
[group('package')]
build-image:
    {{docker}} build \
      {{docker_platform_arg}} \
      {{docker_progress_arg}} \
      -f {{img_dockerfile_path}} \
      -t {{pgidkit_image_name_full}} \
      --build-arg USER={{docker_build_user}} \
      --build-arg PGIDKIT_REVISION=`{{just}} get-revision` \
      --build-arg PGIDKIT_VERSION={{pgidkit_image_tag}} \
      .

# Push the docker image for pg_idkit
[group('package')]
push-image:
    {{docker}} push {{pgidkit_image_name_full}}

#######
# RPM #
#######

rpm_arch := env_var_or_default("RPM_ARCH", "x86_64")

rpm_file_name := env_var_or_default("RPM_OUTPUT_PATH", "pg_idkit-" + version + "-" + pgrx_pg_version + "." + rpm_arch + ".rpm")
rpm_output_path := "target" / "generate-rpm" / rpm_file_name

# Cargo.toml depends on this file being at the location below.
rpm_scratch_location := "/tmp/pg_idkit/rpm/scratch"

# Build an RPM distribution for pg_idkit
[group('package-rpm')]
build-rpm: _check-tool-strip _check-tool-cargo-generate-rpm
    CARGO_FEATURES={{pgrx_pg_version}} {{just}} package
    {{strip}} -s {{pgrx_pkg_output_dir}}/lib/postgresql/pg_idkit.so
    mkdir -p {{rpm_scratch_location}}
    cp -r {{pgrx_pkg_output_dir}} {{rpm_scratch_location}}
    {{cargo_generate_rpm}} --variant {{pgrx_pg_version}}

# Print the RPM output file name
[group('package-rpm')]
@print-rpm-output-file-name:
    echo -n {{rpm_file_name}}

# Print the RPM output path
[group('package-rpm')]
@print-rpm-output-path:
    echo -n {{rpm_output_path}}
