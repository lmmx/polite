use crate::PoliteError;
use connectorx::errors::ConnectorXError;
use connectorx::prelude::*;
use polars::prelude::*;
use rusqlite::types::Value;
use rusqlite::Connection as SqliteConn;
use std::convert::TryFrom;

fn save_err(db_path: &str, table: &str, e: rusqlite::Error) -> PoliteError {
    PoliteError::Save {
        db_path: db_path.to_string(),
        table_name: table.to_string(),
        source: e,
    }
}

/// Run a query through ConnectorX and get a Polars DataFrame
pub fn to_dataframe(db_path: &str, sql: &str) -> Result<DataFrame, PoliteError> {
    // Preflight check: validate query with SQLite first
    let preflight = SqliteConn::open(db_path).map_err(|e| PoliteError::Connect {
        db_path: db_path.to_string(),
        source: e,
    })?;

    if let Err(e) = preflight.prepare(sql) {
        return Err(PoliteError::Query {
            db_path: db_path.to_string(),
            source: ConnectorXError::SqlQueryNotSupported(e.to_string()),
        });
    }

    // ConnectorX connection
    let conn = SourceConn::try_from(format!("sqlite://{}", db_path).as_str()).map_err(|e| {
        PoliteError::Query {
            db_path: db_path.to_string(),
            source: e,
        }
    })?;

    let queries = &[CXQuery::from(sql)];

    // Fetch Arrow batches
    let arrow = get_arrow(&conn, None, queries, None).map_err(|e| PoliteError::Arrow {
        db_path: db_path.to_string(),
        source: e,
    })?;

    // Convert Arrow → Polars
    let df = arrow
        .polars()
        .map_err(|e| PoliteError::ArrowToPolars { source: e })?;

    Ok(df)
}

/// Insert a Polars DataFrame into a SQLite table.
/// Creates the table if it does not exist.
pub fn from_dataframe(
    conn: &rusqlite::Connection,
    table: &str,
    df: &DataFrame,
) -> Result<(), PoliteError> {
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
    conn.execute(&create_stmt, [])
        .map_err(|e| save_err("<connection>", table, e))?;

    // Build INSERT statement
    let placeholders: Vec<String> = (0..df.width()).map(|_| "?".to_string()).collect();
    let insert_stmt = format!("INSERT INTO {} VALUES ({})", table, placeholders.join(", "));
    let mut insert = conn
        .prepare(&insert_stmt)
        .map_err(|e| save_err("<connection>", table, e))?;

    // Insert each row
    for row_idx in 0..df.height() {
        let mut values: Vec<Value> = Vec::new();
        for series in df.get_columns() {
            let val = match series.dtype() {
                DataType::Int64 => series
                    .i64()
                    .unwrap()
                    .get(row_idx)
                    .map(|v| v.into())
                    .unwrap_or(Value::Null),
                DataType::Float64 => series
                    .f64()
                    .unwrap()
                    .get(row_idx)
                    .map(|v| v.into())
                    .unwrap_or(Value::Null),
                DataType::String => series
                    .str()
                    .unwrap()
                    .get(row_idx)
                    .map(|v| v.to_string().into())
                    .unwrap_or(Value::Null),
                _ => Value::Null,
            };
            values.push(val);
        }
        insert
            .execute(rusqlite::params_from_iter(values))
            .map_err(|e| save_err("<connection>", table, e))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_to_dataframe_sqlite() {
        // Create a temporary file-backed SQLite DB
        let db = NamedTempFile::new().unwrap();
        let db_path = db.path().to_str().unwrap();

        // Prepare SQLite DB using rusqlite
        let conn = rusqlite::Connection::open(db_path).unwrap();
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
    }
}
