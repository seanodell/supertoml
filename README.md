# SuperTOML

A powerful TOML processing tool and library written in Rust that extracts, resolves, and formats TOML table data with support for multiple output formats and extensible plugin architecture.

## Overview

SuperTOML is both a command-line tool and a Rust library designed to work with TOML configuration files. It can extract specific tables from TOML files, process them through configurable plugins, and output the results in various formats including TOML, JSON, dotenv, and shell exports.

## Features

- **Table Extraction**: Extract specific tables from TOML files
- **Multiple Output Formats**: Support for TOML, JSON, dotenv, and shell export formats
- **Plugin Architecture**: Extensible plugin system for custom data processing
- **Cycle Detection**: Prevents infinite loops when processing table dependencies
- **Type-Safe Parsing**: Leverages Rust's type system for safe TOML parsing
- **Comprehensive Error Handling**: Detailed error messages for debugging

## Installation

### Prerequisites

- Rust 1.89.0 or later (specified in `mise.toml`)

### Building from Source

```bash
git clone <repository-url>
cd supertoml
cargo build --release
```

The binary will be available at `target/release/supertoml`.

## Usage

### Command Line Interface

```bash
supertoml <file> <table> [--output <format>]
```

**Arguments:**
- `file`: Path to the TOML file
- `table`: Name of the table to extract

**Options:**
- `--output`, `-o`: Output format (toml, json, dotenv, exports) [default: toml]

### Examples

```bash
# Extract a table and output as TOML (default)
supertoml config.toml database

# Extract a table and output as JSON
supertoml config.toml database --output json

# Extract a table and output as dotenv file
supertoml config.toml database --output dotenv

# Extract a table and output as shell exports
supertoml config.toml database --output exports
```

### Sample TOML File

```toml
[database]
host = "localhost"
port = 5432
name = "myapp"
user = "admin"
password = "secret"

[server]
host = "0.0.0.0"
port = 8080
debug = true
```

### Output Examples

**JSON format:**
```json
{
  "host": "localhost",
  "name": "myapp",
  "password": "secret",
  "port": 5432,
  "user": "admin"
}
```

**Dotenv format:**
```
host=localhost
name=myapp
password=secret
port=5432
user=admin
```

**Shell exports format:**
```bash
export "host=localhost"
export "name=myapp"
export "password=secret"
export "port=5432"
export "user=admin"
```

## Library Usage

SuperTOML can also be used as a Rust library:

```rust
use supertoml::{Resolver, format_as_json};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut resolver = Resolver::new(vec![]);
    let values = resolver.resolve_table("config.toml", "database")?;
    let json_output = format_as_json(&values)?;
    println!("{}", json_output);
    Ok(())
}
```

## Plugin System

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
        config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        let config: MyPluginConfig = extract_config!(config, MyPluginConfig, self.name())?;
        
        // Use config.option1, config.option2, etc.
        // Custom processing logic here
        // Access resolver.values, resolver.toml_file, etc.
        // Call resolver.resolve_table_recursive() for recursive processing
        Ok(())
    }
}
```

### Plugin Configuration

Plugins are configured in TOML files using a special `_` key within the table:

```toml
[my_table]
key1 = "value1"
key2 = "value2"
_.my_plugin = { option1 = "config_value", option2 = 42 }
```

### Using Plugins

```rust
use supertoml::{Resolver, format_as_json};
use supertoml::plugins::NoopPlugin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plugins: Vec<&'static dyn supertoml::Plugin> = vec![
        &NoopPlugin,
    ];
    
    let mut resolver = Resolver::new(plugins);
    let values = resolver.resolve_table("config.toml", "my_table")?;
    let json_output = format_as_json(&values)?;
    println!("{}", json_output);
    Ok(())
}
```

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

The integration test framework automatically includes both NoopPlugin and ReferencePlugin for all tests:

```toml
[test]
name = "Plugin test"
description = "Test plugin functionality"
table = "config"

[config]
app_name = "test-app"
_.noop = { message = "Plugin executed", enabled = true }
_.reference = { table = "other_table", prefix = "ref_" }
```

This allows testing of both simple plugins and recursive resolution scenarios.

## Dependencies

- **clap**: Command-line argument parsing with derive macros
- **toml**: TOML parsing and serialization
- **serde_json**: JSON handling and pretty printing
- **serde**: Serialization framework
- **glob**: File pattern matching (build-time)

## Error Handling

SuperTOML provides detailed error messages for common issues:

- **File not found**: Clear indication when TOML files can't be read
- **Parse errors**: Detailed TOML syntax error reporting
- **Table not found**: Specific table name in error message
- **Type mismatches**: Clear indication when expected table is different type
- **Cycle detection**: Prevents infinite loops with descriptive error
- **Plugin errors**: Plugin-specific error reporting with context

## Development

### Project Structure

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

### Build Script

The `build.rs` script automatically generates integration tests from TOML files in `tests/test_cases/`. This ensures all test cases are automatically included when new test files are added.

### Adding New Features

1. **New Output Format**: Add to `OutputFormat` enum and implement in `formatter.rs`
2. **New Plugin**: Implement `Plugin` trait and add to plugins directory
3. **New Error Type**: Add to `SuperTomlError` enum with appropriate display message
4. **New Test Case**: Add TOML file to `tests/test_cases/` directory

### Architecture Notes

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
- Built-in plugins for common transformations
- Configuration file support for default plugins
- Streaming support for large TOML files
- Template interpolation support
- Environment variable substitution

## License

[License information would go here]

## Contributing

[Contributing guidelines would go here]
