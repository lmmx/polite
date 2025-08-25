use polite::prelude::*;
use std::env;
use std::fs;

/// Entrypoint
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_cli()
}

fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        print_help();
        return Ok(());
    }

    // First arg is SQL (for now we keep it simple)
    let sql = &args[1];

    // Optional: if a second arg is provided, treat it as a SQLite DB path
    let db_path = match args.get(2) {
        Some(path) => path.as_str(),
        None => {
            eprintln!("Error: you must provide a SQLite database file path.");
            std::process::exit(1);
        }
    };

    // Special case: if SQL starts with "@" treat it as a file containing SQL
    let sql = if let Some(path) = sql.strip_prefix('@') {
        fs::read_to_string(path)?
    } else {
        sql.clone()
    };

    if sql.trim_start().to_uppercase().starts_with("SELECT") {
        let df = to_dataframe(db_path, &sql)?;
        println!("{df}");
    } else {
        // For writes, we only need rusqlite anyway
        let conn = connect_sqlite(Some(db_path))?;
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
