#[derive(Debug)]
pub enum SuperTomlError {
    FileRead(std::io::Error),
    TomlParse(toml::de::Error),
    TableNotFound(String),
    InvalidTableType(String),
}

impl std::fmt::Display for SuperTomlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SuperTomlError::FileRead(e) => write!(f, "Failed to read file: {}", e),
            SuperTomlError::TomlParse(e) => write!(f, "Failed to parse TOML: {}", e),
            SuperTomlError::TableNotFound(name) => write!(f, "Table '{}' not found", name),
            SuperTomlError::InvalidTableType(name) => write!(f, "Item '{}' is not a table", name),
        }
    }
}

impl std::error::Error for SuperTomlError {}
