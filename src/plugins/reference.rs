
use serde::Deserialize;
use crate::{Plugin, SuperTomlError, extract_config};

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
        config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        let config: ReferenceConfig = extract_config!(config, ReferenceConfig, self.name())?;
        
        let current_values = std::mem::take(&mut resolver.values);
        crate::resolve_table_recursive(resolver, &config.table)?;
        let referenced_values = std::mem::take(&mut resolver.values);
        resolver.values = current_values;
        
        let prefix = config.prefix.unwrap_or_default();
        for (key, value) in &referenced_values {
            let new_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}{}", prefix, key)
            };
            resolver.values.insert(new_key, value.clone());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;
    
    #[test]
    fn test_reference_plugin() {
        let plugin = ReferencePlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        
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
        
        let result = plugin.process(&mut resolver, config);
        assert!(result.is_ok());
        
        assert_eq!(resolver.values.len(), 2);
        assert_eq!(resolver.values.get("ref_key1").unwrap().as_str().unwrap(), "value1");
        assert_eq!(resolver.values.get("ref_key2").unwrap().as_integer().unwrap(), 42);
    }
}
