use crate::connectorx::errors::{ConnectorXOutError, OutResult};
use crate::connectorx::source_router::{SourceConn, SourceType};
// #[cfg(feature = "src_sqlite")]
use crate::connectorx::sql::get_partition_range_query_sep;
use crate::connectorx::sql::{single_col_partition_query, CXQuery};
use anyhow::anyhow;
use fehler::{throw, throws};
// #[cfg(feature = "src_sqlite")]
use rusqlite::{types::Type, Connection};
// #[cfg(feature = "src_sqlite")]
use sqlparser::dialect::SQLiteDialect;
use url::Url;

pub struct PartitionQuery {
    query: String,
    column: String,
    min: Option<i64>,
    max: Option<i64>,
    num: usize,
}

impl PartitionQuery {
    pub fn new(query: &str, column: &str, min: Option<i64>, max: Option<i64>, num: usize) -> Self {
        Self {
            query: query.into(),
            column: column.into(),
            min,
            max,
            num,
        }
    }
}

pub fn partition(part: &PartitionQuery, source_conn: &SourceConn) -> OutResult<Vec<CXQuery>> {
    let mut queries = vec![];
    let num = part.num as i64;
    let (min, max) = match (part.min, part.max) {
        (None, None) => get_col_range(source_conn, &part.query, &part.column)?,
        (Some(min), Some(max)) => (min, max),
        _ => throw!(anyhow!(
            "partition_query range can not be partially specified",
        )),
    };

    let partition_size = (max - min + 1) / num;

    for i in 0..num {
        let lower = min + i * partition_size;
        let upper = match i == num - 1 {
            true => max + 1,
            false => min + (i + 1) * partition_size,
        };
        let partition_query = get_part_query(source_conn, &part.query, &part.column, lower, upper)?;
        queries.push(partition_query);
    }
    Ok(queries)
}

pub fn get_col_range(source_conn: &SourceConn, query: &str, col: &str) -> OutResult<(i64, i64)> {
    match source_conn.ty {
        // #[cfg(feature = "src_sqlite")]
        SourceType::SQLite => sqlite_get_partition_range(&source_conn.conn, query, col),
        _ => unimplemented!("{:?} not implemented!", source_conn.ty),
    }
}

#[throws(ConnectorXOutError)]
pub fn get_part_query(
    source_conn: &SourceConn,
    query: &str,
    col: &str,
    lower: i64,
    upper: i64,
) -> CXQuery<String> {
    let query = match source_conn.ty {
        // #[cfg(feature = "src_sqlite")]
        SourceType::SQLite => {
            single_col_partition_query(query, col, lower, upper, &SQLiteDialect {})?
        }
        _ => unimplemented!("{:?} not implemented!", source_conn.ty),
    };
    CXQuery::Wrapped(query)
}

// #[cfg(feature = "src_sqlite")]
#[throws(ConnectorXOutError)]
fn sqlite_get_partition_range(conn: &Url, query: &str, col: &str) -> (i64, i64) {
    // remove the first "sqlite://" manually since url.path is not correct for windows and for relative path
    let conn = Connection::open(&conn.as_str()[9..])?;
    // SQLite only optimize min max queries when there is only one aggregation
    // https://www.sqlite.org/optoverview.html#minmax
    let (min_query, max_query) = get_partition_range_query_sep(query, col, &SQLiteDialect {})?;
    let mut error = None;
    let min_v = conn.query_row(min_query.as_str(), [], |row| {
        // declare type for count query will be None, only need to check the returned value type
        let col_type = row.get_ref(0)?.data_type();
        match col_type {
            Type::Integer => row.get(0),
            Type::Real => {
                let v: f64 = row.get(0)?;
                Ok(v as i64)
            }
            Type::Null => Ok(0),
            _ => {
                error = Some(anyhow!("Partition can only be done on integer columns"));
                Ok(0)
            }
        }
    })?;
    match error {
        None => {}
        Some(e) => throw!(e),
    }
    let max_v = conn.query_row(max_query.as_str(), [], |row| {
        let col_type = row.get_ref(0)?.data_type();
        match col_type {
            Type::Integer => row.get(0),
            Type::Real => {
                let v: f64 = row.get(0)?;
                Ok(v as i64)
            }
            Type::Null => Ok(0),
            _ => {
                error = Some(anyhow!("Partition can only be done on integer columns"));
                Ok(0)
            }
        }
    })?;
    match error {
        None => {}
        Some(e) => throw!(e),
    }

    (min_v, max_v)
}
