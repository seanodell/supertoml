# SuperTOML Development Guide

This document is for developers who want to contribute to SuperTOML, understand its architecture, or use it as a Rust library.

## Overview

SuperTOML is both a command-line tool and a Rust library designed to work with TOML configuration files. It can extract specific tables from TOML files, process them through configurable plugins, and output the results in various formats including TOML, JSON, dotenv, shell exports, and Terraform variables.

## Features

- **Table Extraction**: Extract specific tables from TOML files
- **Multiple Output Formats**: Support for TOML, JSON, dotenv, shell export, and Terraform variables formats
- **Plugin Architecture**: Extensible plugin system for custom data processing
- **Cycle Detection**: Prevents infinite loops when processing table dependencies
- **Type-Safe Parsing**: Leverages Rust's type system for safe TOML parsing
- **Comprehensive Error Handling**: Detailed error messages for debugging

## Prerequisites

- Rust 1.89.0 or later (specified in `mise.toml`)

## Building from Source

```bash
git clone https://github.com/supertoml/supertoml.git
cd supertoml
cargo build --release
```

The binary will be available at `target/release/supertoml`.

## Library Usage

SuperTOML can also be used as a Rust library:

```rust
use supertoml::{Resolver, format_as_json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use standard plugins (before, import, templating, after)
    let mut resolver = Resolver::new(vec![
        &supertoml::plugins::BeforePlugin as &dyn supertoml::Plugin,
        &supertoml::plugins::ImportPlugin as &dyn supertoml::Plugin,
        &supertoml::plugins::TemplatingPlugin as &dyn supertoml::Plugin,
        &supertoml::plugins::AfterPlugin as &dyn supertoml::Plugin,
    ]);
    let values = resolver.resolve_table("config.toml", "database")?;
    let json_output = format_as_json(&values)?;
    println!("{}", json_output);
    Ok(())
}
```

## Plugin System

### Variable Resolution Order

**Critical Concept**: Variables can only reference values that were defined in previously processed tables. This affects how plugins access and process data:

1. **Dependencies first**: Tables listed in `_.before` are processed before the current table
2. **Current table processing**: The target table is processed with access to all previously resolved variables
3. **Post-processing**: Tables listed in `_.after` are processed after the current table

When a plugin processes a table:
- Variables in `resolver.values` contain all values from previously processed tables
- The current table cannot reference variables defined within itself
- Template resolution uses only the existing context from `resolver.values`

This means plugins should be designed to work with the dependency order and cannot resolve forward references.

### Creating Plugins

Plugins implement the `Plugin` trait with type-safe configuration using the `extract_config!` macro. Key points:

- **Configuration**: Use `extract_config!(config, YourConfigType, plugin_name)?` for type-safe config extraction
- **Processing**: Access `resolver.values`, `resolver.toml_file`, and call `resolve_table_recursive()` for dependencies
- **Values**: Use `crate::utils::add_values_to_resolver(resolver, table_values)` to add processed values
- **Chain Safety**: Don't drain `table_values` as they may be passed to other plugins

See `src/plugins/` for implementation examples.

### Plugin Behavior

Plugins receive three parameters:
- `resolver`: Access to the resolver for recursive table resolution and global values
- `table_values`: The current table's key-value pairs (can be modified but not drained)
- `config`: Plugin-specific configuration from the TOML file

**Important**: Plugins should not drain `table_values` because they may be passed to other plugins in the processing chain. Most plugins should call `crate::utils::add_values_to_resolver(resolver, table_values)` to copy values to the global resolver context.

If a plugin modifies `table_values`, it should also update `resolver.values` to match the modified values, so that if later plugins re-add the table values to `resolver.values`, they get the modified values, not the original ones.

### Plugin Configuration

Plugins are configured in TOML files using a special `_` key within the table:

```toml
[my_table]
key1 = "value1"
key2 = "value2"
_.my_plugin = { option1 = "config_value", option2 = 42 }
```

### Meta Values Implementation

Meta values provide processing context to templates through a `_` object that contains processing arguments like `table_name`, `output_format`, and `file_path`. The implementation is found in:

- **Storage**: `src/resolver.rs` - Meta values are stored with the `_` key
- **Template Access**: `src/plugins/templating.rs` and `src/plugins/import.rs` - `_` object is added to template context
- **Usage**: Templates access meta values via `{{ _.args.* }}` syntax

Meta values are only available in templates, not in plugin configuration (which uses `_` syntax).

### Using Plugins

Create a `Resolver` with your desired plugins and call `resolve_table()`. Standard plugins include `BeforePlugin`, `ImportPlugin`, `TemplatingPlugin`, and `AfterPlugin`. Development plugins include `NoopPlugin` and `ReferencePlugin`.

See `src/main.rs` for CLI usage examples and `src/lib.rs` for library exports.

### Built-in Plugins

SuperTOML comes with several built-in plugins, categorized as follows:

#### Standard Plugins (Included by Default)
These plugins are automatically included when using SuperTOML and provide core functionality:

**TemplatingPlugin**
Processes string values through Minijinja templating using `resolver.values` as context. Always runs and requires no configuration. Provides access to meta values through the `_` object in templates.

**BeforePlugin**
Resolves multiple tables before processing the current table. Configuration:
```toml
_.before = ["table1", "table2", "table3"]
```

**ImportPlugin**
Imports key/value pairs from external TOML files with optional key transformation. Runs after dependency resolution but before templating so imported values can be used in templates. Supports key transformation using templates with access to meta values. Configuration:
```toml
_.import = [
    { file = "database.toml", table = "production", key_format = "db_{{key}}" },
    { file = "tools.toml", table = "versions" }
]
```

**AfterPlugin**
Resolves multiple tables after processing the current table. Does not add current table values to `resolver.values`. Configuration:
```toml
_.after = ["table1", "table2", "table3"]
```

#### Development/Testing Plugins
These plugins are primarily used for testing, development, or specific use cases:

**NoopPlugin**
A simple plugin that adds all table values to `resolver.values`. Always runs and requires no configuration. Useful for testing and development.

**ReferencePlugin**
Resolves another table before processing the current table. Configuration:
```toml
_.reference = { table = "other_table" }
```
Useful for testing recursive resolution scenarios.

### Plugin Implementation Details

#### TemplatingPlugin (`src/plugins/templating.rs`)
- Recursively processes nested TOML structures (strings, arrays, tables)
- Integrates meta values (`_` object) into template context
- Handles template rendering with Minijinja

#### ImportPlugin (`src/plugins/import.rs`)
- Supports key transformation using templates
- Provides meta values access in key transformation templates
- Handles multiple import configurations

#### BeforePlugin/AfterPlugin (`src/plugins/before.rs`, `src/plugins/after.rs`)
- Implements dependency resolution with cycle detection
- Processes multiple table dependencies
- Maintains resolver state across dependencies

### Plugin Config Types

Plugin configuration can be any valid TOML type (strings, numbers, booleans, arrays, tables). Use the `extract_config!` macro with appropriate Rust types for type-safe deserialization.

## Architecture

### Core Components

#### 1. **Resolver** (`src/resolver.rs`)
Core resolution logic, plugin orchestration, and circular dependency detection.

#### 2. **Loader** (`src/loader.rs`)
TOML file operations, parsing, and table extraction utilities.

#### 3. **Formatter** (`src/formatter.rs`)
Output formatting to JSON, TOML, YAML, dotenv, exports, tfvars.

#### 4. **Error Handling** (`src/error.rs`)
Custom error types for file I/O, parsing, resolution, and plugin errors.

#### 5. **Plugin System** (`src/resolver.rs`)
Extensible architecture with `Plugin` trait, `extract_config!` macro, and recursive resolution support.

#### 6. **Utilities** (`src/utils.rs`)
Template environment setup, value management, and common utility functions.


### Data Flow

```
TOML File → Loader → Resolver → Plugins → Formatter → Output
```

1. **Load**: Parse TOML file and extract specified table
2. **Resolve**: Process table through plugins in order
3. **Format**: Convert resolved values to requested output format
4. **Output**: Display or return formatted result

### Recursive Resolution

Plugins can trigger recursive resolution of other tables using `crate::resolve_table_recursive(resolver, &table_name)?`. This ensures every table resolution goes through the complete resolver process, including plugin processing, even when referenced by other tables.

### Meta Values Implementation

The meta values system provides processing context to templates through a `_` object. Key implementation details:

- **Storage**: Meta values are stored with the `_` key in `resolver.meta_values`
- **Template Access**: Both `TemplatingPlugin` and `ImportPlugin` add the `_` object to template context
- **Utility Function**: `create_template_environment_with_meta()` in `src/utils.rs` handles environment setup
- **Conflict Avoidance**: Uses `_` for both meta values and plugin configuration
- **Scope**: Meta values are only available in templates, not in plugin configuration

## Testing

The project includes comprehensive testing:

### Test Structure
- **TOML tests**: `tests/toml_tests.rs`
- **Test cases**: `tests/toml_test_cases/*.toml`
- **Generated tests**: Build script automatically generates tests from test case files
- **Plugin tests**: Noop plugin included in all integration tests

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_basic_strings

# Run with output
cargo test test_noop_plugin -- --nocapture
```

### Test Case Format

Test cases are defined in TOML files with the following structure:

```toml
[test]
name = "Test name"
description = "Test description"
table = "table_to_extract"

[table_to_extract]
# Test data here

[expected.toml]
content = '''
# Expected TOML output
'''

[expected.json]
content = '''
{
  "expected": "json output"
}
'''

[expected.dotenv]
content = '''
key=value
'''

[expected.exports]
content = '''
export "key=value"
'''

[expected.tfvars]
content = '''
key = "value"
'''
```

**Available Test Cases:**
- **Basic functionality**: `basic_strings.toml`, `templating.toml`, `templating_with_reference.toml`
- **Plugin testing**: `import_plugin.toml`, `before_plugin.toml`, `after_plugin.toml`, `noop_plugin.toml`
- **Dependency resolution**: `before_double_reference.toml`, `after_double_reference.toml`, `recursive_plugin.toml`
- **Error handling**: `circular_reference.toml`, `env_function_errors.toml`
- **Advanced features**: `output_formats.toml`, `mixed_types.toml`, `env_functions.toml`, `meta_values.toml`
- **Reference testing**: `reference_plugin.toml`, `recursive_templating.toml`

### Error Testing

Test cases can test for expected errors by adding an `expected_error` field that accepts regex patterns for partial matching of error messages.

### Plugin Testing

The integration test framework automatically includes all built-in plugins for tests, allowing testing of various plugin combinations and recursive resolution scenarios.

## Dependencies

- **clap**: Command-line argument parsing
- **toml**: TOML parsing and serialization
- **serde_json**: JSON handling and pretty printing
- **serde**: Serialization framework
- **minijinja**: Template engine for string interpolation
- **glob**: File pattern matching (build-time)

## Error Handling

SuperTOML provides detailed error messages for file I/O, TOML parsing, table resolution, circular dependencies, plugin errors, and template errors.

## Project Structure

```
supertoml/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library exports
│   ├── error.rs         # Error types
│   ├── loader.rs        # TOML loading utilities
│   ├── formatter.rs     # Output formatting
│   ├── resolver.rs      # Core resolution logic
│   ├── utils.rs         # Shared utilities
│   └── plugins/         # Plugin implementations
├── tests/
│   ├── toml_tests.rs    # Integration tests
│   ├── cli_tests.rs     # CLI tests
│   ├── readme_validation.rs # README validation tests
│   └── toml_test_cases/ # Test case files
├── Cargo.toml           # Project configuration
├── README.md            # User documentation
└── DEVELOPMENT.md       # This file
```

## Build Script

The `build.rs` script automatically generates integration tests from TOML files in `tests/toml_test_cases/`.

## Adding New Features

### Adding a New Output Format

1. Add the format to the `OutputFormat` enum in `src/main.rs`
2. Implement the formatting logic in `src/formatter.rs`
3. Export the new function from `src/lib.rs`
4. Add CLI integration in the `run` function
5. Add README documentation and test validation
6. Run tests to ensure everything works

### Other Features

1. **New Plugin**: Implement `Plugin` trait and add to plugins directory
2. **New Error Type**: Add to `SuperTomlError` enum with appropriate display message
3. **New Test Case**: Add TOML file to `tests/toml_test_cases/` directory

## Architecture Notes

The resolver uses free functions instead of methods to avoid Rust borrowing conflicts:
- `resolve_table_recursive()` - Main resolution logic
- `process_plugins()` - Plugin processing
- `extract_table_from_file()` - Table extraction

This design allows plugins to access the resolver for recursive resolution while maintaining clean ownership semantics.

## Performance Notes

- All output formats sort keys alphabetically for consistent output
- Table processing includes cycle detection to prevent infinite loops
- Memory efficient: processes one table at a time
- Build-time test generation for fast test execution

## Future Enhancements

Potential areas for expansion:
- Configuration file support for default plugins
- Streaming support for large TOML files
- Additional output formats (YAML, XML, etc.)
- Plugin configuration validation
- Enhanced meta values (additional processing context)
- Template caching for performance
- Plugin dependency management

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

Copyright (c) 2025 Sean O'Dell

## Contributing

We welcome contributions from the community! Whether you're reporting bugs, requesting features, or submitting code, your help is appreciated.

### Bug Reports

Before reporting a bug, please:

1. **Check existing issues** - Search the [GitHub issues](https://github.com/supertoml/supertoml/issues) to see if the bug has already been reported
2. **Test with the latest version** - Make sure you're using the most recent release
3. **Provide a minimal example** - Create a small TOML file that reproduces the issue
4. **Include system information** - OS, Rust version, SuperTOML version

**Bug report template:**
```
**Description**
Brief description of the bug

**Steps to reproduce**
1. Create a TOML file with...
2. Run: supertoml file.toml table
3. Expected: [expected output]
4. Actual: [actual output]

**System information**
- OS: [e.g., macOS 14.0, Ubuntu 22.04]
- Rust version: [rustc --version]
- SuperTOML version: [supertoml --version]

**Additional context**
Any other relevant information
```

### Feature Requests

We welcome feature requests! Please:

1. **Check existing requests** - Search issues to see if the feature has been requested
2. **Explain the use case** - Describe why this feature would be useful
3. **Provide examples** - Show how you would use the feature
4. **Consider alternatives** - Is there already a way to achieve this?

**Feature request template:**
```
**Feature description**
Brief description of the requested feature

**Use case**
Explain why this feature would be useful and how you would use it

**Proposed syntax/API**
Show how the feature might work:
```bash
supertoml config.toml table --new-option value
```

**Examples**
Provide concrete examples of the feature in action

**Alternatives considered**
Are there existing ways to achieve this?
```

### Code Contributions

We welcome code contributions! Here's how to get started:

#### Prerequisites

- Rust 1.89.0 or later
- Git
- Basic familiarity with Rust

#### Development Setup

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/supertoml.git
   cd supertoml
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/supertoml/supertoml.git
   ```
4. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

#### Development Workflow

1. **Make your changes** - Write code, add tests, update documentation
2. **Run tests** - Ensure everything works:
   ```bash
   cargo test
   cargo test --release
   ```
3. **Check code quality**:
   ```bash
   cargo fmt
   cargo clippy
   ```
4. **Update documentation** - If adding features, update relevant docs
5. **Commit your changes** - Use clear, descriptive commit messages
6. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Create a pull request** - Use the PR template below

#### Pull Request Guidelines

**Before submitting:**
- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] New features have tests

**PR template:**
```
**Description**
Brief description of the changes

**Type of change**
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring

**Testing**
- [ ] Added tests for new functionality
- [ ] All existing tests pass
- [ ] Tested manually with example files

**Breaking changes**
- [ ] This PR introduces breaking changes
- [ ] Breaking changes are documented

**Additional notes**
Any other relevant information
```

#### Code Style Guidelines

- **Follow Rust conventions** - Use `cargo fmt` and `cargo clippy`
- **Write tests** - New features should include tests
- **Document public APIs** - Add doc comments for new public functions
- **Keep commits focused** - One logical change per commit
- **Use descriptive names** - Variables, functions, and types should be self-documenting

#### Areas for Contribution

Some areas where contributions are particularly welcome:

- **New output formats** - Additional format support (YAML, XML, etc.)
- **Performance improvements** - Optimizations for large files
- **Additional plugins** - Useful plugin implementations
- **Documentation** - Improvements to docs, examples, tutorials
- **Testing** - Additional test cases, edge case coverage
- **Error handling** - Better error messages, more specific error types
- **Meta values enhancements** - Additional processing context
- **Template functions** - New built-in template functions
- **Plugin ecosystem** - Community plugin registry
- **IDE support** - Language server, syntax highlighting

### Getting Help

- **GitHub Discussions** - For questions and general discussion
- **GitHub Issues** - For bugs and feature requests
- **Code review** - Ask questions in PR comments

### Recognition

Contributors will be recognized in:
- The project's README.md
- Release notes
- GitHub contributors list

Thank you for contributing to SuperTOML!
