use polars::prelude::*;
use polite::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_connect_error() {
    let err = connect_sqlite(Some("/nonexistent/dir/db.sqlite")).unwrap_err();
    assert!(matches!(err, PoliteError::Connect { .. }));
}

#[test]
fn test_query_error() {
    let db = NamedTempFile::new().unwrap();
    let db_path = db.path().to_str().unwrap();
    let err = to_dataframe(db_path, "SELECT * FROM nope").unwrap_err();
    assert!(matches!(err, PoliteError::Query { .. }));
}

#[test]
fn test_load_wrapper_error() {
    let db = NamedTempFile::new().unwrap();
    let db_path = db.path().to_str().unwrap();
    let err = load_dataframe(db_path, "SELECT * FROM nope").unwrap_err();
    eprintln!("Got error variant: {err:?}");
    assert!(matches!(err, PoliteError::Load { .. }));
}

#[test]
fn test_save_wrapper_error() {
    let df = df! { "id" => [1] }.unwrap();
    let err = save_dataframe("/dev/null", "friends", &df).unwrap_err();
    assert!(matches!(err, PoliteError::Save { .. }));
}

#[test]
fn test_load_dataframe_error_variant() {
    let db = NamedTempFile::new().unwrap();
    let db_path = db.path().to_str().unwrap();

    // table does not exist -> should trigger PoliteError::Load
    let err = load_dataframe(db_path, "SELECT * FROM imaginary").unwrap_err();
    eprintln!("Got error variant: {err:?}");
    assert!(matches!(err, PoliteError::Load { .. }));
}

#[test]
fn test_save_dataframe_error_variant() {
    let df = df! { "id" => [1] }.unwrap();

    // invalid path -> should trigger PoliteError::Connect or Save depending on failure mode
    let err = save_dataframe("/definitely/not/a/real/path/xxx", "friends", &df).unwrap_err();
    assert!(matches!(
        err,
        PoliteError::Connect { .. } | PoliteError::Save { .. }
    ));
}
