# polite-cli

[![crates.io](https://img.shields.io/crates/v/polite-cli.svg)](https://crates.io/crates/polite-cli)
[![Documentation](https://docs.rs/polite-cli/badge.svg)](https://docs.rs/polite-cli)
[![MIT licensed](https://img.shields.io/crates/l/polite-cli.svg)](https://github.com/lmmx/polite/blob/master/LICENSE)

A command-line interface for [polite](https://github.com/lmmx/polite/tree/master/polite), the rusqlite × Polars bridge library.

## Installation

```bash
cargo install polite-cli
````

## Usage

### Create a table

```bash
polite-cli exec "CREATE TABLE t (id INTEGER, name TEXT)"
```

### Insert a row

```bash
polite-cli exec "INSERT INTO t VALUES (1, 'Alice')"
```

### Query into a DataFrame

```bash
polite-cli query "SELECT * FROM t"
```

By default, the database is in-memory. Use `--db path/to/db.sqlite` to persist to a file.

## Example

```bash
polite-cli --db example.sqlite exec "CREATE TABLE users (id INTEGER, name TEXT)"
polite-cli --db example.sqlite exec "INSERT INTO users VALUES (1, 'Bob')"
polite-cli --db example.sqlite query "SELECT * FROM users"
```

## Documentation

* **Library crate**: [polite](https://github.com/lmmx/polite/tree/master/polite)
* **Workspace guide**: [DEVELOPMENT.md](https://github.com/lmmx/polite/blob/master/DEVELOPMENT.md)

## License

Licensed under the MIT License. See [LICENSE](https://github.com/lmmx/polite/blob/master/LICENSE) for details.
