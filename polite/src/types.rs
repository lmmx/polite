use rusqlite::types::Type;
use connectorx::sources::sqlite::typesystem::SQLiteTypeSystem;

fn schema_from_sqlite(conn: &rusqlite::Connection, table: &str) -> rusqlite::Result<Vec<(String, SQLiteTypeSystem)>> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let rows = stmt.query_map([], |row| {
        let name: String = row.get(1)?;   // column name
        let decl: Option<String> = row.get(2)?;  // declared type, may be NULL
        // fall back to NULL type if not present
        let ts = SQLiteTypeSystem::try_from((decl.as_deref(), Type::Null))
            .unwrap_or(SQLiteTypeSystem::Text(true));
        Ok((name, ts))
    })?;
    let mut schema = Vec::new();
    for r in rows {
        schema.push(r?);
    }
    Ok(schema)
}
