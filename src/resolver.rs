use crate::error::SuperTomlError;
use crate::loader::{load_toml_file, TomlTable};
use std::collections::{HashMap, HashSet};

#[macro_export]
macro_rules! extract_config {
    ($config:expr, $config_type:ty) => {
        $config.try_into::<$config_type>().map_err(|e| {
            $crate::SuperTomlError::PluginDeserialization {
                plugin_name: "unknown".to_string(),
                error: format!("{}", e),
            }
        })
    };
    ($config:expr, $config_type:ty, $plugin_name:expr) => {
        $config.try_into::<$config_type>().map_err(|e| {
            $crate::SuperTomlError::PluginDeserialization {
                plugin_name: $plugin_name.to_string(),
                error: format!("{}", e),
            }
        })
    };
}

pub trait Plugin {
    fn name(&self) -> &str;

    fn process(
        &self,
        resolver: &mut Resolver,
        table_values: &mut HashMap<String, toml::Value>,
        config: toml::Value,
    ) -> Result<(), SuperTomlError>;
}

pub struct Resolver {
    pub plugins: Vec<&'static dyn Plugin>,
    pub values: HashMap<String, toml::Value>,
    pub processed_tables: HashSet<String>,
    pub toml_file: Option<toml::Value>,
    pub file_path: Option<String>,
}

impl Resolver {
    pub fn new(plugins: Vec<&'static dyn Plugin>) -> Self {
        Self {
            plugins,
            values: HashMap::new(),
            processed_tables: HashSet::new(),
            toml_file: None,
            file_path: None,
        }
    }

    pub fn resolve_table(
        &mut self,
        file_path: &str,
        table_name: &str,
    ) -> Result<HashMap<String, toml::Value>, SuperTomlError> {
        self.file_path = Some(file_path.to_string());
        self.toml_file = Some(load_toml_file(file_path)?);
        resolve_table_recursive(self, table_name)?;
        Ok(std::mem::take(&mut self.values))
    }
}

// Move processing logic to free functions to avoid self borrowing issues
pub fn resolve_table_recursive(resolver: &mut Resolver, table_name: &str) -> Result<(), SuperTomlError> {
    if resolver.processed_tables.contains(table_name) {
        return Err(SuperTomlError::CycleDetected(table_name.to_string()));
    }

    resolver.processed_tables.insert(table_name.to_string());
    let table = get_table_from_loaded_file(resolver, table_name)?;

    let mut table_values: HashMap<String, toml::Value> = HashMap::new();
    for (key, value) in &table {
        if key != "_" {
            table_values.insert(key.clone(), value.clone());
        }
    }

    if let Some(plugins_table) = table.get("_").and_then(|v| v.as_table()) {
        process_plugins(resolver, &mut table_values, plugins_table)?;
    }

    for (key, value) in table_values {
        resolver.values.insert(key, value);
    }

    Ok(())
}

fn process_plugins(
    resolver: &mut Resolver, 
    table_values: &mut HashMap<String, toml::Value>, 
    plugins_table: &TomlTable
) -> Result<(), SuperTomlError> {
    let plugins_to_process = resolver.plugins.clone();
    
    for plugin in plugins_to_process {
        let plugin_name = plugin.name().to_string();

        if let Some(plugin_data) = plugins_table.get(&plugin_name) {
            plugin
                .process(resolver, table_values, plugin_data.clone())
                .map_err(|e| match e {
                    SuperTomlError::PluginError { .. }
                    | SuperTomlError::PluginDeserialization { .. } => e,
                    other => SuperTomlError::PluginError {
                        plugin_name: plugin_name.to_string(),
                        error: format!("{}", other),
                    },
                })?;
        }
    }

    Ok(())
}

fn get_table_from_loaded_file(resolver: &Resolver, table_name: &str) -> Result<TomlTable, SuperTomlError> {
    let toml_file = resolver
        .toml_file
        .as_ref()
        .ok_or_else(|| SuperTomlError::TableNotFound("No TOML file loaded".to_string()))?;

    let root_table = toml_file
        .as_table()
        .ok_or_else(|| SuperTomlError::InvalidTableType("root".to_string()))?;

    let table = root_table
        .get(table_name)
        .ok_or_else(|| SuperTomlError::TableNotFound(table_name.to_string()))?;

    table
        .as_table()
        .cloned()
        .ok_or_else(|| SuperTomlError::InvalidTableType(table_name.to_string()))
}
