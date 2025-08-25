use polite::{connect_sqlite, execute_query, from_dataframe, to_dataframe};
use tempfile::NamedTempFile;

#[test]
fn readme_demo() -> anyhow::Result<()> {
    // Create a temporary file-backed SQLite DB
    let db = NamedTempFile::new()?;
    let db_path = db.path().to_str().unwrap();

    // Create a connection (rusqlite is used for writes)
    let conn = connect_sqlite(Some(db_path))?;

    // Create and populate a table
    execute_query(&conn, "CREATE TABLE t (id INTEGER, name TEXT)")?;
    execute_query(&conn, "INSERT INTO t VALUES (1, 'Alice')")?;
    execute_query(&conn, "INSERT INTO t VALUES (2, 'Bob')")?;

    // Query back into a Polars DataFrame (ConnectorX path)
    let df = to_dataframe(db_path, "SELECT * FROM t")?;
    println!("{:?}", df);

    // Write a DataFrame back into another table
    from_dataframe(&conn, "t_copy", &df)?;

    Ok(())
}
