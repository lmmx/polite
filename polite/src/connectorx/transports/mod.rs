//! This module contains transport definitions for the sources and destinations implemented in ConnectorX.

// #[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
mod sqlite_arrow;
// #[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
mod sqlite_arrowstream;
// #[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
pub use sqlite_arrow::{SQLiteArrowTransport, SQLiteArrowTransportError};
// #[cfg(all(feature = "src_sqlite", feature = "dst_arrow"))]
pub use sqlite_arrowstream::SQLiteArrowTransport as SQLiteArrowStreamTransport;
