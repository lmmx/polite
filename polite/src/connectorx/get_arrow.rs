use crate::connectorx::{
    arrow_batch_iter::{ArrowBatchIter, RecordBatchIterator},
    prelude::*,
    sql::CXQuery,
};
use fehler::{throw, throws};
use log::debug;
#[allow(unused_imports)]
use std::sync::Arc;

#[allow(unreachable_code, unreachable_patterns, unused_variables, unused_mut)]
#[throws(ConnectorXOutError)]
pub fn get_arrow(
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
    pre_execution_queries: Option<&[String]>,
) -> ArrowDestination {
    let mut destination = ArrowDestination::new();
    let protocol = source_conn.proto.as_str();
    debug!("Protocol: {}", protocol);

    match source_conn.ty {
        // #[cfg(feature = "src_sqlite")]
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, queries.len())?;
            let dispatcher = Dispatcher::<_, _, SQLiteArrowTransport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run()?;
        }
        _ => throw!(ConnectorXOutError::SourceNotSupport(format!(
            "{:?}",
            source_conn.ty
        ))),
    }

    destination
}

#[allow(unreachable_code, unreachable_patterns, unused_variables, unused_mut)]
pub fn new_record_batch_iter(
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
    batch_size: usize,
    pre_execution_queries: Option<&[String]>,
) -> Box<dyn RecordBatchIterator> {
    let destination = ArrowStreamDestination::new_with_batch_size(batch_size);
    let protocol = source_conn.proto.as_str();
    debug!("Protocol: {}", protocol);

    match source_conn.ty {
        // #[cfg(feature = "src_sqlite")]
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, queries.len()).unwrap();
            let batch_iter = ArrowBatchIter::<_, SQLiteArrowStreamTransport>::new(
                source,
                destination,
                origin_query,
                queries,
            )
            .unwrap();
            return Box::new(batch_iter);
        }
        _ => {}
    }
    panic!("not supported!");
}
