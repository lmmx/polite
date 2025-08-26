use thiserror::Error;

#[derive(Debug, Error)]
pub enum PoliteError {
    #[error("ConnectorX Arrow conversion failed on {db_path}: {source}")]
    Arrow {
        db_path: String,
        #[source]
        source: connectorx::errors::ConnectorXOutError,
    },

    #[error("Arrow → Polars conversion failed: {source}")]
    ArrowToPolars {
        #[source]
        source: connectorx::destinations::arrow::ArrowDestinationError,
    },

    #[error("Failed to connect to {db_path}: {source}")]
    Connect {
        db_path: String,
        #[source]
        source: rusqlite::Error,
    },

    #[error("Failed to convert Arrow to DataFrame: {source}")]
    DataFrame {
        #[source]
        source: polars::error::PolarsError,
    },

    #[error("Failed to execute SQL: {sql}: {source}")]
    Exec {
        sql: String,
        #[source]
        source: rusqlite::Error,
    },

    #[error("Failed to load DataFrame from {db_path}: {source}")]
    Load {
        db_path: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>, // allow wrapping any error
    },

    #[error("Failed to run query on {db_path}: {source}")]
    Query {
        db_path: String,
        #[source]
        source: connectorx::errors::ConnectorXError,
    },

    #[error("Failed to save DataFrame to table '{table_name}' in {db_path}: {source}")]
    Save {
        db_path: String,
        table_name: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>, // now matches Load variant
    },

    #[error("SQLite error: {source}")]
    Sqlite {
        #[source]
        source: rusqlite::Error,
    },
}
