//! Utility functions shared across plugins

use crate::SuperTomlError;
use minijinja::{Environment, Value as JinjaValue};
use std::collections::HashMap;

/// Convert a TOML value to a Jinja value for template rendering
///
/// This function recursively converts TOML values to their Jinja equivalents,
/// enabling template rendering with proper type preservation.
pub fn toml_value_to_jinja(value: &toml::Value) -> JinjaValue {
    match value {
        toml::Value::String(s) => JinjaValue::from(s.as_str()),
        toml::Value::Integer(i) => JinjaValue::from(*i),
        toml::Value::Float(f) => JinjaValue::from(*f),
        toml::Value::Boolean(b) => JinjaValue::from(*b),
        toml::Value::Array(arr) => {
            JinjaValue::from(arr.iter().map(toml_value_to_jinja).collect::<Vec<_>>())
        }
        toml::Value::Table(table) => JinjaValue::from(
            table
                .iter()
                .map(|(k, v)| (k.clone(), toml_value_to_jinja(v)))
                .collect::<HashMap<_, _>>(),
        ),
        toml::Value::Datetime(dt) => JinjaValue::from(dt.to_string()),
    }
}

/// Add values from table_values to resolver.values
///
/// This is a common pattern used by most plugins to propagate their
/// processed values to the global resolver context.
pub fn add_values_to_resolver(
    resolver: &mut crate::Resolver,
    table_values: &HashMap<String, toml::Value>,
) {
    for (key, value) in table_values {
        resolver.values.insert(key.clone(), value.clone());
    }
}

/// Create a shared Minijinja environment for template processing
///
/// This ensures consistent template environment setup across all plugins
/// that need templating functionality. Custom functions are registered here.
pub fn create_template_environment() -> Environment<'static> {
    let mut env = Environment::new();

    // Add custom function to access environment variables
    env.add_function("env", |name: String| -> Result<String, minijinja::Error> {
        std::env::var(&name).map_err(|_| {
            minijinja::Error::new(
                minijinja::ErrorKind::UndefinedError,
                format!("Environment variable '{}' not found", name),
            )
        })
    });

    // Add custom function to get environment variable with default value
    env.add_function("env_or", |name: String, default: String| -> String {
        std::env::var(&name).unwrap_or(default)
    });

    env
}

/// Create a standardized template-related error
///
/// This provides consistent error formatting for template operations
/// across all plugins.
pub fn template_error(
    plugin_name: &str,
    operation: &str,
    error: impl std::fmt::Display,
) -> SuperTomlError {
    SuperTomlError::PluginError {
        plugin_name: plugin_name.to_string(),
        error: format!("{}: {}", operation, error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;

    #[test]
    fn test_toml_string_to_jinja() {
        let toml_val = Value::String("hello".to_string());
        let jinja_val = toml_value_to_jinja(&toml_val);
        assert_eq!(jinja_val.to_string(), "hello");
    }

    #[test]
    fn test_toml_integer_to_jinja() {
        let toml_val = Value::Integer(42);
        let jinja_val = toml_value_to_jinja(&toml_val);
        assert_eq!(jinja_val.to_string(), "42");
    }

    #[test]
    fn test_toml_boolean_to_jinja() {
        let toml_val = Value::Boolean(true);
        let jinja_val = toml_value_to_jinja(&toml_val);
        assert_eq!(jinja_val.to_string(), "true");
    }

    #[test]
    fn test_toml_array_to_jinja() {
        let toml_val = Value::Array(vec![Value::String("first".to_string()), Value::Integer(2)]);
        let jinja_val = toml_value_to_jinja(&toml_val);
        assert!(jinja_val.as_seq().is_some());
    }

    #[test]
    fn test_toml_table_to_jinja() {
        let mut table = toml::map::Map::new();
        table.insert("key1".to_string(), Value::String("value1".to_string()));
        table.insert("key2".to_string(), Value::Integer(42));
        let toml_val = Value::Table(table);
        let jinja_val = toml_value_to_jinja(&toml_val);
        // Just verify it doesn't crash - minijinja Value doesn't have direct type checking methods
        let _str_repr = jinja_val.to_string();
        assert!(!_str_repr.is_empty());
    }

    #[test]
    fn test_custom_env_or_function() {
        let env = create_template_environment();

        // Test env_or with default value
        let template = env
            .template_from_str("{{ env_or('NONEXISTENT_VAR_12345', 'default') }}")
            .unwrap();
        let result = template.render(()).unwrap();
        assert_eq!(result, "default");
    }

    #[test]
    fn test_custom_env_or_function_with_existing_var() {
        // Set a test environment variable
        std::env::set_var("TEST_VAR_12345", "test_value");

        let env = create_template_environment();
        let template = env
            .template_from_str("{{ env_or('TEST_VAR_12345', 'default') }}")
            .unwrap();
        let result = template.render(()).unwrap();
        assert_eq!(result, "test_value");

        // Clean up
        std::env::remove_var("TEST_VAR_12345");
    }

    #[test]
    fn test_custom_env_function_error() {
        let env = create_template_environment();
        let template = env
            .template_from_str("{{ env('NONEXISTENT_VAR_12345') }}")
            .unwrap();
        let result = template.render(());
        assert!(result.is_err());
    }
}
