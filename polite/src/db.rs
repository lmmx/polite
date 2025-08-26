use crate::PoliteError;
use rusqlite::Connection;

/// Open a SQLite connection
pub fn connect_sqlite(path: Option<&str>) -> Result<Connection, PoliteError> {
    let db_path = path.unwrap_or(":memory:"); // default if None is passed
    Connection::open(db_path).map_err(|e| PoliteError::Connect {
        db_path: db_path.to_string(),
        source: e,
    })
}

/// Run a SQL query against a connection
pub fn execute_query(conn: &Connection, sql: &str) -> Result<usize, PoliteError> {
    conn.execute(sql, []).map_err(|e| PoliteError::Exec {
        sql: sql.to_string(),
        source: e,
    })
}
