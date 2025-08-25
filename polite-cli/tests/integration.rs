use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_create_insert_select() {
    let db_path = tempfile::NamedTempFile::new().unwrap();

    // Create table
    let mut cmd = Command::cargo_bin("polite").unwrap();
    cmd.arg("CREATE TABLE people (id INTEGER, name TEXT)")
        .arg(db_path.path());
    cmd.assert().success();

    // Insert
    let mut cmd = Command::cargo_bin("polite").unwrap();
    cmd.arg("INSERT INTO people VALUES (1, 'Alice')")
        .arg(db_path.path());
    cmd.assert().success();

    // Select
    let mut cmd = Command::cargo_bin("polite").unwrap();
    cmd.arg("SELECT * FROM people").arg(db_path.path());
    let output = cmd.assert().success().get_output().stdout.clone();

    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains("Alice"));
}

/// Tests the `--help` flag
#[test]
fn help_flag_shows_usage() {
    let mut cmd = Command::cargo_bin("polite").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("polite <SQL> [DB_PATH]"));
}

/// Tests a basic create + insert + select roundtrip
#[test]
fn create_insert_select_file_fixture() {
    let db = tempfile::NamedTempFile::new().unwrap();

    // Create table
    assert_cmd::Command::cargo_bin("polite")
        .unwrap()
        .arg("CREATE TABLE t (id INTEGER, name TEXT)")
        .arg(db.path())
        .assert()
        .success();

    // Insert row
    assert_cmd::Command::cargo_bin("polite")
        .unwrap()
        .arg("INSERT INTO t VALUES (1, 'Alice')")
        .arg(db.path())
        .assert()
        .success();

    // Select back
    let output = assert_cmd::Command::cargo_bin("polite")
        .unwrap()
        .arg("SELECT * FROM t")
        .arg(db.path())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8_lossy(&output);
    assert!(stdout.contains("Alice"));
}
