# wasm-workspace

> :warning: This repo is experimental and shouldn't be relied upon!

An example Cargo workspace for building wasms for use with [Namada](https://github.com/anoma/namada).

## Tests

### Unit and integration

```shell
cargo test
```

### End-to-end

End-to-end tests are binaries under `crates/e2e_tests/src/bin`. It should be possible to run them against any Namada chain.

#### Running locally using Docker Compose

> :warning: This method of running locally may not work on Apple Silicon.

To run them against a preconfigured network:

```shell
make docker  # should be run any time test runners or wasms change
docker compose up
```

The `ledger` container runs indefinitely.

The `testrunner` container will run all tests in series. It will exit if a test fails (exits with status code 2) or errors (exits with status code 1), or once all tests have successfully passed (exited with status code 0).

There is an `adhoc` container that can be SSH'ed into if you want to run test binaries or interact with the ledger manually.

```shell
docker compose exec -it adhoc /bin/bash
```

The test network can be reset with `docker compose down`.
