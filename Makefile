.PHONY: all \
# Tooling
				check-tool-cargo \
# Build
				print-version build \
# Docker
				image

CARGO ?= cargo

all: build

###########
# Tooling #
###########

check-tool-cargo:
ifeq (,$(shell which $(CARGO)))
	$(error "please enture cargo and the rust toolchain is installed (see: https://github.com/rust-lang/cargo/)")
endif

#########
# Build #
#########

print-version:
	@echo -n "$(VERSION)"

build:
	$(CARGO) build

##########
# Docker #
##########

# NOTE: this assumes the *first* version is the package version
VERSION ?= $(shell grep 'version' Cargo.toml | head -n 1 | sed -rn 's/version\s*=\s*(.*)/\1/p')

POSTGRES_IMAGE_VERSION ?= 14.4.0

IMAGE_NAME ?= postgres
IMAGE_TAG ?= postgres:${POSTGRES_VERSION}-pg_idkit-${VERSION}

DOCKERFILE_PATH ?= ./infra/docker/Dockerfile

image:
	$(DOCKERFILE) -f $(DOCKERFILE_PATH)
