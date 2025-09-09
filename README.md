# Tomler

[![CI](https://github.com/subnero1/tomler/workflows/CI/badge.svg)](https://github.com/subnero1/tomler/actions)
[![Crates.io](https://img.shields.io/crates/v/tomler.svg)](https://crates.io/crates/tomler)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple, lightweight TOML get/set tool for manipulating TOML configuration files from the command line.

## Features

- 🚀 **Fast and lightweight** - Built in Rust for performance
- 📝 **Easy to use** - Simple command-line interface
- 🔍 **Dot notation support** - Access nested values with `database.host`
- 🎯 **Type-aware** - Automatically detects and preserves data types
- 🔧 **File operations** - Get, set, remove, and list keys in TOML files
- ✅ **Well tested** - Comprehensive unit and integration tests
- 🛡️ **Error handling** - Clear error messages and proper exit codes

## Installation

### From Source

```bash
git clone https://github.com/subnero1/tomler.git
cd tomler
cargo install --path .
```

### From Crates.io (when published)

```bash
cargo install tomler
```

## Usage

### Basic Commands

```bash
# Set a value (creates file if it doesn't exist)
tomler set name "my-app"
tomler set version "1.0.0"

# Set nested values using dot notation
tomler set database.host "localhost"
tomler set database.port 5432
tomler set database.enabled true

# Get a value
tomler get name
# Output: my-app

tomler get database.host
# Output: localhost

# List all top-level keys
tomler keys
# Output:
# database
# name
# version

# Remove a key
tomler remove version
# Output: Removed 'version' (was: 1.0.0)
```

### Specify Custom File

```bash
# Use a different TOML file
tomler --file config/app.toml set debug true
tomler -f ~/.config/myapp.toml get theme
```

### Data Types

Tomler automatically detects and preserves data types:

```bash
# Strings
tomler set name "hello world"

# Integers
tomler set count 42

# Floats
tomler set pi 3.14159

# Booleans
tomler set enabled true
tomler set debug false

# Arrays (basic support)
tomler set tags '["rust", "cli", "toml"]'
```

### Example TOML Output

After running the commands above, your `config.toml` might look like:

```toml
name = "my-app"

[database]
enabled = true
host = "localhost"
port = 5432
```

## Command Reference

### `tomler set <key> <value>`

Set a value in the TOML file. Creates the file if it doesn't exist.

- Supports dot notation for nested keys
- Automatically detects value type
- Creates intermediate tables as needed

### `tomler get <key>`

Get a value from the TOML file.

- Returns the value as a string
- Exits with code 1 if key not found
- Supports dot notation for nested keys

### `tomler remove <key>`

Remove a key from the TOML file.

- Shows the removed value
- Exits with code 1 if key not found
- Supports dot notation for nested keys

### `tomler keys`

List all top-level keys in the TOML file.

### Global Options

- `-f, --file <FILE>`: Specify the TOML file to operate on (default: `config.toml`)
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## Library Usage

Tomler can also be used as a Rust library:

```rust
use tomler::{TomlDocument, parse_value};
use anyhow::Result;

fn main() -> Result<()> {
    // Create a new TOML document
    let mut doc = TomlDocument::new();
    
    // Set values
    doc.set("name", parse_value("my-app"))?;
    doc.set("database.host", parse_value("localhost"))?;
    
    // Get values
    if let Some(name) = doc.get("name") {
        println!("Name: {}", name);
    }
    
    // Save to file
    doc.to_file("config.toml")?;
    
    // Load from file
    let loaded_doc = TomlDocument::from_file("config.toml")?;
    
    Ok(())
}
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration_tests
```

### Linting

```bash
# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass (`cargo test`)
6. Run formatting and linting (`cargo fmt && cargo clippy`)
7. Commit your changes (`git commit -am 'Add some amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each version.
