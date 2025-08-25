# polite

[![crates.io](https://img.shields.io/crates/v/polite.svg)](https://crates.io/crates/polite)
[![Documentation](https://docs.rs/polite/badge.svg)](https://docs.rs/polite)
[![MIT licensed](https://img.shields.io/crates/l/polite.svg)](https://github.com/lmmx/polite/blob/master/LICENSE)

The core **rusqlite × Polars bridge**.  
`polite` makes it easy to move data between SQLite databases and Polars `DataFrame`s.

## Features

- Open SQLite databases (file-based or in-memory).
- Execute arbitrary SQL statements.
- Convert query results into Polars `DataFrame`s (`to_dataframe`).
- Write Polars `DataFrame`s into SQLite tables (`from_dataframe`).

### Current Limitations (MVP)

- Supported column types: `INTEGER`, `REAL`, `TEXT` → mapped to Polars `Int64`, `Float64`, `Utf8`.
- Other SQLite types are stored as text (`Utf8`).
- DataFrame output is shown in Polars’ debug format when used via the CLI.
- No schema evolution or type inference beyond the basics (yet).

Absolutely 👌 — that’s a perfect kind of “sharp edge” to call out for users. Here’s a small insert you could add to your **core library README** under *Features* or *Limitations*:

#### ⚠️ Notes on SQLite Backends

* `polite` uses **ConnectorX** under the hood for bulk reads into Polars `DataFrame`s.
* **File-backed databases** (`.sqlite`, `.db`, etc.) work out of the box.
* **In-memory databases** (`:memory:`) are **not supported by ConnectorX** — in that case, `polite` falls back to a slower row-by-row implementation using `rusqlite`.

If you need consistent behavior across both modes, prefer using a temporary file-backed database in tests or workflows.

## Example

```rust
use polite::{connect_sqlite, execute_query, to_dataframe, from_dataframe};
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    // Open an in-memory database
    let conn = connect_sqlite(None)?;

    // Create and populate a table
    execute_query(&conn, "CREATE TABLE t (id INTEGER, name TEXT)")?;
    execute_query(&conn, "INSERT INTO t VALUES (1, 'Alice')")?;

    // Query back into a DataFrame
    let df = to_dataframe(&conn, "SELECT * FROM t")?;
    println!("{:?}", df);

    // Write DataFrame back into another table
    from_dataframe(&conn, "t_copy", &df)?;

    Ok(())
}
````

## Integration

* Use this library in Rust projects that need to bridge SQLite and Polars.
* For a quick playground, try the CLI:
  [polite-cli](https://github.com/lmmx/polite/tree/master/polite-cli)

## Documentation

* **Crate docs**: [docs.rs/polite](https://docs.rs/polite)
* **Workspace guide**: [DEVELOPMENT.md](https://github.com/lmmx/polite/blob/master/DEVELOPMENT.md)

## License

Licensed under the MIT License.
See [LICENSE](https://github.com/lmmx/polite/blob/master/LICENSE) for details.
