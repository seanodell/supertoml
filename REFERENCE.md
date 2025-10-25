# SuperTOML Reference

This document provides a comprehensive reference for all available plugins, templating features, and output formats in SuperTOML.

## Table of Contents

- [Plugins](#plugins)
  - [Before Plugin](#before-plugin)
  - [Import Plugin](#import-plugin)
  - [After Plugin](#after-plugin)
- [Templating](#templating)
  - [How Templating Works in SuperTOML](#how-templating-works-in-supertoml)
  - [Template Context](#template-context)
  - [Jinja2 Documentation](#jinja2-documentation)
  - [SuperTOML-Specific Features](#supertoml-specific-features)
  - [Meta Values](#meta-values)
- [Output Formats](#output-formats)

## Plugins

SuperTOML uses a plugin-based architecture where each plugin processes configuration directives that start with `_.`. Plugins are executed in a specific order to ensure proper value resolution.

### Before Plugin

**Plugin Name:** `before`
**Directive:** `_.before`

Processes tables before the current table is resolved. This is useful for setting up dependencies or prerequisites.

#### Syntax

```toml
_.before = ["table1", "table2", "table3"]
```

#### Example

```toml
[app_config]
app_name = "MyApp"
version = "1.0.0"

# Process database config before app config
_.before = ["database_config"]

# Now we can use database values
db_url = "{{ db_host }}:{{ db_port }}/{{ db_name }}"

[database_config]
db_host = "localhost"
db_port = 5432
db_name = "myapp_db"
```

#### Use Cases

- Setting up configuration dependencies
- Ensuring prerequisite values are available
- Organizing configuration in logical processing order

### Import Plugin

**Plugin Name:** `import`
**Directive:** `_.import`

Imports key-value pairs from external TOML files with optional key transformation using templates.

#### Syntax

```toml
_.import = [
    { file = "path/to/file.toml", table = "table_name", key_format = "prefix_{{key}}" },
    { file = "another/file.toml", table = "other_table" }
]
```

#### Parameters

- `file`: Path to the external TOML file (relative to current file)
- `table`: Name of the table to import from
- `key_format`: Optional template for transforming imported keys

#### Example

```toml
[main_config]
app_name = "MyApp"

# Import database config with db_ prefix
_.import = [
    { file = "database.toml", table = "database", key_format = "db_{{key}}" },
    { file = "redis.toml", table = "redis" }
]

# Use imported values
database_url = "{{ db_host }}:{{ db_port }}"
redis_url = "{{ host }}:{{ port }}"
```

#### Key Format Templates

The `key_format` parameter supports Jinja2 templating with the `key` variable:

```toml
# Original key: "host" -> Transformed key: "db_host"
key_format = "db_{{key}}"

# Original key: "port" -> Transformed key: "database_port"
key_format = "database_{{key}}"

# Original key: "name" -> Transformed key: "app_name"
key_format = "app_{{key}}"
```

#### Use Cases

- Sharing configuration across multiple files
- Organizing configuration by domain (database, redis, etc.)
- Applying consistent naming conventions

### After Plugin

**Plugin Name:** `after`
**Directive:** `_.after`

Processes tables after the current table is resolved. This is useful for cleanup or post-processing operations.

#### Syntax

```toml
_.after = ["table1", "table2", "table3"]
```

#### Example

```toml
[main_config]
app_name = "MyApp"

# Process cleanup after main config
_.after = ["cleanup_config"]

[cleanup_config]
# Cleanup operations
temp_files = ["/tmp/app.log", "/tmp/app.pid"]
```

#### Use Cases

- Post-processing operations
- Cleanup tasks
- Final configuration adjustments

## Templating

SuperTOML provides built-in Jinja2 templating support through [minijinja](https://docs.rs/minijinja/latest/minijinja/). All string values containing template syntax are automatically processed during configuration resolution.

### How Templating Works in SuperTOML

Templates are processed automatically on any string value that contains Jinja2 syntax (`{{`, `{%`, or `{#`). This happens after imports and before post-processing operations.

```toml
[app_config]
app_name = "SuperTOML"
version = "1.0.0"

# Simple variable reference
description = "{{ app_name }} v{{ version }}"

# Conditional logic
debug_mode = "{% if debug %}DEBUG{% else %}PRODUCTION{% endif %}"

# Using filters
app_name_upper = "{{ app_name | upper }}"
```

### Template Context

The template context includes:
- **All resolved configuration values** from the current and imported tables
- **Meta values** providing processing context (see [Meta Values](#meta-values) below)
- **Available variables** are determined by the processing order

### Jinja2 Documentation

SuperTOML uses minijinja for templating, which supports the full Jinja2 syntax. For complete documentation on:

- **Template syntax**: Variables, filters, conditionals, loops
- **Available filters**: `upper`, `lower`, `title`, `length`, `join`, etc.
- **Control structures**: `if/else`, `for` loops, etc.
- **Expressions**: String slicing, math operations, etc.

See the [minijinja documentation](https://docs.rs/minijinja/latest/minijinja/) or the [Jinja2 documentation](https://jinja.palletsprojects.com/).

### SuperTOML-Specific Features

SuperTOML extends the standard Jinja2 functionality with additional built-in functions:

#### Environment Variable Functions

##### `env(name)`

Gets an environment variable value. Throws an error if the variable doesn't exist.

```toml
database_url = "{{ env('DATABASE_URL') }}"
api_key = "{{ env('API_KEY') }}"
```

##### `env_or(name, default)`

Gets an environment variable value with a fallback default.

```toml
debug_mode = "{{ env_or('DEBUG', 'false') }}"
port = "{{ env_or('PORT', '8080') }}"
```

#### Example Usage

```toml
[app_config]
# Required environment variables
database_url = "{{ env('DATABASE_URL') }}"
api_key = "{{ env('API_KEY') }}"

# Optional with defaults
debug_mode = "{{ env_or('DEBUG', 'false') }}"
port = "{{ env_or('PORT', '8080') }}"
log_level = "{{ env_or('LOG_LEVEL', 'info') }}"

# Conditional based on environment
environment = "{{ env_or('ENVIRONMENT', 'development') }}"
is_production = "{% if environment == 'production' %}true{% else %}false{% endif %}"
```

### Meta Values

SuperTOML provides access to processing context through the `_` (underscore) object in templates.

#### Available Meta Values

The `_` object contains the following structure:

```toml
_ = {
    args = {
        file_path = "path/to/current/file.toml",
        table_name = "current_table_name",
        output_format = "toml"  # or "json", "dotenv", "exports", "tfvars"
    }
}
```

#### Usage Examples

```toml
[app_config]
# Access processing context
current_file = "{{ _.args.file_path }}"
current_table = "{{ _.args.table_name }}"
output_format = "{{ _.args.output_format }}"

# Use in conditional logic
is_json_output = "{% if _.args.output_format == 'json' %}true{% else %}false{% endif %}"

# Format with filters
formatted_table = "{{ _.args.table_name | upper }}"
formatted_format = "{{ _.args.output_format | upper }}"

# Combine with other values
debug_info = "Processing {{ _.args.table_name }} from {{ _.args.file_path }}"
```

#### Use Cases

- Dynamic configuration based on output format
- Debug information and logging
- Conditional logic based on processing context
- File and table name references in templates

## Output Formats

SuperTOML supports multiple output formats:

- **TOML** (`toml`): Standard TOML format
- **JSON** (`json`): JSON format
- **Dotenv** (`dotenv`): Environment variable format
- **Exports** (`exports`): Shell export format
- **Terraform Variables** (`tfvars`): Terraform variables format

### Format Examples

#### TOML Output
```toml
app_name = "MyApp"
version = "1.0.0"
debug = true
```

#### JSON Output
```json
{
  "app_name": "MyApp",
  "version": "1.0.0",
  "debug": true
}
```

#### Dotenv Output
```
app_name=MyApp
version=1.0.0
debug=true
```

#### Exports Output
```bash
export "app_name=MyApp"
export "version=1.0.0"
export "debug=true"
```

#### Terraform Variables Output
```
app_name = "MyApp"
version = "1.0.0"
debug = true
```

## Processing Order

SuperTOML processes configuration in the following order:

1. **Before Plugin** - Processes `_.before` directives
2. **Import Plugin** - Processes `_.import` directives
3. **Templating** - Processes all template expressions automatically
4. **After Plugin** - Processes `_.after` directives

This order ensures that:
- Dependencies are resolved before the current table
- External values are imported and available for templating
- All template expressions are processed with full context
- Post-processing operations can clean up or finalize values

## Best Practices

1. **Use descriptive table names** that clearly indicate their purpose
2. **Organize configuration logically** using before/after plugins
3. **Use consistent naming conventions** with import key formats
4. **Leverage meta values** for dynamic configuration
5. **Test with different output formats** to ensure compatibility
6. **Use environment variables** for sensitive or environment-specific values
7. **Keep templates simple** and readable
8. **Document complex template logic** with comments

## Error Handling

SuperTOML provides detailed error messages for common issues:

- **Template syntax errors**: Clear indication of template parsing problems
- **Missing environment variables**: Specific error when `env()` function fails
- **Circular references**: Detection and reporting of circular dependencies
- **File not found**: Clear error messages for missing import files
- **Invalid plugin configuration**: Detailed error messages for plugin configuration issues

## Examples

See the `tests/toml_test_cases/` directory for comprehensive examples of all plugins and features in action.
