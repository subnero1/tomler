use std::fs;
use tempfile::NamedTempFile;
use toml_edit::DocumentMut;
use tomler::{infer_value, set_nested_in_document};
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn round_trip_on_file() {
    let f = NamedTempFile::new().expect("create temp file");
    let initial = r#"# initial config
[app]
name = "example"
"#;
    fs::write(f.path(), initial).expect("write initial");

    // read doc
    let raw = fs::read_to_string(f.path()).unwrap();
    let mut doc: DocumentMut = raw.parse().unwrap();

    // set new nested key
    set_nested_in_document(&mut doc, "app.retries", infer_value("5"));
    // write back
    fs::write(f.path(), doc.to_string()).expect("write back");

    // read and assert
    let after = fs::read_to_string(f.path()).unwrap();
    assert!(after.contains("retries = 5"));
    // preserve comment
    assert!(after.contains("# initial config"));
}

#[test]
fn cli_get_default_and_raw_modes() {
    // Prepare a temp config file
    let f = NamedTempFile::new().expect("create temp file");
    let initial = r#"
[app]
name = "example"
retries = 3
"#;
    fs::write(f.path(), initial).expect("write initial");

    // Default mode: strings include quotes
    Command::cargo_bin("tomler")
        .unwrap()
        .args(["--file", f.path().to_str().unwrap(), "get", "app.name"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("^\"example\"\n$").unwrap());

    // Raw mode: strings without quotes
    Command::cargo_bin("tomler")
        .unwrap()
        .args(["--file", f.path().to_str().unwrap(), "get", "app.name", "--raw"])
        .assert()
        .success()
        .stdout(predicate::str::is_match("^example\n$").unwrap());

    // Non-strings: raw mode should be same as default
    Command::cargo_bin("tomler")
        .unwrap()
        .args(["--file", f.path().to_str().unwrap(), "get", "app.retries", "--raw"])
        .assert()
        .success()
        .stdout(predicate::eq("3\n"));
}
