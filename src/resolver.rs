use std::collections::{HashMap, HashSet};
use crate::error::SuperTomlError;
use crate::loader::{load_toml_file, TomlTable};

#[macro_export]
macro_rules! extract_config {
    ($config:expr, $config_type:ty) => {
        $config.try_into::<$config_type>()
            .map_err(|e| $crate::SuperTomlError::PluginDeserialization {
                plugin_name: "unknown".to_string(),
                error: format!("{}", e),
            })
    };
    ($config:expr, $config_type:ty, $plugin_name:expr) => {
        $config.try_into::<$config_type>()
            .map_err(|e| $crate::SuperTomlError::PluginDeserialization {
                plugin_name: $plugin_name.to_string(),
                error: format!("{}", e),
            })
    };
}

pub trait Plugin {
    fn name(&self) -> &str;
    
    fn process(
        &mut self,
        values: &mut HashMap<String, toml::Value>,
        config: toml::Value,
    ) -> Result<(), SuperTomlError>;
}

pub struct Resolver {
    plugins: Vec<Box<dyn Plugin>>,
    values: HashMap<String, toml::Value>,
    processed_tables: HashSet<String>,
    toml_file: Option<toml::Value>,
}

impl Resolver {
    pub fn new(plugins: Vec<Box<dyn Plugin>>) -> Self {
        Self {
            plugins,
            values: HashMap::new(),
            processed_tables: HashSet::new(),
            toml_file: None,
        }
    }
    
    pub fn resolve_table(&mut self, file_path: &str, table_name: &str) -> Result<HashMap<String, toml::Value>, SuperTomlError> {
        self.values.clear();
        self.processed_tables.clear();
        self.toml_file = Some(load_toml_file(file_path)?);
        self.resolve_table_recursive(table_name)?;
        Ok(std::mem::take(&mut self.values))
    }
    
    pub fn resolve_table_recursive(&mut self, table_name: &str) -> Result<(), SuperTomlError> {
        if self.processed_tables.contains(table_name) {
            return Err(SuperTomlError::CycleDetected(table_name.to_string()));
        }
        
        self.processed_tables.insert(table_name.to_string());
        let table = self.extract_table_from_file(table_name)?;
        
        for (key, value) in &table {
            if key != "_" {
                self.values.insert(key.clone(), value.clone());
            }
        }
        
        if let Some(plugins_table) = table.get("_").and_then(|v| v.as_table()) {
            for plugin in &mut self.plugins {
                if let Some(plugin_data) = plugins_table.get(plugin.name()) {
                    plugin.process(&mut self.values, plugin_data.clone())
                        .map_err(|e| match e {
                            SuperTomlError::PluginError { .. } | 
                            SuperTomlError::PluginDeserialization { .. } => e,
                            other => SuperTomlError::PluginError {
                                plugin_name: plugin.name().to_string(),
                                error: format!("{}", other),
                            }
                        })?;
                }
            }
        }
        
        Ok(())
    }
    
    fn extract_table_from_file(&self, table_name: &str) -> Result<TomlTable, SuperTomlError> {
        let toml_file = self.toml_file.as_ref()
            .ok_or_else(|| SuperTomlError::TableNotFound("No TOML file loaded".to_string()))?;
            
        let root_table = toml_file.as_table()
            .ok_or_else(|| SuperTomlError::InvalidTableType("root".to_string()))?;
        
        let table = root_table.get(table_name)
            .ok_or_else(|| SuperTomlError::TableNotFound(table_name.to_string()))?;
        
        table.as_table()
            .cloned()
            .ok_or_else(|| SuperTomlError::InvalidTableType(table_name.to_string()))
    }
}
