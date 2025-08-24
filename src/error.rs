#[derive(Debug)]
pub enum SuperTomlError {
    FileRead(std::io::Error),
    TomlParse(toml::de::Error),
    TableNotFound(String),
    InvalidTableType(String),
    CycleDetected(String),
    PluginDeserialization { plugin_name: String, error: String },
    PluginError { plugin_name: String, error: String },
}

impl std::fmt::Display for SuperTomlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SuperTomlError::FileRead(e) => write!(f, "Failed to read file: {}", e),
            SuperTomlError::TomlParse(e) => write!(f, "Failed to parse TOML: {}", e),
            SuperTomlError::TableNotFound(name) => write!(f, "Table '{}' not found", name),
            SuperTomlError::InvalidTableType(name) => write!(f, "Item '{}' is not a table", name),
            SuperTomlError::CycleDetected(table) => {
                write!(f, "Cycle detected when processing table '{}'", table)
            }
            SuperTomlError::PluginDeserialization { plugin_name, error } => {
                write!(
                    f,
                    "Plugin '{}' failed to deserialize data: {}",
                    plugin_name, error
                )
            }
            SuperTomlError::PluginError { plugin_name, error } => {
                write!(f, "Plugin '{}' error: {}", plugin_name, error)
            }
        }
    }
}

impl std::error::Error for SuperTomlError {}
