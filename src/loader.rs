use crate::error::SuperTomlError;
use std::fs;
use std::path::Path;

pub type TomlTable = toml::map::Map<String, toml::Value>;

pub fn load_toml_file<P: AsRef<Path>>(path: P) -> Result<toml::Value, SuperTomlError> {
    let content = fs::read_to_string(path).map_err(SuperTomlError::FileRead)?;
    content.parse().map_err(SuperTomlError::TomlParse)
}
