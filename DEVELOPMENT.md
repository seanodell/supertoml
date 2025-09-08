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
git clone https://github.com/seanodell/supertoml.git
cd supertoml
cargo build --release
```

The binary will be available at `target/release/supertoml`.

## Library Usage

SuperTOML can also be used as a Rust library:

```rust
use supertoml::{Resolver, format_as_json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use standard plugins (before, templating, after)
    let mut resolver = Resolver::new(vec![
        &supertoml::plugins::BeforePlugin as &dyn supertoml::Plugin,
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

Plugins allow custom processing of table data with type-safe configuration:

```rust
use supertoml::{Plugin, SuperTomlError, extract_config};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
struct MyPluginConfig {
    option1: String,
    option2: Option<i32>,
}

struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my_plugin"
    }

    fn process(
        &self,
        resolver: &mut supertoml::Resolver,
        table_values: &mut HashMap<String, toml::Value>,
        config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        let config: MyPluginConfig = extract_config!(config, MyPluginConfig, self.name())?;

        // Use config.option1, config.option2, etc.
        // Custom processing logic here
        // Access resolver.values, resolver.toml_file, etc.
        // Call resolver.resolve_table_recursive() for recursive processing

        // Important: Plugins can modify table_values, but should not drain them
        // as they may be passed to other plugins in the processing chain

        // Add values to resolver.values if needed
        for (key, value) in table_values.iter() {
            resolver.values.insert(key.clone(), value.clone());
        }

        Ok(())
    }
}
```

### Plugin Behavior

Plugins receive three parameters:
- `resolver`: Access to the resolver for recursive table resolution and global values
- `table_values`: The current table's key-value pairs (can be modified but not drained)
- `config`: Plugin-specific configuration from the TOML file

**Important**: Plugins should not drain `table_values` because they may be passed to other plugins in the processing chain. Use `.iter()` to copy values to `resolver.values` rather than `.drain()`.

If a plugin modifies `table_values`, it should also update `resolver.values` to match the modified values, so that if later plugins re-add the table values to `resolver.values`, they get the modified values, not the original ones.

### Plugin Configuration

Plugins are configured in TOML files using a special `_` key within the table:

```toml
[my_table]
key1 = "value1"
key2 = "value2"
_.my_plugin = { option1 = "config_value", option2 = 42 }
```

### Using Plugins

#### Standard Plugins (Recommended)
```rust
use supertoml::{Resolver, format_as_json};
use supertoml::plugins::{TemplatingPlugin, BeforePlugin, AfterPlugin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plugins: Vec<&'static dyn supertoml::Plugin> = vec![
        &BeforePlugin,
        &TemplatingPlugin,
        &AfterPlugin,
    ];

    let mut resolver = Resolver::new(plugins);
    let values = resolver.resolve_table("config.toml", "my_table")?;
    let json_output = format_as_json(&values)?;
    println!("{}", json_output);
    Ok(())
}
```

#### Development/Testing Plugins
```rust
use supertoml::{Resolver, format_as_json};
use supertoml::plugins::{NoopPlugin, ReferencePlugin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plugins: Vec<&'static dyn supertoml::Plugin> = vec![
        &NoopPlugin,
        &ReferencePlugin,
    ];

    let mut resolver = Resolver::new(plugins);
    let values = resolver.resolve_table("config.toml", "my_table")?;
    let json_output = format_as_json(&values)?;
    println!("{}", json_output);
    Ok(())
}
```

### Built-in Plugins

SuperTOML comes with several built-in plugins, categorized as follows:

#### Standard Plugins (Included by Default)
These plugins are automatically included when using SuperTOML and provide core functionality:

**TemplatingPlugin**
Processes string values through Minijinja templating using `resolver.values` as context. Always runs and requires no configuration.

**BeforePlugin**
Resolves multiple tables before processing the current table. Configuration:
```toml
_.before = ["table1", "table2", "table3"]
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

### Plugin Config Types

Your plugin config can be any valid TOML type:

```toml
# Simple string
_.simple = "hello"

# Number
_.count = 42

# Boolean
_.enabled = true

# String array
_.list = ["item1", "item2", "item3"]

# Complex structure
_.complex = {
    database = { host = "localhost", port = 5432 },
    cache = { ttl = 300, size = 1000 }
}
```

## Architecture

### Core Components

#### 1. **Resolver** (`src/resolver.rs`)
The heart of SuperTOML, responsible for:
- Loading and parsing TOML files
- Extracting specific tables
- Processing tables through plugins
- Detecting and preventing circular dependencies
- Managing resolved values

#### 2. **Loader** (`src/loader.rs`)
Handles TOML file operations:
- File reading and parsing
- Table extraction utilities
- Type conversion traits (`FromTomlValue`)
- Helper extensions for TOML tables

#### 3. **Formatter** (`src/formatter.rs`)
Converts resolved data to various output formats:
- **TOML**: Native TOML format
- **JSON**: Pretty-printed JSON
- **Dotenv**: Environment variable format (`KEY=value`)
- **Exports**: Shell export format (`export "KEY=value"`)
- **Tfvars**: Terraform variables format (`key = "value"`)

#### 4. **Error Handling** (`src/error.rs`)
Comprehensive error types:
- File I/O errors
- TOML parsing errors
- Table resolution errors
- Plugin errors
- Cycle detection errors

#### 5. **Plugin System** (`src/resolver.rs`)
Extensible architecture for custom processing:
- Simple `Plugin` trait with type-safe configuration extraction
- `extract_config!` macro for easy deserialization
- Support for any TOML data type as configuration
- Full recursive resolution support - plugins can resolve other tables
- Ownership transfer design to avoid Rust borrowing conflicts

### Data Flow

```
TOML File → Loader → Resolver → Plugins → Formatter → Output
```

1. **Load**: Read and parse TOML file
2. **Extract**: Extract specified table
3. **Resolve**: Process through resolver and plugins (with recursive resolution)
4. **Format**: Convert to requested output format
5. **Output**: Display or return formatted result

### Recursive Resolution

Plugins can trigger recursive resolution of other tables within the same TOML file:

```rust
// Inside a plugin's process method
crate::resolve_table_recursive(resolver, &config.table)?;
```

This ensures that every table resolution goes through the complete resolver process, including plugin processing, even when referenced by other tables.

## Testing

The project includes comprehensive testing:

### Test Structure
- **Integration tests**: `tests/integration_tests.rs`
- **Test cases**: `tests/test_cases/*.toml`
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
```

### Error Testing

Test cases can also test for expected errors by adding an `expected_error` field:

```toml
[test]
name = "Error test"
description = "Test that specific errors are raised"
table = "problematic_table"
expected_error = "Cycle detected"

[problematic_table]
# This will cause a cycle detection error
_.reference = { table = "problematic_table" }
```

The `expected_error` field accepts a regex pattern for partial matching of error messages.

### Plugin Testing

The integration test framework automatically includes all built-in plugins for tests:

```toml
[test]
name = "Plugin test"
description = "Test plugin functionality"
table = "config"

[config]
app_name = "test-app"
_.reference = { table = "other_table" }
_.before = ["setup", "init"]
_.after = ["cleanup", "logging"]
```

This allows testing of various plugin combinations and recursive resolution scenarios.

## Dependencies

- **clap**: Command-line argument parsing with derive macros
- **toml**: TOML parsing and serialization
- **serde_json**: JSON handling and pretty printing
- **serde**: Serialization framework
- **minijinja**: Template engine for string interpolation and templating plugin
- **glob**: File pattern matching (build-time)

## Error Handling

SuperTOML provides detailed error messages for common issues:

- **File not found**: Clear indication when TOML files can't be read
- **Parse errors**: Detailed TOML syntax error reporting
- **Table not found**: Specific table name in error message
- **Type mismatches**: Clear indication when expected table is different type
- **Cycle detection**: Prevents infinite loops with descriptive error
- **Plugin errors**: Plugin-specific error reporting with context

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
│   └── plugins/         # Plugin implementations
│       ├── mod.rs       # Plugin module exports
│       ├── noop.rs      # Example noop plugin
│       └── reference.rs # Example reference plugin with recursive resolution
├── tests/
│   ├── integration_tests.rs    # Test framework
│   └── test_cases/            # Test case definitions
├── build.rs            # Build script for test generation
├── Cargo.toml          # Project configuration
└── mise.toml          # Tool version specification
```

## Build Script

The `build.rs` script automatically generates integration tests from TOML files in `tests/test_cases/`. This ensures all test cases are automatically included when new test files are added.

## Adding New Features

### Adding a New Output Format

To add a new output format (e.g., YAML, XML), follow these steps:

1. **Add to CLI enum**: Add the new format to the `OutputFormat` enum in `src/main.rs`
2. **Implement formatter**: Add a `format_as_<format>` function in `src/formatter.rs`
3. **Export from library**: Add the new function to the exports in `src/lib.rs`
4. **Add CLI integration**: Add a new match arm in the `run` function in `src/main.rs`
5. **Add README documentation**: Add an example output section in the "Advanced Features Example" in `README.md`
6. **Add test validation**: Add the new format to the `get_output_formats()` function in `tests/readme_validation.rs` with appropriate start/end markers
7. **Test the implementation**: Run `cargo test` to ensure all tests pass

**Example for adding YAML format:**
```rust
// In src/formatter.rs
pub fn format_as_yaml(values: &HashMap<String, toml::Value>) -> Result<String, SuperTomlError> {
    // Implementation here
}

// In src/lib.rs
pub use formatter::{
    format_as_dotenv, format_as_exports, format_as_json, format_as_tfvars, format_as_toml, format_as_yaml,
};

// In src/main.rs OutputFormat enum
#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Toml,
    Json,
    Dotenv,
    Exports,
    Tfvars,
    Yaml, // Add here
}

// In tests/readme_validation.rs get_output_formats()
OutputFormat {
    name: "yaml",
    format_fn: format_as_yaml,
    start_marker: "For YAML output:\n\n```bash\nsupertoml app.toml prod --output yaml\n```\n\n**Output:**\n```yaml\n",
    end_marker: "\n```\n\nFor [next format]:",
    assert_fn: assert_string_equivalent, // or custom assertion function
},
```

### Other Features

1. **New Plugin**: Implement `Plugin` trait and add to plugins directory
2. **New Error Type**: Add to `SuperTomlError` enum with appropriate display message
3. **New Test Case**: Add TOML file to `tests/test_cases/` directory

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
- Environment variable substitution
- Additional output formats (YAML, XML, etc.)
- Plugin configuration validation

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

Copyright (c) 2025 Sean O'Dell

## Contributing

We welcome contributions from the community! Whether you're reporting bugs, requesting features, or submitting code, your help is appreciated.

### Bug Reports

Before reporting a bug, please:

1. **Check existing issues** - Search the [GitHub issues](https://github.com/seanodell/supertoml/issues) to see if the bug has already been reported
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
   git remote add upstream https://github.com/seanodell/supertoml.git
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
