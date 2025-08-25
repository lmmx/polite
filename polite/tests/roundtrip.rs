// polite/tests/roundtrip.rs
use polars::prelude::*;
use polite::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_roundtrip_dataframe() {
    // Temp DB on disk (ConnectorX needs file-based DB)
    let db = NamedTempFile::new().unwrap();
    let db_path = db.path().to_str().unwrap();

    // Build a Polars DataFrame
    let df = df! {
        "id" => &[1i64, 2, 3],
        "name" => &["Alice", "Bob", "Charlie"],
    }
    .unwrap();

    // Insert into SQLite (rusqlite path)
    let conn = connect_sqlite(Some(db_path)).unwrap();
    from_dataframe(&conn, "people", &df).unwrap();

    // Read back using ConnectorX → Polars
    let df2 = to_dataframe(db_path, "SELECT * FROM people ORDER BY id").unwrap();

    // Check shapes
    assert_eq!(df.shape(), df2.shape());

    // Check values
    assert_eq!(
        df.column("id")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<_>>(),
        df2.column("id")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        df.column("name")
            .unwrap()
            .str()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<_>>(),
        df2.column("name")
            .unwrap()
            .str()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<_>>()
    );
}

#[test]
#[ignore] // Values not coming through correctly yet
fn test_roundtrip_dataframe_values() {
    let db = NamedTempFile::new().unwrap();
    let db_path = db.path().to_str().unwrap();
    let conn = connect_sqlite(Some(db_path)).unwrap();

    // Build a Polars DataFrame
    let df = df! {
        "id" => &[1, 2, 3],
        "name" => &["Alice", "Bob", "Charlie"],
    }
    .unwrap();

    // Insert into SQLite
    from_dataframe(&conn, "people", &df).unwrap();

    // Query back
    let df2 = to_dataframe(db_path, "SELECT * FROM people ORDER BY id").unwrap();

    // Compare values
    assert_eq!(df.shape(), df2.shape());
    assert_eq!(
        df.column("id")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<_>>(),
        df2.column("id")
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<_>>()
    );
    assert_eq!(
        df.column("name")
            .unwrap()
            .str()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<_>>(),
        df2.column("name")
            .unwrap()
            .str()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<_>>()
    );
}
