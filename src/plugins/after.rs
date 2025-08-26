use crate::{Plugin, SuperTomlError};
use std::collections::HashMap;

pub struct AfterPlugin;

impl Plugin for AfterPlugin {
    fn name(&self) -> &str {
        "after"
    }

    fn process(
        &self,
        resolver: &mut crate::Resolver,
        _table_values: &mut HashMap<String, toml::Value>,
        config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        if let Some(table_names) = config.as_array() {
            for table_name_value in table_names {
                if let Some(table_name) = table_name_value.as_str() {
                    crate::resolve_table_recursive(resolver, table_name)?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use toml::Value;

    #[test]
    fn test_after_plugin() {
        let plugin = AfterPlugin;
        let mut resolver =
            crate::Resolver::new(vec![&crate::plugins::NoopPlugin as &dyn crate::Plugin]);
        let mut table_values = HashMap::new();
        table_values.insert(
            "main_key".to_string(),
            Value::String("main_value".to_string()),
        );

        let config = Value::try_from(toml::toml! {
            ["source"]
        })
        .unwrap();

        let mut toml_data = toml::map::Map::new();
        let mut source_table = toml::map::Map::new();
        source_table.insert("key1".to_string(), Value::String("value1".to_string()));
        source_table.insert("key2".to_string(), Value::Integer(42));

        toml_data.insert("source".to_string(), Value::Table(source_table));

        resolver.toml_file = Some(Value::Table(toml_data));

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_ok());

        assert_eq!(table_values.len(), 1);
        assert_eq!(
            table_values.get("main_key").unwrap().as_str().unwrap(),
            "main_value"
        );

        assert_eq!(resolver.values.len(), 0);

        assert!(resolver.values.get("key1").is_none());
        assert!(resolver.values.get("key2").is_none());
    }

    #[test]
    fn test_after_plugin_empty_config() {
        let plugin = AfterPlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        let mut table_values = HashMap::new();
        table_values.insert("key1".to_string(), Value::String("value1".to_string()));

        let config = Value::Array(vec![]);

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_ok());

        assert_eq!(resolver.values.len(), 0);
        assert_eq!(table_values.len(), 1);
    }
}
