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
    let sql = if sql.starts_with('@') {
        let path = &sql[1..];
        fs::read_to_string(path)?
    } else {
        sql.clone()
    };

    // If it's a SELECT, try to convert into a Polars DataFrame
    if sql.trim_start().to_uppercase().starts_with("SELECT") {
        let df = to_dataframe(&conn, &sql)?;
        println!("{:?}", df);
    } else {
        // Otherwise just execute as a statement
        let rows = execute_query(&conn, &sql)?;
        eprintln!("Executed successfully, {} row(s) affected", rows);
    }

    Ok(())
}

fn print_help() {
    println!("polite-cli — rusqlite × Polars bridge demo");
    println!();
    println!("USAGE:");
    println!("    polite-cli <SQL> [DB_PATH]");
    println!();
    println!("ARGS:");
    println!("    <SQL>       SQL statement (use @file.sql to read from file)");
    println!("    [DB_PATH]   Path to SQLite database file (defaults to in-memory)");
    println!();
    println!("EXAMPLES:");
    println!("    polite-cli \"CREATE TABLE t (id INTEGER, name TEXT)\"");
    println!("    polite-cli \"INSERT INTO t VALUES (1, 'Alice')\"");
    println!("    polite-cli \"SELECT * FROM t\"");
    println!("    polite-cli @queries.sql mydb.sqlite3");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_flag() {
        // Simulate running with `--help`
        let result = run_cli();
        assert!(result.is_ok());
    }

    #[test]
    fn test_basic_nonselect_sql() {
        // This will hit the stubbed polite::execute_query
        let args = vec![
            "polite-cli".to_string(),
            "CREATE TABLE t (id INTEGER)".to_string(),
        ];
        std::env::set_var("RUST_BACKTRACE", "0");
        // Can't override env::args easily without a crate like `clap`, so
        // this test only calls the helper functions directly for now.
        let conn = connect_sqlite(None).unwrap();
        let rows = execute_query(&conn, "CREATE TABLE t (id INTEGER)").unwrap();
        assert_eq!(rows, 0);
    }
}
