# SuperTOML

A powerful command-line tool for extracting and processing TOML configuration data with support for multiple output formats.

## What is SuperTOML?

SuperTOML extracts specific tables from TOML files and outputs them in various formats (TOML, JSON, dotenv, shell exports, Terraform variables). It's perfect for:

- **Configuration management**: Extract specific sections from complex config files
- **Environment setup**: Convert TOML configs to environment variables
- **CI/CD pipelines**: Process configuration data in automated workflows
- **Development tools**: Quick access to specific configuration sections

## Installation

### Using mise (Recommended)

If you use [mise](https://mise.jdx.dev/) for version management:

```bash
# Install directly using ubi backend
mise install ubi:seanodell/supertoml@latest
```

Or add to your `mise.toml`:
```toml
[tools]
"ubi:seanodell/supertoml" = "latest"
```

mise will automatically download the appropriate binary for your platform from GitHub releases.

> **Note**: We're working on getting supertoml added to the mise registry, which will enable even simpler installation with just `supertoml = "latest"` in your mise.toml.

### From GitHub Releases

Download the latest binary for your platform from [GitHub Releases](https://github.com/seanodell/supertoml/releases).

### From Source

```bash
git clone https://github.com/seanodell/supertoml.git
cd supertoml
cargo build --release
```

The binary will be available at `target/release/supertoml`.

### Using Cargo

```bash
cargo install supertoml
```

## Quick Start

### Basic Usage

```bash
# Extract a table and output as TOML (default)
supertoml config.toml database

# Extract a table and output as JSON
supertoml config.toml database --output json

# Extract a table and output as dotenv file
supertoml config.toml database --output dotenv

# Extract a table and output as shell exports
supertoml config.toml database --output exports

# Extract a table and output as Terraform variables
supertoml config.toml database --output tfvars
```

### Example

Given this TOML file (`config.toml`):

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

Extract the database configuration:

```bash
supertoml config.toml database
```

**Output (TOML format):**
```toml
host = "localhost"
name = "myapp"
password = "secret"
port = 5432
user = "admin"
```

**JSON format:**
```bash
supertoml config.toml database --output json
```

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
```bash
supertoml config.toml database --output dotenv
```

```
host=localhost
name=myapp
password=secret
port=5432
user=admin
```

**Shell exports format:**
```bash
supertoml config.toml database --output exports
```

```bash
export "host=localhost"
export "name=myapp"
export "password=secret"
export "port=5432"
export "user=admin"
```

**Terraform variables format:**
```bash
supertoml config.toml database --output tfvars
```

```hcl
host = "localhost"
name = "myapp"
password = "secret"
port = 5432
user = "admin"
```

## Command Line Reference

### Syntax

```bash
supertoml <file> <table> [--output <format>]
```

### Arguments

- `file`: Path to the TOML file
- `table`: Name of the table to extract

### Options

- `--output`, `-o`: Output format
  - `toml` (default): Native TOML format
  - `json`: Pretty-printed JSON
  - `dotenv`: Environment variable format (`KEY=value`)
  - `exports`: Shell export format (`export "KEY=value"`)
  - `tfvars`: Terraform variables format (`key = "value"`)

### Examples

```bash
# Extract database config as TOML
supertoml app.toml database

# Extract server config as JSON
supertoml app.toml server -o json

# Extract logging config as dotenv
supertoml app.toml logging --output dotenv

# Extract API config as shell exports
supertoml app.toml api --output exports

# Extract Terraform config as tfvars
supertoml app.toml terraform --output tfvars
```

## Use Cases

### Configuration Management

Extract specific sections from complex configuration files:

```bash
# Extract only the database configuration
supertoml config.toml database > database.toml

# Extract only the logging configuration
supertoml config.toml logging > logging.toml
```

### Environment Variables

Convert TOML configuration to environment variables:

```bash
# Load database config as environment variables
eval $(supertoml config.toml database --output exports)

# Or save to a file
supertoml config.toml database --output dotenv > .env
```

### CI/CD Pipelines

Use in automated workflows:

```bash
# Extract version info for deployment
VERSION=$(supertoml app.toml app --output json | jq -r .version)

# Extract build configuration
supertoml build.toml production --output json > build-config.json
```

### Development Tools

Quick access to configuration data:

```bash
# Check current database settings
supertoml config.toml database --output json | jq .

# Extract API endpoints
supertoml api.toml endpoints --output json
```

## Output Formats

### TOML (Default)

Native TOML format, perfect for configuration files:

```toml
host = "localhost"
port = 5432
name = "myapp"
```

### JSON

Pretty-printed JSON, great for APIs and data processing:

```json
{
  "host": "localhost",
  "port": 5432,
  "name": "myapp"
}
```

### Dotenv

Environment variable format for `.env` files:

```
host=localhost
port=5432
name=myapp
```

### Shell Exports

Shell export format for sourcing in scripts:

```bash
export "host=localhost"
export "port=5432"
export "name=myapp"
```

### Terraform Variables (tfvars)

Terraform variables format for `.tfvars` files:

```hcl
host = "localhost"
port = 5432
name = "myapp"
```

## Error Handling

SuperTOML provides clear error messages for common issues:

- **File not found**: Clear indication when TOML files can't be read
- **Parse errors**: Detailed TOML syntax error reporting
- **Table not found**: Specific table name in error message
- **Type mismatches**: Clear indication when expected table is different type

## Examples

### Web Application Configuration

```toml
# config.toml
[app]
name = "my-web-app"
version = "1.0.0"
debug = false

[database]
host = "localhost"
port = 5432
name = "myapp"
user = "admin"
password = "secret"

[redis]
host = "localhost"
port = 6379
db = 0

[email]
smtp_host = "smtp.gmail.com"
smtp_port = 587
username = "user@gmail.com"
password = "app-password"
```

Extract specific configurations:

```bash
# Get app info
supertoml config.toml app --output json

# Get database config
supertoml config.toml database --output dotenv

# Get email config
supertoml config.toml email --output exports
```

### Docker Compose Environment

```toml
# docker.toml
[postgres]
image = "postgres:15"
port = 5432
environment = { POSTGRES_DB = "myapp", POSTGRES_USER = "admin" }

[redis]
image = "redis:7"
port = 6379

[app]
image = "myapp:latest"
port = 8080
depends_on = ["postgres", "redis"]
```

Extract service configurations:

```bash
# Get postgres config
supertoml docker.toml postgres --output json

# Get app config
supertoml docker.toml app --output json
```

## Troubleshooting

### Common Issues

**"Table not found" error**
- Check that the table name exists in your TOML file
- Table names are case-sensitive
- Use `supertoml config.toml` to see available tables

**"File not found" error**
- Verify the file path is correct
- Use absolute paths if needed: `supertoml /path/to/config.toml table`

**"Invalid TOML" error**
- Check your TOML syntax
- Use a TOML validator to verify the file

### Getting Help

- Check the [GitHub repository](https://github.com/seanodell/supertoml) for issues and discussions
- Review the [development documentation](DEVELOPMENT.md) for advanced usage
- Open an issue for bugs or feature requests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Sean O'Dell

## Contributing

[Contributing guidelines would go here]
