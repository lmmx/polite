use connectorx::prelude::*;
use polars::prelude::*;
use rusqlite::Connection;

/// Run a query and collect results into a Polars DataFrame
pub fn to_dataframe(conn: &Connection, sql: &str) -> PolarsResult<DataFrame> {
    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
    let column_count = stmt.column_count();
    let column_names: Vec<String> = (0..column_count)
        .map(|i| stmt.column_name(i).unwrap_or("").to_string())
        .collect();

    let mut cols: Vec<Vec<Option<String>>> = vec![Vec::new(); column_count];
    let mut rows = stmt
        .query([])
        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?;
    while let Some(row) = rows
        .next()
        .map_err(|e| PolarsError::ComputeError(e.to_string().into()))?
    {
        for (i, col) in cols.iter_mut().enumerate() {
            let v: Result<Option<String>, _> = row.get(i);
            col.push(v.unwrap_or(None));
        }
    }

    // Build all columns as Utf8 for now
    let mut series = Vec::with_capacity(column_count);
    for (name, col) in column_names.into_iter().zip(cols.into_iter()) {
        let parsed: Vec<Option<&str>> = col.iter().map(|opt| opt.as_deref()).collect();
        series.push(Series::new(name.into(), parsed).into());
    }

    DataFrame::new(series)
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

/// Query a SQLite database into a Polars DataFrame using ConnectorX
pub fn cx_query_sqlite(db_path: &str, sql: &str) -> PolarsResult<DataFrame> {
    // SQLite connection string (use `:memory:` for in-memory)
    let conn_str = format!("sqlite://{}", db_path);

    // Run the query via ConnectorX
    let df: DataFrame = load(&conn_str, sql)
        .map_err(|e| PolarsError::ComputeError(format!("ConnectorX error: {}", e).into()))?;

    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_cx_query_sqlite() {
        let db_path = "test.db";
        let conn = rusqlite::Connection::open(db_path).unwrap();
        conn.execute("DROP TABLE IF EXISTS t", []).unwrap();
        conn.execute("CREATE TABLE t (id INTEGER, name TEXT)", [])
            .unwrap();
        conn.execute("INSERT INTO t VALUES (1, 'Alice')", [])
            .unwrap();

        let df = cx_query_sqlite(db_path, "SELECT * FROM t").unwrap();
        assert_eq!(df.shape().0, 1);
        assert!(df
            .column("name")
            .unwrap()
            .utf8()
            .unwrap()
            .get(0)
            .unwrap()
            .contains("Alice"));

        fs::remove_file(db_path).unwrap();
    }
}
