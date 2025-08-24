use crate::{Plugin, SuperTomlError};
use std::collections::HashMap;

pub struct NoopPlugin;

impl Plugin for NoopPlugin {
    fn name(&self) -> &str {
        "noop"
    }

    fn process(
        &self,
        resolver: &mut crate::Resolver,
        table_values: &mut HashMap<String, toml::Value>,
        _config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        println!("NoopPlugin: Running with {} values", table_values.len());

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
    fn test_noop_plugin() {
        let plugin = NoopPlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        let mut table_values = HashMap::new();
        table_values.insert("key1".to_string(), Value::String("value1".to_string()));

        let config = Value::Table(toml::map::Map::new());

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_ok());

        assert_eq!(table_values.len(), 1);
        assert_eq!(
            table_values.get("key1").unwrap().as_str().unwrap(),
            "value1"
        );
        assert_eq!(
            resolver.values.get("key1").unwrap().as_str().unwrap(),
            "value1"
        );
    }
}
