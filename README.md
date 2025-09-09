# 🔧 Tomler

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A simple, fast, and reliable CLI utility for **in-place editing of TOML files** with automatic type inference and nested key support.

## ✨ Features

- **🔄 In-place editing**: Modify TOML files while preserving comments and formatting
- **🎯 Dot notation**: Access nested keys with `server.database.port` syntax
- **🧠 Smart type inference**: Automatically detects booleans, integers, floats, arrays, and strings
- **📝 Format preservation**: Uses `toml_edit` to maintain original file structure
- **⚡ Fast and lightweight**: Single binary with minimal dependencies
- **🔧 Shell-friendly**: Perfect for automation and scripting

## 🚀 Quick Start

```bash
# Create or edit config.toml (default file)
echo '[database]' > config.toml
echo 'host = "localhost"' >> config.toml
echo '[app]' >> config.toml
echo '[server]' >> config.toml

# Get a value
tomler get database.host

# Set a string value
tomler set app.name "My Application"

# Set a number
tomler set server.port 8080

# Set a boolean
tomler set debug true

# Set an array
tomler set allowed_hosts "localhost,127.0.0.1,example.com"

# Work with nested keys
tomler set database.host.url "tcp://localhost:6379"
```

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/subnero1/tomler.git
cd tomler

# Build release binary
cargo build --release

# Install to ~/.cargo/bin (make sure it's in your PATH)
cargo install --path .
```

### Prerequisites

- Rust 1.70+ ([Install via rustup](https://rustup.rs/))

## 📚 Usage

```
Edit TOML files in-place with simple type inference and nested keys

Usage: tomler [OPTIONS] <COMMAND>

Commands:
  get   Get a value by key (dot notation)
  set   Set a value by key (dot notation)
  help  Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>  TOML file path (default: config.toml) [default: config.toml]
  -h, --help         Print help
```

### Commands

#### `get <key>`
Retrieve a value from the TOML file using dot notation.

```bash
tomler get server.port
tomler -f app.toml get database.url
```

#### `set <key> <value>`
Set a value in the TOML file with automatic type inference.

```bash
tomler set server.port 3000
tomler -f app.toml set debug false
```

## 🧠 Type Inference

Tomler automatically infers the correct TOML type based on the input:

| Input | TOML Type | Example |
|-------|-----------|---------|
| `true`, `false` | Boolean | `debug = true` |
| `42`, `-10` | Integer | `port = 8080` |
| `3.14`, `0.5` | Float | `timeout = 30.5` |
| `a,b,c` | Array | `hosts = ["a", "b", "c"]` |
| `1,2,3` | Integer Array | `ports = [80, 443, 8080]` |
| `"hello"`, `anything else` | String | `name = "hello"` |

### Array Syntax

Simple comma-separated values are converted to arrays:

```bash
# Creates: ports = [80, 443, 8080]
tomler set ports "80,443,8080"

# Creates: tags = ["web", "api", "production"]
tomler set tags "web,api,production"

# Quoted strings are treated as single values:
# Creates: description = "Hello, World!"
tomler set description "\"Hello, World!\""
```

## 📁 File Handling

- **Default file**: `config.toml` (in current directory)
- **Custom file**: Use `-f` or `--file` option
- **File creation**: Creates file if it doesn't exist (for `set` operations)
- **Format preservation**: Maintains comments, spacing, and key order

## 🎯 Examples

### Basic Configuration Management

```bash
# Initialize a new config
tomler set app.name "MyApp"
tomler set app.version "1.0.0"
tomler set app.debug false

# Configure server
tomler set server.host "0.0.0.0"
tomler set server.port 8080
tomler set server.workers 4

# Set up database
tomler set database.url "postgresql://localhost/myapp"
tomler set database.pool_size 10
tomler set database.timeout 30.0
```

Result in `config.toml`:
```toml
[app]
name = "MyApp"
version = "1.0.0"
debug = false

[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "postgresql://localhost/myapp"
pool_size = 10
timeout = 30.0
```

### Working with Arrays

```bash
# Set allowed origins for CORS
tomler set cors.allowed_origins "http://localhost:3000,https://example.com"

# Set multiple environment variables
tomler set env.required "DATABASE_URL,SECRET_KEY,API_KEY"

# Set numeric arrays
tomler set monitoring.alert_thresholds "50,75,90,95"
```

### Shell Scripting Integration

```bash
#!/bin/bash

# Read current port
current_port=$(tomler get server.port)
echo "Current port: $current_port"

# Update configuration based on environment
if [[ "$ENV" == "production" ]]; then
    tomler set app.debug false
    tomler set server.workers 8
    tomler set database.pool_size 20
else
    tomler set app.debug true
    tomler set server.workers 2
    tomler set database.pool_size 5
fi

# Set deployment timestamp
tomler set deployment.timestamp "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
```

### Docker Integration

```dockerfile
FROM rust:1.70 AS builder
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /target/release/tomler /usr/local/bin/
RUN tomler set app.environment "production"
```

## ⚠️ Error Handling

Tomler provides clear error messages:

```bash
# File not found
$ tomler -f missing.toml get key
Error: failed to read missing.toml: No such file or directory

# Invalid TOML syntax
$ echo "invalid toml [" > bad.toml
$ tomler -f bad.toml get key
Error: failed to parse toml file bad.toml: expected `]`, found eof

# Key not found
$ tomler get nonexistent.key
Key not found: nonexistent.key
```

Exit codes:
- `0`: Success
- `1`: General error (file I/O, parsing, etc.)
- `2`: Key not found (get command only)

## 🛠️ Development

### Building from Source

```bash
git clone https://github.com/subnero1/tomler.git
cd tomler

# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check formatting and linting
cargo fmt --check
cargo clippy -- -D warnings
```

### Project Structure

```
tomler/
├── src/
│   ├── main.rs          # CLI interface and argument parsing
│   └── lib.rs           # Core TOML manipulation logic
├── tests/
│   └── integration.rs   # Integration tests
├── Cargo.toml           # Project configuration
└── README.md           # This file
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Guidelines

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/AmazingFeature`)
3. **Commit** your changes (`git commit -m 'Add some AmazingFeature'`)
4. **Push** to the branch (`git push origin feature/AmazingFeature`)
5. **Open** a Pull Request

Please ensure:
- Tests pass (`cargo test`)
- Code is formatted (`cargo fmt`)
- No clippy warnings (`cargo clippy`)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [toml_edit](https://crates.io/crates/toml_edit) for format-preserving TOML manipulation
- CLI powered by [clap](https://crates.io/crates/clap)
- Error handling via [anyhow](https://crates.io/crates/anyhow)

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/subnero1/tomler/issues)
- **Discussions**: [GitHub Discussions](https://github.com/subnero1/tomler/discussions)

---

**Made with ❤️ by [Chinmay Pendharkar](mailto:chinmay@subnero.com)**
