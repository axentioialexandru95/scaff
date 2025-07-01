use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn scaff_cmd() -> Command {
    Command::cargo_bin("scaff").unwrap()
}

#[test]
fn test_cli_help() {
    scaff_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Architecture in your pocket"));
}

#[test]
fn test_scan_default() {
    let temp_dir = TempDir::new().unwrap();

    scaff_cmd()
        .arg("scan")
        .current_dir(temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_scan_rust() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn main() {}").unwrap();

    scaff_cmd()
        .arg("scan")
        .arg("--language")
        .arg("rust")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("test.rs"));
}

#[test]
fn test_list_empty() {
    let temp_dir = TempDir::new().unwrap();

    scaff_cmd()
        .arg("list")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("No scaffs found"));
}

#[test]
fn test_save_pattern() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("test.rs"), "fn main() {}").unwrap();

    scaff_cmd()
        .arg("save")
        .arg("test_pattern")
        .arg("--language")
        .arg("rust")
        .current_dir(temp_dir.path())
        .assert()
        .success();
}
