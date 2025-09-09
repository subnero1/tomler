use std::fs;
use tempfile::NamedTempFile;
use toml_edit::DocumentMut;
use tomler::{infer_value, set_nested_in_document};

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
