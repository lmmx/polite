// polite/tests/int_size.rs
use polars::prelude::*;
use polite::prelude::*;
use tempfile::NamedTempFile;

// 64-bit number just over i32::MAX
const BIG: i64 = i32::MAX as i64 + 1;

// These tests just check that we aren't getting any weird type inference

// A single small integer should be Int64 regardless of its value

#[test]
fn test_small_integer_promotes_to_i64_solo() {
    let db = NamedTempFile::new().unwrap();
    let conn = connect_sqlite(Some(db.path().to_str().unwrap())).unwrap();

    conn.execute("CREATE TABLE t (id INTEGER)", []).unwrap();
    conn.execute("INSERT INTO t (id) VALUES (?)", [42]).unwrap();

    let df = to_dataframe(db.path().to_str().unwrap(), "SELECT * FROM t").unwrap();
    let col = df.column("id").unwrap();

    // Ensure dtype is Int64
    assert_eq!(col.dtype(), &DataType::Int64);

    // Ensure the value is preserved
    let got = col.i64().unwrap().get(0).unwrap();
    assert_eq!(got, 42);
}

// Same goes with large numbers (larger than i32::MAX) even if a small one is first

#[test]
fn test_large_integer_promotes_to_i64_solo() {
    let db = NamedTempFile::new().unwrap();
    let conn = connect_sqlite(Some(db.path().to_str().unwrap())).unwrap();

    conn.execute("CREATE TABLE t (id INTEGER)", []).unwrap();
    conn.execute("INSERT INTO t (id) VALUES (?)", [BIG])
        .unwrap();

    let df = to_dataframe(db.path().to_str().unwrap(), "SELECT * FROM t").unwrap();
    let col = df.column("id").unwrap();

    // Ensure dtype is Int64
    assert_eq!(col.dtype(), &DataType::Int64);

    // Ensure the value is preserved
    let got = col.i64().unwrap().get(0).unwrap();
    assert_eq!(got, BIG);
}

#[test]
fn test_large_integer_promotes_to_i64_with_small_before() {
    let db = NamedTempFile::new().unwrap();
    let conn = connect_sqlite(Some(db.path().to_str().unwrap())).unwrap();

    conn.execute("CREATE TABLE t (id INTEGER)", []).unwrap();

    // Insert the big number after a small number
    conn.execute("INSERT INTO t (id) VALUES (42)", []).unwrap();
    conn.execute("INSERT INTO t (id) VALUES (?)", [BIG])
        .unwrap();

    let df = to_dataframe(db.path().to_str().unwrap(), "SELECT * FROM t").unwrap();
    let col = df.column("id").unwrap();

    assert_eq!(col.dtype(), &DataType::Int64);

    let values: Vec<i64> = col.i64().unwrap().into_no_null_iter().collect();
    assert!(values.contains(&42));
    assert!(values.contains(&BIG));
}

#[test]
fn test_large_integer_promotes_to_i64_with_small_after() {
    let db = NamedTempFile::new().unwrap();
    let conn = connect_sqlite(Some(db.path().to_str().unwrap())).unwrap();

    conn.execute("CREATE TABLE t (id INTEGER)", []).unwrap();

    // Insert the big number FIRST
    conn.execute("INSERT INTO t (id) VALUES (?)", [BIG])
        .unwrap();
    conn.execute("INSERT INTO t (id) VALUES (42)", []).unwrap();

    let df = to_dataframe(db.path().to_str().unwrap(), "SELECT * FROM t").unwrap();
    let col = df.column("id").unwrap();

    assert_eq!(col.dtype(), &DataType::Int64);

    let values: Vec<i64> = col.i64().unwrap().into_no_null_iter().collect();
    assert!(values.contains(&BIG));
    assert!(values.contains(&42));
}
