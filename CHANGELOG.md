# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2025-10-25

### Added
- Added metadata context object (`_`) available in all template expressions with three properties:
  - `_.file_path`: The path to the current source TOML file being processed
  - `_.table_name`: The name of the current table being processed
  - `_.output_format`: The output format specified on the command line (json, toml, exports, tfvars)
- Added comprehensive REFERENCE.md documentation covering all plugins, features, and usage patterns

## [0.6.0] - 2025-10-10

### Fixed
- Fixed import path resolution to resolve paths relative to the source file directory instead of the current working directory
- Fixed CLI test scripts to use SUPERTOML_BIN environment variable for better test isolation

### Added
- Added comprehensive CLI integration tests with test cases for various import path scenarios
- Added test cases for importing from child directories, nested children, sibling directories, and dot-slash imports

### Changed
- Refactored test organization by renaming `test_cases` to `toml_test_cases` for better clarity and organization
- Improved test infrastructure with better build system integration

### Removed
- Dropped Windows support and removed Windows build workflow from CI/CD pipeline

## [0.5.0] - 2025-09-14

### Fixed
- Fixed cycle detection to allow multiple references to the same table while still preventing infinite recursion, enabling value accumulation and overwriting

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
