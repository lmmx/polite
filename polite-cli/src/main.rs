use polite::{connect_sqlite, execute_query, to_dataframe};
use std::env;
use std::fs;

/// Entrypoint
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_cli()
}

/// Extract the main logic so tests can call it directly
fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        return Ok(());
    }

    // First arg is SQL (for now we keep it simple)
    let sql = &args[1];

    // Optional: if a second arg is provided, treat it as a SQLite DB path
    let db_path = if args.len() > 2 {
        Some(&args[2][..])
    } else {
        None
    };

    // Connect to SQLite (in-memory by default)
    let conn = connect_sqlite(db_path)?;

    // Special case: if SQL starts with "@" treat it as a file containing SQL
    let sql = if let Some(path) = sql.strip_prefix('@') {
        fs::read_to_string(path)?
    } else {
        sql.clone()
    };

    // If it's a SELECT, try to convert into a Polars DataFrame
    if sql.trim_start().to_uppercase().starts_with("SELECT") {
        let df = to_dataframe(&conn, &sql)?;
        println!("{df:?}");
    } else {
        // Otherwise just execute as a statement
        let rows = execute_query(&conn, &sql)?;
        eprintln!("Executed successfully, {rows} row(s) affected");
    }

    Ok(())
}

fn print_help() {
    println!("polite — rusqlite × Polars bridge demo");
    println!();
    println!("USAGE:");
    println!("    polite <SQL> [DB_PATH]");
    println!();
    println!("ARGS:");
    println!("    <SQL>       SQL statement (use @file.sql to read from file)");
    println!("    [DB_PATH]   Path to SQLite database file (defaults to in-memory)");
    println!();
    println!("EXAMPLES:");
    println!("    polite \"CREATE TABLE t (id INTEGER, name TEXT)\"");
    println!("    polite \"INSERT INTO t VALUES (1, 'Alice')\"");
    println!("    polite \"SELECT * FROM t\"");
    println!("    polite @queries.sql mydb.sqlite3");
}
