#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! polite = "0.1.1"
//! anyhow = "*"
//! ```

use polite::prelude::*;

fn main() -> anyhow::Result<()> {
    // Create (or open) a SQLite database file
    let db_path = "demo.db";
    let conn = connect_sqlite(Some(db_path))?;

    // Create and populate a table
    execute_query(&conn, "CREATE TABLE IF NOT EXISTS users (id INTEGER, name TEXT)")?;
    execute_query(&conn, "INSERT INTO users VALUES (1, 'Alice')")?;
    execute_query(&conn, "INSERT INTO users VALUES (2, 'Bob')")?;

    // Query into a Polars DataFrame
    let df = to_dataframe(db_path, "SELECT * FROM users")?;
    println!("Loaded DataFrame:\n{df:?}");

    // Write it back into a new table
    from_dataframe(&conn, "users_copy", &df)?;
    println!("Saved DataFrame into table `users_copy`");

    Ok(())
}
