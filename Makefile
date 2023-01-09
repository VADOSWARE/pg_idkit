# Makefile preamble (https://tech.davis-hansson.com/p/make/)
SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-print-directory
MAKEFLAGS += --quiet

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

OPENSSL ?= openssl
DOCKER ?= docker

PULUMI_SECRET_DIR ?= secrets/pulumi
PULUMI_AWS_ACCESS_KEY_ID_PATH ?= $(PULUMI_SECRET_DIR)/aws-access-key-id.secret
PULUMI_AWS_SECRET_ACCESS_KEY_PATH ?= $(PULUMI_SECRET_DIR)/aws-secret-access-key.secret

.PHONY: all \
# Tooling
>				check-tool-cargo check-tool-cargo-watch \
# Build
>				print-version \
>				build build-watch build-test-watch \
>				package test \
# Docker
>				image \
>				build-ci-image push-ci-image

CARGO ?= cargo
CARGO_WATCH ?= cargo-watch

CHANGELOG_FILE_PATH ?= CHANGELOG

all: build

###########
# Tooling #
###########

check-tool-cargo:
ifeq (,$(shell which $(CARGO)))
>	$(error "please enture cargo and the rust toolchain is installed (see: https://github.com/rust-lang/cargo/)")
endif

check-tool-cargo-watch:
ifeq (, $(shell which $(CARGO_WATCH)))
>	$(error "`cargo-watch` is not available please install cargo-watch (https://github.com/passcod/cargo-watch)")
endif

#########
# Build #
#########

# NOTE: this assumes the *first* version is the package version
VERSION ?= $(shell grep 'version' Cargo.toml | head -n 1 | sed -rn 's/version\s*=\s*(.*)/\1/p')

print-version:
>	@echo -n "$(VERSION)"

changelog:
>	$(GIT) cliff --unreleased --tag=$(VERSION) --prepend=$(CHANGELOG_FILE_PATH)

build:
>	$(CARGO) build

build-watch: check-tool-cargo check-tool-cargo-watch
>	$(CARGO_WATCH) -x "build $(CARGO_BUILD_FLAGS)" --watch src

build-test-watch: check-tool-cargo check-tool-cargo-watch
>	$(CARGO_WATCH) -x "test $(CARGO_BUILD_FLAGS)" --watch src

package:
>	$(CARGO) pgx package

test:
>	$(CARGO) test
>	$(CARGO) pgx test

##########
# Docker #
##########

POSTGRES_IMAGE_VERSION ?= 15.1.0
POSTGRES_IMAGE_TAG ?= ${POSTGRES_IMAGE_VERSION}-alpine

IMAGE_NAME ?= postgres
IMAGE_TAG ?= ${POSTGRES_IMAGE_VERSION}-pg_idkit-${VERSION}
IMAGE_NAME_FULL ?= "$(IMAGE_NAME):$(IMAGE_TAG)"

DOCKERFILE_PATH ?= ./infra/docker/${POSTGRES_IMAGE_TAG}.Dockerfile

CI_DOCKERFILE_PATH ?= ./infra/docker/ci.Dockerfile
CI_IMAGE_NAME ?= github.com/vadosware/pg_idkit/builder
CI_IMAGE_TAG ?= 0.x.x
CI_IMAGE_NAME_FULL ?= "$(IMAGE_NAME):$(IMAGE_TAG)"

image:
>	$(DOCKER) build -f $(DOCKERFILE_PATH) -t $(IMAGE_NAME_FULL)

build-ci-image:
>	$(DOCKER) build -f $(CI_DOCKERFILE_PATH) -t $(CI_IMAGE_NAME_FULL) .

push-ci-image:
>	$(DOCKER) push $(IMAGE_NAME_FULL)
