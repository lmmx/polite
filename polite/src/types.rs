use polars::prelude::{DataType, Schema};
use rusqlite::Statement;

/// Polars Schema from SQLite query (parsed from a string by `Connection::prepare`)
pub(crate) fn schema_from_sqlite(stmt: &Statement) -> Schema {
    Schema::from_iter(stmt.columns().iter().map(|col| {
        let name = col.name();
        let decl_type = col.decl_type();

        let dtype = match decl_type.as_deref() {
            Some("INTEGER") => DataType::Int64,
            Some("REAL") => DataType::Float64,
            Some("TEXT") => DataType::String,
            _ => DataType::String,
        };

        (name.into(), dtype)
    }))
}
