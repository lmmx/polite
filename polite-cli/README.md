# polite-cli

[![crates.io](https://img.shields.io/crates/v/polite-cli.svg)](https://crates.io/crates/polite-cli)
[![Documentation](https://docs.rs/polite-cli/badge.svg)](https://docs.rs/polite-cli)
[![MIT licensed](https://img.shields.io/crates/l/polite-cli)](https://github.com/lmmx/polite/blob/master/LICENSE)

A command-line interface for [polite](https://github.com/lmmx/polite/tree/master/polite),
the rusqlite × Polars bridge library.

## Installation

```bash
cargo install polite-cli
```

## Usage

The CLI takes:

```bash
polite "<SQL>" <DB_PATH>
```

and automatically decides:

- If the SQL starts with `SELECT` → runs it and prints a Polars `DataFrame`.
- Otherwise → executes the statement and prints rows affected.

⚠️ **Important:**
ConnectorX (used internally for DataFrame reads) requires a **file-backed database**.
You must provide a database file path (e.g. `mydb.sqlite`). In-memory databases (`:memory:`) are not supported.

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

## Example

```bash
polite "CREATE TABLE users (id INTEGER, name TEXT)" example.sqlite
polite "INSERT INTO users VALUES (1, 'Bob')" example.sqlite
polite "SELECT * FROM users" example.sqlite
```

## Notes

- **File-backed SQLite databases only.** ConnectorX cannot read from in-memory databases.
- Temporary files (`tempfile`) are a good option if you don’t want the DB to persist.

## Documentation

- **Library crate**: [polite](https://github.com/lmmx/polite/tree/master/polite)
- **Workspace guide**: [DEVELOPMENT.md](https://github.com/lmmx/polite/blob/master/DEVELOPMENT.md)

## License

Licensed under the MIT License.
See [LICENSE](https://github.com/lmmx/polite/blob/master/LICENSE) for details.
