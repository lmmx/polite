use connectorx::prelude::*;
use polars::prelude::*;
use rusqlite::Connection;
use std::convert::TryFrom;

/// Run a query through ConnectorX and get a Polars DataFrame
pub fn to_dataframe(db_path: &str, sql: &str) -> PolarsResult<DataFrame> {
    let uri = format!("sqlite://{}", db_path);
    let source = SourceConn::try_from(uri.as_str())
        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;

    let queries = &[CXQuery::from(sql)];

    // With `dst_polars` feature enabled, this gives you a single DataFrame directly
    let df = get_arrow(&source, None, queries, None)
        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
        .polars()
        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;

    Ok(df)
}

/// Insert a Polars DataFrame into a SQLite table.
/// Creates the table if it does not exist.
pub fn from_dataframe(conn: &Connection, table: &str, df: &DataFrame) -> rusqlite::Result<()> {
    // Build CREATE TABLE statement
    let mut cols_sql = Vec::new();
    for (name, dtype) in df.get_columns().iter().map(|s| (s.name(), s.dtype())) {
        let sql_type = match dtype {
            DataType::Int64 => "INTEGER",
            DataType::Float64 => "REAL",
            DataType::String => "TEXT",
            _ => "TEXT", // fallback
        };
        cols_sql.push(format!("{} {}", name, sql_type));
    }
    let create_stmt = format!(
        "CREATE TABLE IF NOT EXISTS {} ({})",
        table,
        cols_sql.join(", ")
    );
    conn.execute(&create_stmt, [])?;

    // Build INSERT statement
    let placeholders: Vec<String> = (0..df.width()).map(|_| "?".to_string()).collect();
    let insert_stmt = format!("INSERT INTO {} VALUES ({})", table, placeholders.join(", "));
    let mut insert = conn.prepare(&insert_stmt)?;

    // Insert each row
    for row_idx in 0..df.height() {
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        for series in df.get_columns() {
            let val = match series.dtype() {
                DataType::Int64 => series
                    .i64()
                    .unwrap()
                    .get(row_idx)
                    .map(|v| v.into())
                    .unwrap_or(rusqlite::types::Value::Null),
                DataType::Float64 => series
                    .f64()
                    .unwrap()
                    .get(row_idx)
                    .map(|v| v.into())
                    .unwrap_or(rusqlite::types::Value::Null),
                DataType::String => series
                    .str()
                    .unwrap()
                    .get(row_idx)
                    .map(|v| v.to_string().into())
                    .unwrap_or(rusqlite::types::Value::Null),
                _ => rusqlite::types::Value::Null,
            };
            values.push(val);
        }
        insert.execute(rusqlite::params_from_iter(values))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::fs;

    #[test]
    fn test_to_dataframe_sqlite() {
        let db_path = "test.db";

        // Prepare SQLite DB using rusqlite
        let conn = Connection::open(db_path).unwrap();
        conn.execute("DROP TABLE IF EXISTS t", []).unwrap();
        conn.execute("CREATE TABLE t (id INTEGER, name TEXT)", [])
            .unwrap();
        conn.execute("INSERT INTO t VALUES (1, 'Alice')", [])
            .unwrap();

        // Use ConnectorX to read it into a DataFrame
        let df = to_dataframe(db_path, "SELECT * FROM t").unwrap();

        // Assert shape and values
        assert_eq!(df.shape(), (1, 2)); // 1 row, 2 cols
        let name_col = df.column("name").unwrap();
        let val = name_col.str().unwrap().get(0).unwrap();
        assert_eq!(val, "Alice");

        // Cleanup
        fs::remove_file(db_path).unwrap();
    }
}
