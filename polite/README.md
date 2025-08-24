# polite

[![crates.io](https://img.shields.io/crates/v/polite.svg)](https://crates.io/crates/polite)
[![Documentation](https://docs.rs/polite/badge.svg)](https://docs.rs/polite)
[![MIT licensed](https://img.shields.io/crates/l/polite.svg)](https://github.com/lmmx/polite/blob/master/LICENSE)

The core **rusqlite × Polars bridge**. Provides conversion utilities between SQLite databases and Polars DataFrames.

## Features

- Open SQLite databases (file or in-memory)
- Run SQL statements
- Convert query results into Polars `DataFrame`s
- Write Polars `DataFrame`s into SQLite tables

## Example

```rust
use polite::{connect_sqlite, execute_query, to_dataframe};
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let conn = connect_sqlite(None)?; // in-memory DB

    execute_query(&conn, "CREATE TABLE t (id INTEGER, name TEXT)")?;
    execute_query(&conn, "INSERT INTO t VALUES (1, 'Alice')")?;

    let df = to_dataframe(&conn, "SELECT * FROM t")?;
    println!("{:?}", df);

    Ok(())
}
````

## Integration

* Use this library in Rust projects that need to bridge SQLite and Polars seamlessly.
* For a quick playground, see the companion CLI: [polite-cli](https://github.com/lmmx/polite/tree/master/polite-cli).

## Documentation

* **Crate docs**: [docs.rs/polite](https://docs.rs/polite)
* **Workspace guide**: [DEVELOPMENT.md](https://github.com/lmmx/polite/blob/master/DEVELOPMENT.md)

## License

Licensed under the MIT License. See [LICENSE](https://github.com/lmmx/polite/blob/master/LICENSE) for details.
