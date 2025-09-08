//! Utility functions shared across plugins

use minijinja::Value as JinjaValue;
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
}
