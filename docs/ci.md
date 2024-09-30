# Continuous Integration

To push up images that are used from continuous integration:

1. Get a personal access token from Github
2. Ensuring `DOCKER_LOGIN` is set (see instructions above)
3. Perform a login
   1. Manually via `echo $GH_PAT | docker login ghcr.io -u <username> --password-stdin`
   2. Automatically, via `just docker-login` which will use the `git-crypt` protected credentials (you must have run `git-crypt` unlock first)
4. Observe the docker login credentials generated in this local repo directory (`secrets/docker/config.json`)
5. Run `just build-ci-image push-ci-image`

## FAQ

Issues that come up during FAQ

### Old/outdated `cargo-pgrx` versions during CI runs

If you find a CI run failing due to an unexpectedly outdated version of `cargo-pgrx`, try the following:

- Ensure relevant builder Dockerfiles are up to date (ex. [`infra/docker/builder-gnu.Dockerfile`](../infra/docker/builder-gnu.Dockerfile))
- Ensure relevant base Dockerfiles are up to date (ex. [`infra/docker/base-pkg-alpine3.20.3-amd64.Dockerfile`](../infra/docker/base-pkg-alpine3.20.3-amd64.Dockerfile))
- Clear CI build caches (via online GUI)

> ![NOTE]
> See [the `Justfile`](../Justfile) for easy-to-run targets that build and push
> the images mentioned above (ex. `just build-builder-image`)
