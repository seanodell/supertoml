use std::fs;
use std::path::Path;
use crate::error::SuperTomlError;

pub type TomlTable = toml::map::Map<String, toml::Value>;

pub fn load_toml_file<P: AsRef<Path>>(path: P) -> Result<toml::Value, SuperTomlError> {
    let content = fs::read_to_string(path).map_err(SuperTomlError::FileRead)?;
    content.parse().map_err(SuperTomlError::TomlParse)
}

pub fn extract_table(toml_value: &toml::Value, table_name: &str) -> Result<TomlTable, SuperTomlError> {
    let root_table = toml_value.as_table()
        .ok_or_else(|| SuperTomlError::InvalidTableType("root".to_string()))?;
    
    let table = root_table.get(table_name)
        .ok_or_else(|| SuperTomlError::TableNotFound(table_name.to_string()))?;
    
    table.as_table()
        .cloned()
        .ok_or_else(|| SuperTomlError::InvalidTableType(table_name.to_string()))
}
