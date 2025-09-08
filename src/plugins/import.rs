use crate::{
    extract_config,
    utils::{
        add_values_to_resolver, create_template_environment, template_error, toml_value_to_jinja,
    },
    Plugin, SuperTomlError,
};
use minijinja::Value as JinjaValue;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, serde::Serialize)]
pub struct ImportConfig {
    pub file: String,
    pub table: String,
    pub key_format: Option<String>,
}

pub struct ImportPlugin;

impl Plugin for ImportPlugin {
    fn name(&self) -> &str {
        "import"
    }

    fn process(
        &self,
        resolver: &mut crate::Resolver,
        table_values: &mut HashMap<String, toml::Value>,
        config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        // Skip processing if config is not an array (no import configurations)
        if config.as_array().is_none() {
            // No import configuration, add table_values to resolver and return
            add_values_to_resolver(resolver, table_values);
            return Ok(());
        }

        let import_configs: Vec<ImportConfig> =
            extract_config!(config, Vec<ImportConfig>, self.name())?;

        for import_config in import_configs {
            self.process_single_import(&import_config, table_values, resolver)?;
        }

        // Add all table_values to resolver.values (following the pattern from other plugins)
        add_values_to_resolver(resolver, table_values);

        Ok(())
    }
}

impl ImportPlugin {
    /// Process a single import configuration
    fn process_single_import(
        &self,
        import_config: &ImportConfig,
        table_values: &mut HashMap<String, toml::Value>,
        resolver: &crate::Resolver,
    ) -> Result<(), SuperTomlError> {
        // Load the external TOML file
        let external_toml = crate::loader::load_toml_file(&import_config.file)?;

        // Extract the specified table using idiomatic Rust
        let table_data = self.extract_table_from_toml(&external_toml, import_config)?;

        // Process each key/value pair
        for (key, value) in table_data {
            let final_key = if let Some(ref key_format) = import_config.key_format {
                // Transform the key using minijinja
                self.transform_key_with_template(key, key_format, &resolver.values)?
            } else {
                key.clone()
            };

            // Add the key/value pair to table_values
            table_values.insert(final_key, value.clone());
        }

        Ok(())
    }

    /// Extract a table from TOML with clear error messages
    fn extract_table_from_toml<'a>(
        &self,
        toml: &'a toml::Value,
        config: &ImportConfig,
    ) -> Result<&'a toml::map::Map<String, toml::Value>, SuperTomlError> {
        toml.as_table()
            .ok_or_else(|| {
                SuperTomlError::InvalidTableType(format!(
                    "Root element in file '{}' is not a table",
                    config.file
                ))
            })?
            .get(&config.table)
            .ok_or_else(|| {
                SuperTomlError::TableNotFound(format!(
                    "Table '{}' not found in file '{}'",
                    config.table, config.file
                ))
            })?
            .as_table()
            .ok_or_else(|| {
                SuperTomlError::TableNotFound(format!(
                    "Table '{}' in file '{}' is not a table",
                    config.table, config.file
                ))
            })
    }

    /// Transform a key using a minijinja template
    fn transform_key_with_template(
        &self,
        key: &str,
        template: &str,
        context: &HashMap<String, toml::Value>,
    ) -> Result<String, SuperTomlError> {
        let env = create_template_environment();

        // Create template context with the key variable
        let mut template_context = HashMap::new();
        template_context.insert("key".to_string(), JinjaValue::from(key));

        // Add resolver context
        for (k, v) in context {
            template_context.insert(k.clone(), toml_value_to_jinja(v));
        }

        let template_obj = env
            .template_from_str(template)
            .map_err(|e| template_error(self.name(), "Failed to parse key_format template", e))?;

        let result = template_obj
            .render(&template_context)
            .map_err(|e| template_error(self.name(), "Failed to render key_format template", e))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::NamedTempFile;
    use toml::Value;

    /// Helper function to create a test setup
    fn create_test_setup() -> (ImportPlugin, crate::Resolver, HashMap<String, toml::Value>) {
        let plugin = ImportPlugin;
        let resolver = crate::Resolver::new(vec![]);
        let table_values = HashMap::new();
        (plugin, resolver, table_values)
    }

    /// Helper function to create a temporary TOML file
    fn create_temp_toml_file(content: &str) -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), content).unwrap();
        temp_file
    }

    #[test]
    fn test_import_plugin_basic() {
        let (plugin, mut resolver, mut table_values) = create_test_setup();
        table_values.insert(
            "existing_key".to_string(),
            Value::String("existing_value".to_string()),
        );

        // Create a temporary TOML file
        let temp_file = create_temp_toml_file(
            r#"
[database]
host = "localhost"
port = 5432
user = "test_user"

[cache]
ttl = 300
size = 1000
"#,
        );

        let file_path = temp_file.path().to_str().unwrap().to_string();
        let config_vec = vec![ImportConfig {
            file: file_path,
            table: "database".to_string(),
            key_format: None,
        }];
        let config = Value::try_from(config_vec).unwrap();

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_ok());

        // Check that values were imported
        assert_eq!(
            table_values.get("host").unwrap().as_str().unwrap(),
            "localhost"
        );
        assert_eq!(
            table_values.get("port").unwrap().as_integer().unwrap(),
            5432
        );
        assert_eq!(
            table_values.get("user").unwrap().as_str().unwrap(),
            "test_user"
        );

        // Check that existing values are preserved
        assert_eq!(
            table_values.get("existing_key").unwrap().as_str().unwrap(),
            "existing_value"
        );

        // Check that values were added to resolver.values
        assert_eq!(
            resolver.values.get("host").unwrap().as_str().unwrap(),
            "localhost"
        );
        assert_eq!(
            resolver
                .values
                .get("existing_key")
                .unwrap()
                .as_str()
                .unwrap(),
            "existing_value"
        );
    }

    #[test]
    fn test_import_plugin_with_key_format() {
        let plugin = ImportPlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        let mut table_values = HashMap::new();

        // Create a temporary TOML file
        let temp_file = NamedTempFile::new().unwrap();
        let external_toml_content = r#"
[config]
host = "example.com"
port = 443
"#;
        fs::write(temp_file.path(), external_toml_content).unwrap();

        let file_path = temp_file.path().to_str().unwrap().to_string();
        let config_vec = vec![ImportConfig {
            file: file_path,
            table: "config".to_string(),
            key_format: Some("api_{{key}}".to_string()),
        }];
        let config = Value::try_from(config_vec).unwrap();

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_ok());

        // Check that keys were transformed
        assert_eq!(
            table_values.get("api_host").unwrap().as_str().unwrap(),
            "example.com"
        );
        assert_eq!(
            table_values.get("api_port").unwrap().as_integer().unwrap(),
            443
        );

        // Original keys should not exist
        assert!(table_values.get("host").is_none());
        assert!(table_values.get("port").is_none());
    }

    #[test]
    fn test_import_plugin_multiple_imports() {
        let plugin = ImportPlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        let mut table_values = HashMap::new();

        // Create temporary TOML files
        let temp_file1 = NamedTempFile::new().unwrap();
        let external_toml_content1 = r#"
[database]
host = "db.example.com"
port = 5432
"#;
        fs::write(temp_file1.path(), external_toml_content1).unwrap();

        let temp_file2 = NamedTempFile::new().unwrap();
        let external_toml_content2 = r#"
[cache]
host = "cache.example.com"
port = 6379
"#;
        fs::write(temp_file2.path(), external_toml_content2).unwrap();

        let file_path1 = temp_file1.path().to_str().unwrap().to_string();
        let file_path2 = temp_file2.path().to_str().unwrap().to_string();
        let config_vec = vec![
            ImportConfig {
                file: file_path1,
                table: "database".to_string(),
                key_format: Some("db_{{key}}".to_string()),
            },
            ImportConfig {
                file: file_path2,
                table: "cache".to_string(),
                key_format: Some("cache_{{key}}".to_string()),
            },
        ];
        let config = Value::try_from(config_vec).unwrap();

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_ok());

        // Check that both imports worked with different prefixes
        assert_eq!(
            table_values.get("db_host").unwrap().as_str().unwrap(),
            "db.example.com"
        );
        assert_eq!(
            table_values.get("db_port").unwrap().as_integer().unwrap(),
            5432
        );
        assert_eq!(
            table_values.get("cache_host").unwrap().as_str().unwrap(),
            "cache.example.com"
        );
        assert_eq!(
            table_values
                .get("cache_port")
                .unwrap()
                .as_integer()
                .unwrap(),
            6379
        );
    }

    #[test]
    fn test_import_plugin_file_not_found() {
        let plugin = ImportPlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        let mut table_values = HashMap::new();

        let config_vec = vec![ImportConfig {
            file: "nonexistent.toml".to_string(),
            table: "test".to_string(),
            key_format: None,
        }];
        let config = Value::try_from(config_vec).unwrap();

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_plugin_table_not_found() {
        let plugin = ImportPlugin;
        let mut resolver = crate::Resolver::new(vec![]);
        let mut table_values = HashMap::new();

        // Create a temporary TOML file
        let temp_file = NamedTempFile::new().unwrap();
        let external_toml_content = r#"
[database]
host = "localhost"
"#;
        fs::write(temp_file.path(), external_toml_content).unwrap();

        let file_path = temp_file.path().to_str().unwrap().to_string();
        let config_vec = vec![ImportConfig {
            file: file_path,
            table: "nonexistent_table".to_string(),
            key_format: None,
        }];
        let config = Value::try_from(config_vec).unwrap();

        let result = plugin.process(&mut resolver, &mut table_values, config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Table 'nonexistent_table' not found"));
    }
}
