use polars::prelude::*;
use polite::prelude::*;
use tempfile::NamedTempFile;

#[test]
fn test_friends_db() -> anyhow::Result<()> {
    // Open (or create) a SQLite database
    let db = NamedTempFile::new()?;
    let db_path = db.path().to_str().unwrap();
    let conn = connect_sqlite(Some(db_path))?;

    execute_query(&conn, "CREATE TABLE friends_made (id INTEGER, name TEXT)")?;

    let nobody = load_dataframe(db_path, "SELECT * FROM friends_made")?;
    println!("🤓 I am making friends in SQLite! I don't have any there yet...\n{nobody:?}");

    // Create a table to keep your friends' names in
    execute_query(&conn, "INSERT INTO friends_made VALUES (1, 'Alice')")?;
    execute_query(&conn, "INSERT INTO friends_made VALUES (2, 'Bob')")?;
    execute_query(&conn, "INSERT INTO friends_made VALUES (3, 'Charlie')")?;

    // Query your friends back into a Polars DataFrame
    let dbf = to_dataframe(db_path, "SELECT * FROM friends_made")?;
    println!("🪄 I have lovingly restored my friends into a Polars DataFrame:\n{dbf:?}");

    // Add some more friends directly from a Polars DataFrame
    let polars_friends = df! {
        "id" => [4, 5],
        "name" => ["Dora", "Eve"],
    }?;

    from_dataframe(&conn, "cool_friends", &polars_friends)?;

    println!("🆒 My friends from Polars are now my friends in SQLite:\n{polars_friends:?}");

    let all_friends = load_dataframe(
        db_path,
        "SELECT * FROM friends_made UNION ALL SELECT * FROM cool_friends ORDER BY id",
    )?;
    println!("🎉 All my friends are politely gathered in a DataFrame:\n{all_friends:?}");

    Ok(())
}
