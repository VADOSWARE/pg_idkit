# Continuous Integration

To push up images that are used from continuous integration:

1. Get a personal access token from Github
2. Ensuring `DOCKER_LOGIN` is set (see instructions above)
3. Perform a login
   1. Manually via `echo $GH_PAT | docker login ghcr.io -u <username> --password-stdin`
   2. Automatically, via `just docker-login` which will use the `git-crypt` protected credentials (you must have run `git-crypt` unlock first)
4. Observe the docker login credentials generated in this local repo directory (`secrets/docker/config.json`)
5. Run `just build-ci-image push-ci-image`
