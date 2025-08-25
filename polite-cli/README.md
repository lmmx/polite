# polite-cli

[![crates.io](https://img.shields.io/crates/v/polite-cli.svg)](https://crates.io/crates/polite-cli)
[![Documentation](https://docs.rs/polite-cli/badge.svg)](https://docs.rs/polite-cli)
[![MIT licensed](https://img.shields.io/crates/l/polite-cli.svg)](https://github.com/lmmx/polite/blob/master/LICENSE)

A command-line interface for [polite](https://github.com/lmmx/polite/tree/master/polite),  
the rusqlite × Polars bridge library.

## Installation

```bash
cargo install polite-cli
````

## Usage

The CLI takes:

```bash
polite "<SQL>" [DB_PATH]
```

and automatically decides:

* If the SQL starts with `SELECT` → runs it and prints a Polars `DataFrame`.
* Otherwise → executes the statement and prints rows affected.

### Create a table

```bash
polite "CREATE TABLE t (id INTEGER, name TEXT)" mydb.sqlite
```

### Insert a row

```bash
polite "INSERT INTO t VALUES (1, 'Alice')" mydb.sqlite
```

### Query into a DataFrame

```bash
polite "SELECT * FROM t" mydb.sqlite
```

By default, the database is in-memory. Provide a path to persist to a file.

## Example

```bash
polite "CREATE TABLE users (id INTEGER, name TEXT)" example.sqlite
polite "INSERT INTO users VALUES (1, 'Bob')" example.sqlite
polite "SELECT * FROM users" example.sqlite
```

## Notes

* If you don’t pass a DB path, each command runs against a fresh in-memory database (so data won’t persist between runs).
* Output for `SELECT` queries is the Polars `DataFrame` debug format (MVP). Pretty-print and export options will be added in future versions.

## Documentation

* **Library crate**: [polite](https://github.com/lmmx/polite/tree/master/polite)
* **Workspace guide**: [DEVELOPMENT.md](https://github.com/lmmx/polite/blob/master/DEVELOPMENT.md)

## License

Licensed under the MIT License. See [LICENSE](https://github.com/lmmx/polite/blob/master/LICENSE) for details.
