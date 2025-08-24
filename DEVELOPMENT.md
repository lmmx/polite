## Precommit and Justfile

We use [pre-commit](https://pre-commit.com/) for formatting, linting, and quick CI checks,
and the tasks they run are stored in the Justfile (run by [just](https://github.com/casey/just)).

Pre-commit bundles all the dependencies for CI and you can just run these and execute
the tests you need to check.

```sh
pre-commit run --all-files
```

To install the hooks locally:

```sh
just install-hooks
```

Run them manually:

- `just check` runs cargo check + clippy
- `just test` runs cargo test
- `just full` runs the full suite of checks
- `just run-pc` runs the pre-commit suite

Requirements:

- Rust toolchain
- `just`
- `echo-comment` (via cargo)

To install precommit hooks run `just install-hooks` and to run them use `just run-pc`

## Release

The release process is two more commands (if it works the first time it could be one)

```sh
just ship
```

If the dry run fails, you can revert and re-run the last step when it succeeds (but if all is OK you
won't need to):

```sh
just publish
```
