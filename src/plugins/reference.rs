
use serde::Deserialize;
use crate::{Plugin, SuperTomlError, extract_config};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ReferenceConfig {
    pub table: String,
    pub prefix: Option<String>,
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
        let config: ReferenceConfig = extract_config!(config, ReferenceConfig, self.name())?;
        
        let current_table_values = std::mem::take(table_values);
        crate::resolve_table_recursive(resolver, &config.table)?;
        let referenced_values = std::mem::take(&mut resolver.values);
        *table_values = current_table_values;
        
        let prefix = config.prefix.unwrap_or_default();
        for (key, value) in &referenced_values {
            let new_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}{}", prefix, key)
            };
            table_values.insert(new_key, value.clone());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;
    use std::collections::HashMap;
    
    #[test]
    fn test_reference_plugin() {
        let plugin = ReferencePlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        let mut table_values = HashMap::new();
        
        let config = Value::try_from(toml::toml! {
            table = "source"
            prefix = "ref_"
        }).unwrap();
        
        let mut toml_data = toml::map::Map::new();
        let mut source_table = toml::map::Map::new();
        source_table.insert("key1".to_string(), Value::String("value1".to_string()));
        source_table.insert("key2".to_string(), Value::Integer(42));
        toml_data.insert("source".to_string(), Value::Table(source_table));
        
        resolver.toml_file = Some(Value::Table(toml_data));
        
        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_ok());
        
        assert_eq!(table_values.len(), 2);
        assert_eq!(table_values.get("ref_key1").unwrap().as_str().unwrap(), "value1");
        assert_eq!(table_values.get("ref_key2").unwrap().as_integer().unwrap(), 42);
    }
}
