use polars::prelude::*;
use rusqlite::Connection;

/// Helper to map rusqlite errors into Polars errors
fn to_polars_err(err: rusqlite::Error) -> PolarsError {
    PolarsError::ComputeError(err.to_string().into())
}

/// Convert a SQLite query result into a Polars DataFrame
///
/// ⚠️ Currently returns an empty DataFrame as a placeholder.
/// Replace row extraction and Series construction with real logic.
pub fn to_dataframe(conn: &Connection, sql: &str) -> PolarsResult<DataFrame> {
    let mut stmt = conn.prepare(sql).map_err(to_polars_err)?;
    let mut rows = stmt.query([]).map_err(to_polars_err)?;

    // TODO: actually read rows into columns
    while let Some(_row) = rows.next().map_err(to_polars_err)? {
        // Placeholder: you would extract values here
    }

    Ok(DataFrame::empty())
}

/// Insert a Polars DataFrame into a SQLite table
///
/// ⚠️ Currently a stub — implement schema derivation and INSERTs.
pub fn from_dataframe(conn: &Connection, table: &str, df: &DataFrame) -> rusqlite::Result<()> {
    let _ = (conn, table, df); // silence unused vars
    Ok(())
}
