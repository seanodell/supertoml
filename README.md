# SuperTOML

A powerful command-line tool for extracting and processing TOML configuration data with support for multiple output formats.

## Additional Documentation

- [REFERENCE.md](REFERENCE.md) - Complete plugin and feature reference
- [CHANGELOG.md](CHANGELOG.md) - Version history and changes
- [BUILDING.md](BUILDING.md) - Build instructions and development setup
- [DEVELOPMENT.md](DEVELOPMENT.md) - Development guidelines and workflows

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
mise install ubi:supertoml/supertoml@latest
```

Or add to your `mise.toml`:
```toml
[tools]
"ubi:supertoml/supertoml" = "latest"
```

mise will automatically download the appropriate binary for your platform from GitHub releases.

> **Note**: We're working on getting supertoml added to the mise registry, which will enable even simpler installation with just `supertoml = "latest"` in your mise.toml.

### From GitHub Releases

Download the latest binary for your platform from [GitHub Releases](https://github.com/supertoml/supertoml/releases).

### From Source

```bash
git clone https://github.com/supertoml/supertoml.git
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

## Advanced Features Example

SuperTOML's power comes from its built-in plugin system that enables template processing and dependency resolution. Here's a comprehensive example:

```toml
[global]
debug = false
log_level = "info"
db_port = 5432
api_port = 443
api_version = "v1"
db_name = "myapp"
db_user = "app_user"


[dev]
_.before = ["global"]

environment = "dev"
namespace = "dev"

db_host = "db.dev.example.com"
db_password = "secret456"

api_host = "api.dev.example.com"
api_secret_key = "sk-abcdef1234567890"

_.after = ["final"]


[prod]
_.before = ["global"]
_.import = [
    { file = "mise.toml", table = "tools", key_format = "tool_{{key}}" }
]

environment = "prod"
namespace = "prod"

db_host = "db.prod.example.com"
# Example: Get password from environment variable with fallback
db_password = "{{ env_or('DB_PASSWORD', 'secret123') }}"

api_host = "api.prod.example.com"
api_secret_key = "sk-1234567890abcdef"


# These values come from the imported mise.toml file
rust_version = "{{ tool_rust }}"
act_version = "{{ tool_act }}"
gh_version = "{{ tool_gh }}"

_.after = ["final"]


[final]
app_name = "{{ environment | upper }}-MyApp"
database_url = "postgresql://{{ db_user }}:{{ db_password }}@{{ db_host }}:{{ db_port }}/{{ db_name }}"
api_endpoint = "https://{{ api_host }}:{{ api_port }}/{{ api_version }}"
full_config_path = "/etc/myapp/{{ environment }}/{{ namespace }}/config.json"

is_production = "{% if environment == 'production' %}true{% else %}false{% endif %}"
replica_count = "{% if debug %}1{% else %}3{% endif %}"

_.after = ["post-final"]


[post-final]
deployment_name = "{{ app_name | lower | replace('-', '_') }}"
services = ["PROD-MyApp-web", "PROD-MyApp-worker", "PROD-MyApp-scheduler"]
config = { database_url = "{{ database_url }}", api_endpoint = "{{ api_endpoint }}", debug = "{{ debug }}", log_level = "{{ log_level }}" }
```

Extract the fully processed production configuration:

```bash
supertoml app.toml prod --output toml
```

**Output:**
```toml
act_version = "latest"
api_endpoint = "https://api.prod.example.com:443/v1"
api_host = "api.prod.example.com"
api_port = 443
api_secret_key = "sk-1234567890abcdef"
api_version = "v1"
app_name = "PROD-MyApp"
database_url = "postgresql://app_user:secret123@db.prod.example.com:5432/myapp"
db_host = "db.prod.example.com"
db_name = "myapp"
db_password = "secret123"
db_port = 5432
db_user = "app_user"
debug = false
deployment_name = "prod_myapp"
environment = "prod"
full_config_path = "/etc/myapp/prod/prod/config.json"
gh_version = "latest"
is_production = "false"
log_level = "info"
namespace = "prod"
replica_count = "3"
rust_version = "1.89.0"
services = ["PROD-MyApp-web", "PROD-MyApp-worker", "PROD-MyApp-scheduler"]
tool_act = "latest"
tool_gh = "latest"
tool_pre-commit = "3.7.0"
tool_rust = "1.89.0"

[config]
api_endpoint = "https://api.prod.example.com:443/v1"
database_url = "postgresql://app_user:secret123@db.prod.example.com:5432/myapp"
debug = "false"
log_level = "info"
```

For JSON output:

```bash
supertoml app.toml prod --output json
```

**Output:**
```json
{
  "act_version": "latest",
  "api_endpoint": "https://api.prod.example.com:443/v1",
  "gh_version": "latest",
  "api_host": "api.prod.example.com",
  "api_port": 443,
  "api_secret_key": "sk-1234567890abcdef",
  "api_version": "v1",
  "app_name": "PROD-MyApp",
  "config": {
    "api_endpoint": "https://api.prod.example.com:443/v1",
    "database_url": "postgresql://app_user:secret123@db.prod.example.com:5432/myapp",
    "debug": "false",
    "log_level": "info"
  },
  "database_url": "postgresql://app_user:secret123@db.prod.example.com:5432/myapp",
  "db_host": "db.prod.example.com",
  "db_name": "myapp",
  "db_password": "secret123",
  "db_port": 5432,
  "db_user": "app_user",
  "debug": false,
  "deployment_name": "prod_myapp",
  "environment": "prod",
  "full_config_path": "/etc/myapp/prod/prod/config.json",
  "is_production": "false",
  "log_level": "info",
  "namespace": "prod",
  "replica_count": "3",
  "rust_version": "1.89.0",
  "services": [
    "PROD-MyApp-web",
    "PROD-MyApp-worker",
    "PROD-MyApp-scheduler"
  ],
  "tool_act": "latest",
  "tool_gh": "latest",
  "tool_pre-commit": "3.7.0",
  "tool_rust": "1.89.0"
}
```

For environment variables (dotenv):

```bash
supertoml app.toml prod --output dotenv
```

**Output:**
```
act_version=latest
api_endpoint=https://api.prod.example.com:443/v1
gh_version=latest
api_host=api.prod.example.com
api_port=443
api_secret_key=sk-1234567890abcdef
api_version=v1
app_name=PROD-MyApp
config={"api_endpoint":"https://api.prod.example.com:443/v1","database_url":"postgresql://app_user:secret123@db.prod.example.com:5432/myapp","debug":"false","log_level":"info"}
database_url=postgresql://app_user:secret123@db.prod.example.com:5432/myapp
db_host=db.prod.example.com
db_name=myapp
db_password=secret123
db_port=5432
db_user=app_user
debug=false
deployment_name=prod_myapp
environment=prod
full_config_path=/etc/myapp/prod/prod/config.json
is_production=false
log_level=info
namespace=prod
replica_count=3
rust_version=1.89.0
services=["PROD-MyApp-web","PROD-MyApp-worker","PROD-MyApp-scheduler"]
tool_act=latest
tool_gh=latest
tool_pre-commit=3.7.0
tool_rust=1.89.0
```

For shell exports:

```bash
supertoml app.toml prod --output exports
```

**Output:**
```bash
export "act_version=latest"
export "api_endpoint=https://api.prod.example.com:443/v1"
export "gh_version=latest"
export "api_host=api.prod.example.com"
export "api_port=443"
export "api_secret_key=sk-1234567890abcdef"
export "api_version=v1"
export "app_name=PROD-MyApp"
export "config={\"api_endpoint\":\"https://api.prod.example.com:443/v1\",\"database_url\":\"postgresql://app_user:secret123@db.prod.example.com:5432/myapp\",\"debug\":\"false\",\"log_level\":\"info\"}"
export "database_url=postgresql://app_user:secret123@db.prod.example.com:5432/myapp"
export "db_host=db.prod.example.com"
export "db_name=myapp"
export "db_password=secret123"
export "db_port=5432"
export "db_user=app_user"
export "debug=false"
export "deployment_name=prod_myapp"
export "environment=prod"
export "full_config_path=/etc/myapp/prod/prod/config.json"
export "is_production=false"
export "log_level=info"
export "namespace=prod"
export "replica_count=3"
export "rust_version=1.89.0"
export "services=[\"PROD-MyApp-web\",\"PROD-MyApp-worker\",\"PROD-MyApp-scheduler\"]"
export "tool_act=latest"
export "tool_gh=latest"
export "tool_pre-commit=3.7.0"
export "tool_rust=1.89.0"
```

For Terraform variables:

```bash
supertoml app.toml prod --output tfvars
```

**Output:**
```hcl
act_version = "latest"
api_endpoint = "https://api.prod.example.com:443/v1"
gh_version = "latest"
api_host = "api.prod.example.com"
api_port = 443
api_secret_key = "sk-1234567890abcdef"
api_version = "v1"
app_name = "PROD-MyApp"
config = {api_endpoint = "https://api.prod.example.com:443/v1", database_url = "postgresql://app_user:secret123@db.prod.example.com:5432/myapp", debug = "false", log_level = "info"}
database_url = "postgresql://app_user:secret123@db.prod.example.com:5432/myapp"
db_host = "db.prod.example.com"
db_name = "myapp"
db_password = "secret123"
db_port = 5432
db_user = "app_user"
debug = false
deployment_name = "prod_myapp"
environment = "prod"
full_config_path = "/etc/myapp/prod/prod/config.json"
is_production = "false"
log_level = "info"
namespace = "prod"
replica_count = "3"
rust_version = "1.89.0"
services = ["PROD-MyApp-web", "PROD-MyApp-worker", "PROD-MyApp-scheduler"]
tool_act = "latest"
tool_gh = "latest"
tool_pre-commit = "3.7.0"
tool_rust = "1.89.0"
```

### Variable Resolution Order

**Important**: Variables can only reference values that were defined in previously processed tables. The processing order is determined by the dependency chain:

1. **Dependencies first**: Tables listed in `_.before` are processed before the current table
2. **Current table**: The target table is processed with access to all previously resolved variables
3. **Post-processing**: Tables listed in `_.after` are processed after the current table

In the example above:
- `global` table is processed first (via `_.before = ["global"]`)
- `prod` table can reference variables from `global` (like `{{ db_port }}`, `{{ api_version }}`)
- `final` table can reference variables from both `global` and `prod` (like `{{ env }}`, `{{ db_user }}`)
- `post-final` table can reference variables from all previous tables (like `{{ app_name }}`, `{{ database_url }}`)

**This means you cannot reference a variable within the same table where it's defined**, and you cannot reference variables from tables that haven't been processed yet.

### Key Features Demonstrated

- **Multi-stage Plugin Processing**: Uses `before`, `import`, `templating`, and `after` plugins in sequence
- **Dependency Chain**: `_.before = ["global"]` loads base configuration, `_.after = ["final"]` for post-processing
- **External Configuration Import**: `_.import` for loading key/value pairs from external TOML files with optional key transformation
- **Template Processing**: `{{ variable }}` syntax for dynamic value substitution
- **Advanced Template Filters**: `| upper`, `| lower`, `| replace()` for string transformations
- **Custom Template Functions**: Built-in functions for environment variables and more
- **Conditional Logic**: `{% if env == 'production' %}` statements for environment-specific configuration
- **Recursive Template Resolution**: Templates in arrays and objects are fully resolved (e.g., `services` array and `config` object)
- **Cross-table Variable Access**: Variables from `database`, `api`, and other dependency tables
- **Multiple Output Formats**: TOML, JSON, dotenv, shell exports, and Terraform variables (.tfvars)
- **Complex Data Structures**: Nested objects and arrays with fully resolved template variables
- **Environment-specific Configuration**: Dynamic values based on deployment environment

## Error Handling

SuperTOML provides clear error messages for common issues:

- **File not found**: Clear indication when TOML files can't be read
- **Parse errors**: Detailed TOML syntax error reporting
- **Table not found**: Specific table name in error message
- **Type mismatches**: Clear indication when expected table is different type

## Custom Template Functions

SuperTOML provides built-in custom functions that can be used within Minijinja templates:

### Environment Variable Functions

- **`env(name)`**: Returns the value of an environment variable. Throws an error if the variable doesn't exist.
- **`env_or(name, default)`**: Returns the value of an environment variable, or the default value if the variable doesn't exist.

**Example:**
```toml
[config]
database_url = "{{ env('DATABASE_URL') }}"
debug_mode = "{{ env_or('DEBUG', 'false') }}"
app_version = "{{ env_or('APP_VERSION', '1.0.0') }}"
log_level = "{{ env_or('LOG_LEVEL', 'info') | upper }}"
```

These functions are particularly useful for:
- Loading secrets from environment variables
- Providing sensible defaults for optional configuration
- Creating environment-specific configurations
- Integrating with deployment systems that inject environment variables

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

- Check the [GitHub repository](https://github.com/supertoml/supertoml) for issues and discussions
- Review the [development documentation](DEVELOPMENT.md) for advanced usage
- Open an issue for bugs or feature requests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Sean O'Dell

## Contributing

[Contributing guidelines would go here]
