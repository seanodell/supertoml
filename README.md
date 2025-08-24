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
    // Create a resolver with no plugins
    let mut resolver = Resolver::new(vec![]);
    
    // Resolve a table from a TOML file
    let values = resolver.resolve_table("config.toml", "database")?;
    
    // Format as JSON
    let json_output = format_as_json(&values)?;
    println!("{}", json_output);
    
    Ok(())
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
- Plugin trait definition
- Plugin data processing
- Error handling and reporting

### Data Flow

```
TOML File → Loader → Resolver → Plugins → Formatter → Output
```

1. **Load**: Read and parse TOML file
2. **Extract**: Extract specified table
3. **Resolve**: Process through resolver and plugins
4. **Format**: Convert to requested output format
5. **Output**: Display or return formatted result

### Plugin System

Plugins allow custom processing of table data. To create a plugin:

```rust
use supertoml::{Plugin, SuperTomlError};
use std::collections::HashMap;

struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my_plugin"
    }
    
    fn process(
        &mut self,
        values: &mut HashMap<String, toml::Value>,
        plugin_data: toml::Value,
    ) -> Result<(), SuperTomlError> {
        // Custom processing logic here
        Ok(())
    }
}
```

Plugins are configured in TOML files using a special `_` key within the table:

```toml
[my_table]
key1 = "value1"
key2 = "value2"
_.my_plugin = { option1 = "config_value" }
```

## Testing

The project includes comprehensive testing:

### Test Structure
- **Integration tests**: `tests/integration_tests.rs`
- **Test cases**: `tests/test_cases/*.toml`
- **Generated tests**: Build script automatically generates tests from test case files

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_basic_strings
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
│   ├── plugins/         # Plugin implementations (empty)
│   └── bin/             # Additional binaries (empty)
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
2. **New Plugin**: Implement `Plugin` trait and register with resolver
3. **New Error Type**: Add to `SuperTomlError` enum with appropriate display message
4. **New Test Case**: Add TOML file to `tests/test_cases/` directory

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
