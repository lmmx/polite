//! # Polite: SQLite × Polars Integration
//!
//! A Rust library that provides seamless integration between SQLite databases
//! and Polars DataFrames using ConnectorX for efficient data transfer.
//!
//! ## Features
//!
//! - **Fast data loading**: Use ConnectorX to efficiently load SQLite query results into Polars DataFrames
//! - **Easy data writing**: Save Polars DataFrames directly to SQLite tables
//! - **Type preservation**: Automatic mapping between Polars and SQLite data types
//! - **Simple API**: Minimal boilerplate for common database operations
//!
//! ## Quick Start
//!
//! ```rust
//! use polite::{connect_sqlite, to_dataframe, from_dataframe};
//! use polars::prelude::*;
//!
//! // Open a SQLite connection
//! let conn = connect_sqlite(Some("data.db")).unwrap();
//!
//! // Load data from SQLite into a DataFrame
//! let df = to_dataframe("data.db", "SELECT * FROM users").unwrap();
//!
//! // Save a DataFrame to SQLite
//! from_dataframe(&conn, "new_table", &df).unwrap();
//! ```
//!
//! ## Modules
//!
//! - [`dataframe`] - Functions for converting between DataFrames and SQLite
//! - [`db`] - Database connection utilities

pub mod dataframe;
pub mod db;

// Re-export the main entrypoints at crate root
pub use dataframe::{from_dataframe, to_dataframe};
pub use db::{connect_sqlite, execute_query};

/// Common imports for polite users.
///
/// This prelude module re-exports the most commonly used items from polite,
/// allowing users to quickly import everything they need with a single glob import.
///
/// # Examples
///
/// ```rust
/// use polite::prelude::*;
///
/// // Now you have access to all the main functions:
/// let conn = connect_sqlite(Some("data.db")).unwrap();
/// let df = to_dataframe("data.db", "SELECT * FROM users").unwrap();
/// from_dataframe(&conn, "backup_users", &df).unwrap();
/// ```
pub mod prelude {
    pub use crate::{connect_sqlite, execute_query, from_dataframe, to_dataframe};

    // If you add the convenience functions, include them too:
    pub use crate::{load_dataframe, save_dataframe};
}

/// Create a DataFrame from a SQLite file with error handling and logging.
///
/// This convenience function provides better error messages and optional logging
/// compared to the raw `to_dataframe` function.
///
/// # Arguments
///
/// * `db_path` - Path to the SQLite database file
/// * `sql` - SQL query to execute
///
/// # Examples
///
/// ```rust
/// use polite::load_dataframe;
///
/// let df = load_dataframe("data.db", "SELECT id, name FROM users LIMIT 10")
///     .expect("Failed to load data");
///
/// println!("Loaded {} rows", df.height());
/// ```
pub fn load_dataframe(db_path: &str, sql: &str) -> Result<polars::prelude::DataFrame, String> {
    to_dataframe(db_path, sql)
        .map_err(|e| format!("Failed to load DataFrame from {}: {}", db_path, e))
}

/// Save a DataFrame to SQLite with automatic table creation and better error handling.
///
/// This convenience function handles connection creation and provides clearer error messages.
///
/// # Arguments
///
/// * `db_path` - Path to the SQLite database file (will be created if it doesn't exist)
/// * `table_name` - Name of the table to create/insert into
/// * `df` - The DataFrame to save
///
/// # Examples
///
/// ```rust
/// use polite::save_dataframe;
/// use polars::prelude::*;
///
/// let df = df! {
///     "id" => [1, 2, 3],
///     "name" => ["Alice", "Bob", "Charlie"],
/// }.unwrap();
///
/// save_dataframe("output.db", "users", &df)
///     .expect("Failed to save DataFrame");
/// ```
pub fn save_dataframe(
    db_path: &str,
    table_name: &str,
    df: &polars::prelude::DataFrame,
) -> Result<(), String> {
    let conn = connect_sqlite(Some(db_path))
        .map_err(|e| format!("Failed to connect to {}: {}", db_path, e))?;

    from_dataframe(&conn, table_name, df)
        .map_err(|e| format!("Failed to save DataFrame to table '{}': {}", table_name, e))?;

    Ok(())
}
