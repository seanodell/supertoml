
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
        &self,
        resolver: &mut crate::Resolver,
        config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        let config: NoopConfig = extract_config!(config, NoopConfig, self.name())?;
        
        if !config.enabled {
            return Ok(());
        }
        
        if let Some(message) = config.message {
            println!("NoopPlugin: {}", message);
        } else {
            println!("NoopPlugin: Running with {} values", resolver.values.len());
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
        let plugin = NoopPlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        resolver.values.insert("key1".to_string(), Value::String("value1".to_string()));
        
        let config = Value::try_from(toml::toml! {
            message = "Hello from noop!"
            enabled = true
        }).unwrap();
        
        let result = plugin.process(&mut resolver, config);
        assert!(result.is_ok());
        
        assert_eq!(resolver.values.len(), 1);
        assert_eq!(resolver.values.get("key1").unwrap().as_str().unwrap(), "value1");
    }
    
    #[test]
    fn test_noop_plugin_disabled() {
        let plugin = NoopPlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        resolver.values.insert("key1".to_string(), Value::String("value1".to_string()));
        
        let config = Value::try_from(toml::toml! {
            message = "This should not print"
            enabled = false
        }).unwrap();
        
        let result = plugin.process(&mut resolver, config);
        assert!(result.is_ok());
        
        assert_eq!(resolver.values.len(), 1);
    }
}
