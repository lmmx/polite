//! Core utilities for working with rusqlite × Polars

pub mod dataframe;
pub mod db;

// Re-export the main entrypoints
pub use dataframe::{from_dataframe, to_dataframe};
pub use db::{connect_sqlite, execute_query};
