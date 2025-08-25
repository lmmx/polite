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

## Example

```rust
use polite::{connect_sqlite, execute_query, from_dataframe, to_dataframe};
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
