use rusqlite::{Connection, Result};

/// Open a SQLite connection (in-memory by default)
pub fn connect_sqlite(path: Option<&str>) -> Result<Connection> {
    match path {
        Some(p) => Connection::open(p),
        None => Connection::open_in_memory(),
    }
}

/// Run a SQL query against a connection
pub fn execute_query(conn: &Connection, sql: &str) -> Result<usize> {
    conn.execute(sql, [])
}
