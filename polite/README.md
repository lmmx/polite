# polite

[![crates.io](https://img.shields.io/crates/v/polite.svg)](https://crates.io/crates/polite)
[![Documentation](https://docs.rs/polite/badge.svg)](https://docs.rs/polite)
[![MIT licensed](https://img.shields.io/crates/l/polite.svg)](https://github.com/lmmx/polite/blob/master/LICENSE)

The core **rusqlite × Polars bridge**.
`polite` makes it easy to move data between SQLite databases and Polars `DataFrame`s.

## Features

- Open SQLite databases (**file-based only**).
- Execute arbitrary SQL statements.
- Bulk-load query results into Polars `DataFrame`s (`to_dataframe`) via \[ConnectorX].
- Write Polars `DataFrame`s into SQLite tables (`from_dataframe`).

## Requirements

When using `polite`, please be aware of the current upstream version restrictions:

- Built against Polars **0.45** (the latest release supported by ConnectorX).
- Pins `chrono <= 0.4.39` due to [an upstream Arrow/Polars issue](https://github.com/apache/arrow-rs/issues/7196)
  (this will be removed once the conflict is resolved there).

## Limitations (MVP)

- Supported SQLite column types:
  `INTEGER` → Polars `Int64`
  `REAL` → Polars `Float64`
  `TEXT` → Polars `String`
- Other SQLite types are stored as `String`.
- Output uses Polars’ standard debug `DataFrame` format.
- No advanced type inference or schema evolution yet.

⚠️ **Notes on SQLite backends**

- `polite` uses **ConnectorX** for bulk reads into Polars.
- **File-backed databases** (`.sqlite`, `.db`) are required.
- **In-memory databases** (`:memory:`) are not supported — use a `tempfile` if you don’t want persistence.

## Core functions

💡 **All of these functions are also available via use `polite::prelude::*;`.**

The two basic functions provided by the library are:

- `to_dataframe(db_path, sql)` – run a query and return a `DataFrame`.
- `from_dataframe(&conn, table, &df)` – write a `DataFrame` into a table.
  Takes an open `rusqlite::Connection`, the table name to write to, and your DataFrame.

`polite` also provides a couple of convenience wrappers
(with simplified string errors and without connection handling):

- `save_dataframe(db_path, table, &df)`  
  Opens a connection and writes the DataFrame in one step.  
  Creates and closes its own connection; use this for one-off saves.

- `load_dataframe(db_path, sql)`  
  Wraps `to_dataframe` but adds context to errors (e.g. `"Failed to load DataFrame from demo.db: no such table: users"`).  
  This makes it clearer where the failure came from, especially if you’re working with multiple databases.

These helpers are for convenience and don’t add new capabilities beyond the core API,
but they can reduce boilerplate or give clearer error messages when debugging.

## Example

```rust
use polite::prelude::*;
use tempfile::NamedTempFile;

fn main() -> anyhow::Result<()> {
    // Create a temporary file-backed SQLite DB
    let db = NamedTempFile::new()?;
    let db_path = db.path().to_str().unwrap();

    // Create a connection (rusqlite is used for writes)
    let conn = connect_sqlite(Some(db_path))?;

    // Create and populate a table
    execute_query(&conn, "CREATE TABLE t (id INTEGER, name TEXT)")?;
    execute_query(&conn, "INSERT INTO t VALUES (1, 'Alice')")?;
    execute_query(&conn, "INSERT INTO t VALUES (2, 'Bob')")?;

    // Query back into a Polars DataFrame (ConnectorX path)
    let df = to_dataframe(db_path, "SELECT * FROM t")?;
    println!("{:?}", df);

    // Write a DataFrame back into another table
    from_dataframe(&conn, "t_copy", &df)?;

    Ok(())
}
```

```
shape: (2, 2)
┌─────┬───────┐
│ id  ┆ name  │
│ --- ┆ ---   │
│ i64 ┆ str   │
╞═════╪═══════╡
│ 1   ┆ Alice │
│ 2   ┆ Bob   │
└─────┴───────┘
```

## Type system

Note that the type system used by `rusqlite` via ConnectorX is as shown
[here](https://github.com/sfu-db/connector-x/blob/d57428c56b99fb8de40f1226ce0388fc1338e3b2/connectorx/src/sources/sqlite/typesystem.rs)

## Integration

- Use this library in Rust projects that need to bridge SQLite and Polars.
- For a quick playground, see the [CLI](https://github.com/lmmx/polite/tree/master/polite-cli).

## Documentation

- **Crate docs**: [docs.rs/polite](https://docs.rs/polite)
- **Workspace guide**: [DEVELOPMENT.md](https://github.com/lmmx/polite/blob/master/DEVELOPMENT.md)

## License

Licensed under the MIT License.
See [LICENSE](https://github.com/lmmx/polite/blob/master/LICENSE) for details.
