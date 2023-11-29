# Packaging

## Setting up for Docker usage

Assuming you are using something like [`direnv`][direnv], use the following `.envrc` file:

```
# Use local docker auth file
export DOCKER_CONFIG=$(realpath secrets/docker)
```

**NOTE**, that is *not* a `.env` file, it is a `.envrc` file, with separate semantics
