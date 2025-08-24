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

/// Trait for types that can be extracted from TOML values
pub trait FromTomlValue: Sized {
    fn from_toml_value(value: &toml::Value) -> Option<Self>;
}

impl FromTomlValue for String {
    fn from_toml_value(value: &toml::Value) -> Option<Self> {
        value.as_str().map(|s| s.to_string())
    }
}

impl FromTomlValue for i64 {
    fn from_toml_value(value: &toml::Value) -> Option<Self> {
        value.as_integer()
    }
}

impl FromTomlValue for f64 {
    fn from_toml_value(value: &toml::Value) -> Option<Self> {
        value.as_float()
    }
}

impl FromTomlValue for bool {
    fn from_toml_value(value: &toml::Value) -> Option<Self> {
        value.as_bool()
    }
}

/// Trait to add object-oriented field extraction methods to TomlTable  
pub trait TomlTableExt {
    /// Extract a field - returns unwrapped value or error
    fn get_field<T: FromTomlValue>(&self, field_name: &str) -> Result<T, SuperTomlError>;
}

impl TomlTableExt for TomlTable {
    fn get_field<T: FromTomlValue>(&self, field_name: &str) -> Result<T, SuperTomlError> {
        self.get(field_name)
            .and_then(T::from_toml_value)
            .ok_or_else(|| SuperTomlError::TableNotFound(field_name.to_string()))
    }
}


