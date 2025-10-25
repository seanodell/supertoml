use crate::error::SuperTomlError;
use crate::loader::{load_toml_file, TomlTable};
use std::collections::HashMap;

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
    pub call_stack: Vec<String>,
    pub toml_file: Option<toml::Value>,
    pub file_path: Option<String>,
    pub meta_values: HashMap<String, toml::Value>,
}

impl Resolver {
    pub fn new(plugins: Vec<&'static dyn Plugin>) -> Self {
        Self {
            plugins,
            values: HashMap::new(),
            call_stack: Vec::new(),
            toml_file: None,
            file_path: None,
            meta_values: HashMap::new(),
        }
    }

    pub fn resolve_table(
        &mut self,
        file_path: &str,
        table_name: &str,
    ) -> Result<HashMap<String, toml::Value>, SuperTomlError> {
        self.file_path = Some(file_path.to_string());
        self.toml_file = Some(load_toml_file(file_path)?);

        // Populate meta values with processing context as nested TOML structure
        let mut args_map = toml::map::Map::new();
        args_map.insert(
            "file_path".to_string(),
            toml::Value::String(file_path.to_string()),
        );
        args_map.insert(
            "table_name".to_string(),
            toml::Value::String(table_name.to_string()),
        );

        let mut underscore_map = toml::map::Map::new();
        underscore_map.insert("args".to_string(), toml::Value::Table(args_map));

        self.meta_values
            .insert("_".to_string(), toml::Value::Table(underscore_map));

        resolve_table_recursive(self, table_name)?;
        Ok(std::mem::take(&mut self.values))
    }

    pub fn resolve_table_with_meta(
        &mut self,
        file_path: &str,
        table_name: &str,
        output_format: &str,
    ) -> Result<HashMap<String, toml::Value>, SuperTomlError> {
        self.file_path = Some(file_path.to_string());
        self.toml_file = Some(load_toml_file(file_path)?);

        // Populate meta values with processing context as nested TOML structure
        let mut args_map = toml::map::Map::new();
        args_map.insert(
            "file_path".to_string(),
            toml::Value::String(file_path.to_string()),
        );
        args_map.insert(
            "table_name".to_string(),
            toml::Value::String(table_name.to_string()),
        );
        args_map.insert(
            "output_format".to_string(),
            toml::Value::String(output_format.to_string()),
        );

        let mut underscore_map = toml::map::Map::new();
        underscore_map.insert("args".to_string(), toml::Value::Table(args_map));

        self.meta_values
            .insert("_".to_string(), toml::Value::Table(underscore_map));

        resolve_table_recursive(self, table_name)?;
        Ok(std::mem::take(&mut self.values))
    }
}

pub fn resolve_table_recursive(
    resolver: &mut Resolver,
    table_name: &str,
) -> Result<(), SuperTomlError> {
    // Check if we're currently processing this table (cycle detection)
    if resolver.call_stack.contains(&table_name.to_string()) {
        return Err(SuperTomlError::CycleDetected(table_name.to_string()));
    }

    // Add to call stack for cycle detection
    resolver.call_stack.push(table_name.to_string());

    let table = get_table_from_loaded_file(resolver, table_name)?;

    let mut table_values: HashMap<String, toml::Value> = HashMap::new();
    for (key, value) in &table {
        if key != "_" {
            table_values.insert(key.clone(), value.clone());
        }
    }

    let plugins_table = table.get("_").and_then(|v| v.as_table());
    process_plugins(resolver, &mut table_values, plugins_table)?;

    // Remove from call stack
    resolver.call_stack.pop();

    Ok(())
}

fn process_plugins(
    resolver: &mut Resolver,
    table_values: &mut HashMap<String, toml::Value>,
    plugins_table: Option<&TomlTable>,
) -> Result<(), SuperTomlError> {
    let plugins_to_process = resolver.plugins.clone();

    for plugin in plugins_to_process {
        let plugin_name = plugin.name().to_string();

        let config = if let Some(plugins_table) = plugins_table {
            plugins_table
                .get(&plugin_name)
                .cloned()
                .unwrap_or(toml::Value::Table(TomlTable::new()))
        } else {
            toml::Value::Table(TomlTable::new())
        };

        plugin
            .process(resolver, table_values, config)
            .map_err(|e| match e {
                SuperTomlError::PluginError { .. }
                | SuperTomlError::PluginDeserialization { .. } => e,
                other => SuperTomlError::PluginError {
                    plugin_name: plugin_name.to_string(),
                    error: format!("{}", other),
                },
            })?;
    }

    Ok(())
}

fn get_table_from_loaded_file(
    resolver: &Resolver,
    table_name: &str,
) -> Result<TomlTable, SuperTomlError> {
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
