# polite

[![crates.io](https://img.shields.io/crates/v/polite.svg)](https://crates.io/crates/polite)
[![Documentation](https://docs.rs/polite/badge.svg)](https://docs.rs/polite)
[![MIT licensed](https://img.shields.io/crates/l/polite)](https://github.com/lmmx/polite/blob/master/LICENSE)

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

💡 **All of these functions are also available via `use polite::prelude::*;`**

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

### Why use these helpers?

These helpers don’t add new capabilities beyond the core API, but they provide more ergonomic errors.

The raw API (`to_dataframe`, `from_dataframe`) exposes detailed error variants (`Query`, `Arrow`, `Polars`, `rusqlite`, etc.), which is useful if you want to distinguish exactly what failed.

The convenience wrappers (`load_dataframe`, `save_dataframe`) normalize those into a **single error variant per operation**:

- ✅ They normalize errors:
    - `load_dataframe` always yields `PoliteError::Load`
    - `save_dataframe` always yields `PoliteError::Save`.
- ✅ You don’t have to juggle `Query`, `Arrow`, `ArrowToPolars` variants of `PoliteError`, `rusqlite::Error` etc.
- ✅ They’re the "safe default" for people who just want “load/save a DataFrame” and don’t care which stage failed.
- ✅ Advanced users can drop down to `to_dataframe` / `from_dataframe` for finer control and granular error inspection.

In practice, wrappers are the **recommended default** for most use cases. Drop down to the raw API when you want maximum control.

## 🎤 Demo time

```rust
use polite::prelude::*;
use polars::prelude::*;

fn main() -> anyhow::Result<(), String> {
    // Open (or create) a SQLite database
    let db_path = "polite.db";
    let conn = connect_sqlite(Some(db_path))?;

    execute_query(&conn, "CREATE TABLE friends_made (id INTEGER, name TEXT)")?;

    let nobody = load_dataframe(db_path, "SELECT * FROM friends_made")?;
    println!("🤓 I am making friends in SQLite! I don't have any there yet...\n{nobody:?}");

    // Create a table to keep your friends' names in
    execute_query(&conn, "INSERT INTO friends_made VALUES (1, 'Alice')")?;
    execute_query(&conn, "INSERT INTO friends_made VALUES (2, 'Bob')")?;
    execute_query(&conn, "INSERT INTO friends_made VALUES (3, 'Charlie')")?;

    // Query your friends back into a Polars DataFrame
    let dbf = to_dataframe(db_path, "SELECT * FROM friends_made")?;
    println!("🪄 I have lovingly restored my friends into a Polars DataFrame:\n{dbf:?}");

    // Add some more friends directly from a Polars DataFrame
    let polars_friends = df! {
        "id" => [4, 5],
        "name" => ["Dora", "Eve"],
    }?;
    
    from_dataframe(&conn, "cool_friends", &polars_friends)?;

    println!("🆒 My friends from Polars are now my friends in SQLite:\n{polars_friends:?}");

    let all_friends = load_dataframe(
        db_path,
        "SELECT * FROM friends_made UNION ALL SELECT * FROM cool_friends ORDER BY id",
    )?;
    println!("🎉 All my friends are politely gathered in a DataFrame:\n{all_friends:?}");

    Ok(())
}
```

```
🤓 I am making friends in SQLite! I don't have any there yet...
shape: (0, 2)
┌─────┬──────┐
│ id  ┆ name │
│ --- ┆ ---  │
│ str ┆ str  │
╞═════╪══════╡
└─────┴──────┘
🪄 I have lovingly restored my friends into a Polars DataFrame:
shape: (3, 2)
┌─────┬─────────┐
│ id  ┆ name    │
│ --- ┆ ---     │
│ i64 ┆ str     │
╞═════╪═════════╡
│ 1   ┆ Alice   │
│ 2   ┆ Bob     │
│ 3   ┆ Charlie │
└─────┴─────────┘
🆒 My friends from Polars are now my friends in SQLite:
shape: (2, 2)
┌─────┬──────┐
│ id  ┆ name │
│ --- ┆ ---  │
│ i32 ┆ str  │
╞═════╪══════╡
│ 4   ┆ Dora │
│ 5   ┆ Eve  │
└─────┴──────┘
🎉 All my friends are politely gathered in a DataFrame:
shape: (5, 2)
┌──────┬─────────┐
│ id   ┆ name    │
│ ---  ┆ ---     │
│ i64  ┆ str     │
╞══════╪═════════╡
│ null ┆ Dora    │
│ null ┆ Eve     │
│ 1    ┆ Alice   │
│ 2    ┆ Bob     │
│ 3    ┆ Charlie │
└──────┴─────────┘
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
