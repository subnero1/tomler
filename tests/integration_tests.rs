use anyhow::Result;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_cli_basic_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let toml_file = temp_dir.path().join("test.toml");

    // Test setting values
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&[
            "--file",
            toml_file.to_str().unwrap(),
            "set",
            "name",
            "test-app",
        ])
        .output()?;
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Set 'name' = 'test-app'"));

    // Test setting nested values
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&[
            "--file",
            toml_file.to_str().unwrap(),
            "set",
            "database.host",
            "localhost",
        ])
        .output()?;
    assert!(output.status.success());

    // Test setting different data types
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&[
            "--file",
            toml_file.to_str().unwrap(),
            "set",
            "database.port",
            "5432",
        ])
        .output()?;
    assert!(output.status.success());

    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&[
            "--file",
            toml_file.to_str().unwrap(),
            "set",
            "database.enabled",
            "true",
        ])
        .output()?;
    assert!(output.status.success());

    // Test getting values
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&["--file", toml_file.to_str().unwrap(), "get", "name"])
        .output()?;
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "test-app");

    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&[
            "--file",
            toml_file.to_str().unwrap(),
            "get",
            "database.host",
        ])
        .output()?;
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "localhost");

    // Test listing keys
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&["--file", toml_file.to_str().unwrap(), "keys"])
        .output()?;
    assert!(output.status.success());
    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("name"));
    assert!(output_str.contains("database"));

    // Test removing values
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&["--file", toml_file.to_str().unwrap(), "remove", "name"])
        .output()?;
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Removed 'name'"));

    // Verify the TOML file structure
    let content = fs::read_to_string(&toml_file)?;
    println!("Final TOML content:\n{}", content);
    assert!(content.contains("[database]"));
    assert!(content.contains("host = \"localhost\""));
    assert!(content.contains("port = 5432"));
    assert!(content.contains("enabled = true"));
    assert!(!content.contains("name ="));

    Ok(())
}

#[test]
fn test_cli_error_handling() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let toml_file = temp_dir.path().join("test.toml");

    // Test getting from non-existent file
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&["--file", toml_file.to_str().unwrap(), "get", "nonexistent"])
        .output()?;
    assert!(!output.status.success());

    // Create a file and test getting non-existent key
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&[
            "--file",
            toml_file.to_str().unwrap(),
            "set",
            "test",
            "value",
        ])
        .output()?;
    assert!(output.status.success());

    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&["--file", toml_file.to_str().unwrap(), "get", "nonexistent"])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("not found"));

    Ok(())
}

#[test]
fn test_cli_help_and_version() -> Result<()> {
    // Test --help
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&["--help"])
        .output()?;
    assert!(output.status.success());
    let help_text = String::from_utf8_lossy(&output.stdout);
    assert!(help_text.contains("A simple lightweight TOML get/set tool"));
    assert!(help_text.contains("Commands:"));
    assert!(help_text.contains("get"));
    assert!(help_text.contains("set"));

    // Test --version
    let output = Command::new(env!("CARGO_BIN_EXE_tomler"))
        .args(&["--version"])
        .output()?;
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("tomler"));

    Ok(())
}
