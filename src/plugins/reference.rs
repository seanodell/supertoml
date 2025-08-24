use crate::{extract_config, Plugin, SuperTomlError};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ReferenceConfig {
    pub table: Option<String>,
}

pub struct ReferencePlugin;

impl Plugin for ReferencePlugin {
    fn name(&self) -> &str {
        "reference"
    }

    fn process(
        &self,
        resolver: &mut crate::Resolver,
        table_values: &mut HashMap<String, toml::Value>,
        config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        if !config.as_table().map(|t| t.is_empty()).unwrap_or(true) {
            let config: ReferenceConfig = extract_config!(config, ReferenceConfig, self.name())?;

            if let Some(table_name) = config.table {
                crate::resolve_table_recursive(resolver, &table_name)?;
            }
        }

        for (key, value) in table_values.iter() {
            resolver.values.insert(key.clone(), value.clone());
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
    fn test_reference_plugin() {
        let plugin = ReferencePlugin;
        let mut resolver =
            crate::Resolver::new(vec![&crate::plugins::NoopPlugin as &dyn crate::Plugin]);
        let mut table_values = HashMap::new();

        let config = Value::try_from(toml::toml! {
            table = "source"
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

        assert_eq!(table_values.len(), 0);
        assert_eq!(
            resolver.values.get("key1").unwrap().as_str().unwrap(),
            "value1"
        );
        assert_eq!(
            resolver.values.get("key2").unwrap().as_integer().unwrap(),
            42
        );
    }

    #[test]
    fn test_reference_plugin_empty_config() {
        let plugin = ReferencePlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        let mut table_values = HashMap::new();
        table_values.insert("key1".to_string(), Value::String("value1".to_string()));

        let config = Value::Table(toml::map::Map::new());

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_ok());

        assert_eq!(table_values.len(), 1);
    }
}
