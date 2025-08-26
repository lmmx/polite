#![allow(clippy::upper_case_acronyms)]

//! # ConnectorX
//!
//! ConnectorX enables you to load data from databases into dataframes in the fastest and most memory efficient way by leveraging
//! zero-copy and partition-based parallelism.
//!
//! Currently, ConnectorX consists of a Rust core library and a python library. This is the documentation for the Rust crate.
//! For the documentation of the Python library, please refer to our [Github Readme](https://github.com/sfu-db/connector-x).
//!
//! # Design
//!
//! A data loading problem consists of three sub-problems:
//! 1. How to connect to the data source and read data.
//! 2. How to connect to the data destination and write data.
//! 3. How to map the types between the source and destination.
//!
//! Additionally, since ConnectorX will partition a query into partitions and execute them in parallel, we also have
//! 4. How to partition the query and run them in parallel.
//!
//! ConnectorX approaches these problems by defining abstractions on sources, destinations, and mapping rules.
//! For the partition-based parallelism, ConnectorX will partition the query as well as the source and the destination
//! together and put them into threads.
//! Each thread will own exactly 1 query, 1 partitioned source, and 1 partitioned destination.
//!
//! The following graph depicts the internal mechanism when ConnectorX is downloading the data.
//!
//! ```text
//!                     +------------------------------------------------------------+
//!                     |                           Thread 1                         |
//!                     |                                                            |
//!     +---+           | +-----------------+   +-------------+  +-----------------+ |          +---+
//!     |   +-----------+>| Partitioned Src +-->| Type Mapper +->| Partitioned Dst +-+--------->|   |
//!     |   |           | +-----------------+   +-------------+  +-----------------+ |          |   |
//!     | D |           |                                                            |          | D |
//!     | a |           +------------------------------------------------------------+          | a |
//!     | t |                                          .                                        | t |
//!     | a |                                          .                                        | a |
//!     | b |                                          .                                        | f |
//!     | a |           +------------------------------------------------------------+          | r |
//!     | s |           |                           Thread n                         |          | a |
//!     | e |           |                                                            |          | m |
//!     |   |           | +-----------------+   +-------------+  +-----------------+ |          | e |
//!     |   +-----------+>| Partitioned Src +-->| Type Mapper +->| Partitioned Dst +-+--------->|   |
//!     +---+           | +-----------------+   +-------------+  +-----------------+ |          +---+
//!                     |                                                            |
//!                     +------------------------------------------------------------+
//!
//! ```
//! ## How does ConnectorX download the data?
//!
//! Upon receiving the query, e.g. SELECT * FROM lineitem, ConnectorX will first issue a LIMIT 1 query SELECT * FROM lineitem LIMIT 1 to get the schema of the result set.
//!
//! Then, if partition_on is specified, ConnectorX will issue `SELECT MIN($partition_on), MAX($partition_on) FROM (SELECT * FROM lineitem)` to know the range of the partition column.
//! After that, the original query is split into partitions based on the min/max information, e.g. `SELECT * FROM (SELECT * FROM lineitem) WHERE $partition_on > 0 AND $partition_on < 10000`.
//! ConnectorX will then run a count query to get the partition size (e.g. `SELECT COUNT(*) FROM (SELECT * FROM lineitem) WHERE $partition_on > 0 AND $partition_on < 10000`).
//! If the partition is not specified, the count query will be `SELECT COUNT(*) FROM (SELECT * FROM lineitem)`.
//!
//! Finally, ConnectorX will use the schema info as well as the count info to allocate memory and download data by executing the queries normally.
//! Once the downloading begins, there will be one thread for each partition so that the data are downloaded in parallel at the partition level.
//! The thread will issue the query of the corresponding partition to the database and then write the returned data to the destination row-wise or column-wise (depends on the database) in a streaming fashion.
//! This mechanism implies that having an index on the partition column is recommended to make full use of the parallel downloading power provided by ConnectorX.
//!
//! # Extending ConnectorX
//! ## Adding a new source
//!
//! To add a new data source, you need to implement [`sources::Source`], [`sources::SourcePartition`], [`sources::PartitionParser`], and [`sources::Produce`] for the source.
//! In detail, [`sources::Source`] describes how to connect to the database from a connection string, as well as how to do partitioning on the source to produce a list of [`sources::SourcePartition`].
//! [`sources::SourcePartition`] describes how to get the row count for the specific partition so that the destination can preallocate the memory.
//! Finally, [`sources::PartitionParser`] and [`sources::Produce`] abstracts away the detail about how does each partition parse different types.
//!
//! ## Adding a new destination
//!
//! To add a new data destination, you need to implement [`destinations::Destination`], [`destinations::DestinationPartition`], and [`destinations::Consume`]. Similar to the sources,
//! [`destinations::Destination`] describes how to allocate the memory of the data destination, as well as how to do partitioning on the destination to produce a list of [`destinations::DestinationPartition`].
//! [`destinations::DestinationPartition`] and [`destinations::Consume`] abstract away the detail about how does each partition writes different types.
//!
//! ## Adding a new transport (type mapping)
//!
//! After having a source and a destination that describes how to read and write the data,
//! ConnectorX also needs to know how to convert the values with different types from the source to the destination.
//! For example, Postgres can produce a `uuid` type but there's no uuid in Arrow. It is the transport's duty to convert
//! the `uuid` into an Arrow compatible type, e.g. string. You can use the [`impl_transport!`] macro to define a transport.
//!
//! ## Putting things together
//!
//! Say, you decide to load data from SQL Server to Arrow. In ConnectorX we already provided the source for SQL Server as [`sources::sqlite::SQLiteSource`], and the
//! Arrow destination [`destinations::arrow::ArrowDestination`], as well as the transport [`transports::SQLiteArrowTransport`].
//! Given the source, destination and transport already implemented, you can use [`dispatcher::Dispatcher`] to load the data:
//!
//! ```no_run
//! use crate::connectorx::prelude::*;
//!
//! let mut destination = ArrowDestination::new();
//! let source = SQLiteSource::new("/path/to/db", 10).expect("cannot create the source");
//! let queries = &["SELECT * FROM db WHERE id < 100", "SELECT * FROM db WHERE id >= 100"];
//! let dispatcher = Dispatcher::<SQLiteSource, ArrowDestination, SQLiteArrowTransport>::new(source, &mut destination, queries, None);
//! dispatcher.run().expect("run failed");
//!
//! let data = destination.arrow();
//! ```
//!
//! Or simply you can directly use the [`get_arrow::get_arrow`] in which we wrapped the above procedures:
//!
//! ```no_run
//! use crate::connectorx::prelude::*;
//! use std::convert::TryFrom;
//!
//! let mut source_conn = SourceConn::try_from("postgresql://username:password@host:port/db?cxprotocol=binary").expect("parse conn str failed");
//! let queries = &[CXQuery::from("SELECT * FROM table WHERE id < 100"), CXQuery::from("SELECT * FROM table WHERE id >= 100")];
//! let destination = get_arrow(&source_conn, None, queries).expect("run failed");
//!
//! let data = destination.arrow();
//! ```
//!
//! NOTE: the pool size parameter `nconn` used in initializing the source should be larger than or equal to the number of partitioned queries input later.
//!
//! ## Need more examples?
//! You can use the existing implementation as the example.
//! [MySQL source](https://github.com/sfu-db/connector-x/tree/main/connectorx/src/sources/mysql),
//! [Arrow destination](https://github.com/sfu-db/connector-x/tree/main/connectorx/src/destinations/arrow),
//! [MySQL to Arrow transport](https://github.com/sfu-db/connector-x/blob/main/connectorx/src/transports/mysql_arrow.rs).
//!
//! # Sources protocols & Destinations that is implemented in the Rust core.
//!
//! ## Sources
//! - [x] Postgres
//! - [x] Mysql
//! - [x] Sqlite
//! - [x] SQL Server
//! - [x] Oracle
//! - [x] BigQuery
//!
//! ## Destinations
//! - [x] Arrow
//! - [x] Polars
//!
//! # Feature gates
//! By default, ConnectorX does not enable any sources / destinations to keep the dependencies minimal.
//! Instead, we provide following features for you to opt-in: `src_sqlite`, `src_postgres`, `src_mysql`, `src_mssql`, `src_oracle`, `dst_arrow`, `dst_polars`.
//! For example, if you'd like to load data from Postgres to Arrow, you can enable `src_postgres` and `dst_arrow` in `Cargo.toml`.
//! This will enable [`sources::postgres`], [`destinations::arrow`] and [`transports::PostgresArrowTransport`].

pub mod typesystem;
#[macro_use]
mod macros;
// #[cfg(feature = "dst_arrow")]
pub mod arrow_batch_iter;
pub mod constants;
pub mod data_order;
pub mod destinations;
mod dispatcher;
pub mod errors;
// #[cfg(feature = "dst_arrow")]
pub mod get_arrow;
pub mod partition;
pub mod source_router;
pub mod sources;
#[doc(hidden)]
pub mod sql;
pub mod transports;
#[doc(hidden)]
pub mod utils;

pub mod prelude {
    // #[cfg(feature = "dst_arrow")]
    pub use crate::connectorx::arrow_batch_iter::RecordBatchIterator;
    pub use crate::connectorx::data_order::DataOrder;
    // #[cfg(feature = "dst_arrow")]
    pub use crate::connectorx::destinations::arrow::ArrowDestination;
    // #[cfg(feature = "dst_arrow")]
    pub use crate::connectorx::destinations::arrowstream::{
        ArrowDestination as ArrowStreamDestination,
        ArrowPartitionWriter as ArrowStreamPartitionWriter,
        ArrowTypeSystem as ArrowStreamTypeSystem,
    };
    pub use crate::connectorx::destinations::DestinationPartition;
    pub use crate::connectorx::dispatcher::Dispatcher;
    pub use crate::connectorx::errors::{ConnectorXError, ConnectorXOutError};
    // #[cfg(feature = "dst_arrow")]
    pub use crate::connectorx::get_arrow::get_arrow;
    pub use crate::connectorx::source_router::*;
    // #[cfg(feature = "src_sqlite")]
    pub use crate::connectorx::sources::sqlite::SQLiteSource;
    pub use crate::connectorx::sources::{PartitionParser, Source, SourcePartition};
    pub use crate::connectorx::sql::CXQuery;
    pub use crate::connectorx::transports::*;
    pub use crate::connectorx::typesystem::Transport;
}
