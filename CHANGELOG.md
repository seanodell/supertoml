# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-09-11

### Added
- `env(name)` and `env_or(name, default)` template functions for environment variable access

## [0.3.0] - 2025-09-08

### Added
- ImportPlugin for importing key/value pairs from external TOML files with optional key transformation
- Shared utilities module (`src/utils.rs`) for common plugin functions

### Changed
- Improved idiomatic Rust error handling across all plugins
- Reduced code duplication through shared utility functions
- Updated documentation with new plugin and modern code patterns

## [0.2.0] - 2025-09-07

### Added
- Terraform variables (tfvars) output format support
- Advanced configuration example with comprehensive integration tests

### Fixed
- Fixed exports output not properly escaping double quotes
- Fixed variables not resolving correctly in maps and arrays

## [0.1.0] - 2024-09-07

### Added
- Initial release of supertoml TOML processing tool
- Command-line interface with Clap argument parsing
- Plugin-based architecture for extensible TOML processing
- Core plugins: noop, templating, before, after, and reference
- Recursive table resolving with circular reference detection
- Comprehensive error handling and validation
- TOML-based unit test system
- Cross-platform binary builds for Windows, macOS, and Linux
- Automated release workflow with GitHub Actions
- Pre-commit hooks for code quality
- MIT license and development documentation
