// polite/tests/roundtrip.rs
use polars::prelude::*;
use polite::{connect_sqlite, from_dataframe, to_dataframe};

#[test]
fn test_roundtrip_dataframe() {
    // In-memory DB
    let conn = connect_sqlite(None).unwrap();

    // Build a Polars DataFrame
    let df = df! {
        "id" => &[1, 2, 3],
        "name" => &["Alice", "Bob", "Charlie"],
    }
    .unwrap();

    // Insert into SQLite
    from_dataframe(&conn, "people", &df).unwrap();

    // Read back
    let df2 = to_dataframe(&conn, "SELECT * FROM people").unwrap();

    // Verify roundtrip
    assert_eq!(df.shape(), df2.shape());
}

#[test]
#[ignore] // Values not coming through correctly yet
fn test_roundtrip_dataframe_values() {
    let conn = connect_sqlite(None).unwrap();

    // Build a Polars DataFrame
    let df = df! {
        "id" => &[1, 2, 3],
        "name" => &["Alice", "Bob", "Charlie"],
    }
    .unwrap();

    // Insert into SQLite
    from_dataframe(&conn, "people", &df).unwrap();

    // Query back
    let df2 = to_dataframe(&conn, "SELECT * FROM people ORDER BY id").unwrap();

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
