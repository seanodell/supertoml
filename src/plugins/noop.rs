use std::collections::HashMap;
use serde::Deserialize;
use crate::{Plugin, SuperTomlError, extract_config};

#[derive(Debug, Deserialize)]
pub struct NoopConfig {
    pub message: Option<String>,
    pub enabled: bool,
}

pub struct NoopPlugin;

impl Plugin for NoopPlugin {
    fn name(&self) -> &str {
        "noop"
    }
    
    fn process(
        &mut self,
        values: &mut HashMap<String, toml::Value>,
        config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        let config: NoopConfig = extract_config!(config, NoopConfig, self.name())?;
        
        if !config.enabled {
            return Ok(());
        }
        
        if let Some(message) = config.message {
            println!("NoopPlugin: {}", message);
        } else {
            println!("NoopPlugin: Running with {} values", values.len());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;
    
    #[test]
    fn test_noop_plugin_with_message() {
        let mut plugin = NoopPlugin;
        let mut values = HashMap::new();
        values.insert("key1".to_string(), Value::String("value1".to_string()));
        
        let config = Value::try_from(toml::toml! {
            message = "Hello from noop!"
            enabled = true
        }).unwrap();
        
        let result = plugin.process(&mut values, config);
        assert!(result.is_ok());
        
        assert_eq!(values.len(), 1);
        assert_eq!(values.get("key1").unwrap().as_str().unwrap(), "value1");
    }
    
    #[test]
    fn test_noop_plugin_disabled() {
        let mut plugin = NoopPlugin;
        let mut values = HashMap::new();
        values.insert("key1".to_string(), Value::String("value1".to_string()));
        
        let config = Value::try_from(toml::toml! {
            message = "This should not print"
            enabled = false
        }).unwrap();
        
        let result = plugin.process(&mut values, config);
        assert!(result.is_ok());
        
        assert_eq!(values.len(), 1);
    }
}
