# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-09

### Added
- Initial release of tomler
- Basic CLI functionality for getting, setting, and removing TOML values
- Support for nested keys using dot notation (e.g., `database.host`)
- Auto-detection of value types (string, number, boolean, array)
- Pretty-printed TOML output
- Comprehensive error handling
- Unit and integration tests
- CI/CD pipeline with GitHub Actions

### Features
- `get` command to retrieve values from TOML files
- `set` command to set values in TOML files
- `remove` command to delete keys from TOML files
- `keys` command to list all top-level keys
- Support for nested table structures
- Automatic type inference for values

[Unreleased]: https://github.com/subnero1/tomler/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/subnero1/tomler/releases/tag/v0.1.0