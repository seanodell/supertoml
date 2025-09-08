use crate::{utils::toml_value_to_jinja, Plugin, SuperTomlError};
use minijinja::{Environment, Value as JinjaValue};
use std::collections::HashMap;

pub struct TemplatingPlugin;

fn process_value_with_jinja(
    value: &toml::Value,
    context: &HashMap<String, toml::Value>,
) -> Result<toml::Value, SuperTomlError> {
    let env = Environment::new();

    let context_jinja: HashMap<String, JinjaValue> = context
        .iter()
        .map(|(k, v)| (k.clone(), toml_value_to_jinja(v)))
        .collect();

    match value {
        toml::Value::String(s) => {
            if s.contains("{{") || s.contains("{%") || s.contains("{#") {
                let template =
                    env.template_from_str(s)
                        .map_err(|e| SuperTomlError::PluginError {
                            plugin_name: "templating".to_string(),
                            error: format!("Template error: {}", e),
                        })?;

                let rendered =
                    template
                        .render(&context_jinja)
                        .map_err(|e| SuperTomlError::PluginError {
                            plugin_name: "templating".to_string(),
                            error: format!("Render error: {}", e),
                        })?;

                Ok(toml::Value::String(rendered))
            } else {
                Ok(value.clone())
            }
        }
        toml::Value::Array(arr) => {
            // Recursively process each element in the array
            let processed_arr: Result<Vec<toml::Value>, SuperTomlError> = arr
                .iter()
                .map(|item| process_value_with_jinja(item, context))
                .collect();
            Ok(toml::Value::Array(processed_arr?))
        }
        toml::Value::Table(table) => {
            // Recursively process each value in the table
            let mut processed_table = toml::Table::new();
            for (key, val) in table {
                let processed_val = process_value_with_jinja(val, context)?;
                processed_table.insert(key.clone(), processed_val);
            }
            Ok(toml::Value::Table(processed_table))
        }
        _ => Ok(value.clone()),
    }
}

impl Plugin for TemplatingPlugin {
    fn name(&self) -> &str {
        "templating"
    }

    fn process(
        &self,
        resolver: &mut crate::Resolver,
        table_values: &mut HashMap<String, toml::Value>,
        _config: toml::Value,
    ) -> Result<(), SuperTomlError> {
        let processed_values: HashMap<String, toml::Value> = table_values
            .iter()
            .map(|(key, value)| {
                let processed_value = process_value_with_jinja(value, &resolver.values)?;
                Ok((key.clone(), processed_value))
            })
            .collect::<Result<HashMap<_, _>, SuperTomlError>>()?;

        *table_values = processed_values;

        for (key, value) in table_values.iter() {
            resolver.values.insert(key.clone(), value.clone());
        }

        Ok(())
    }
}
