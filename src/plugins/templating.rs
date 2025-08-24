use crate::{Plugin, SuperTomlError};
use minijinja::{Environment, Value as JinjaValue};
use std::collections::HashMap;

pub struct TemplatingPlugin;

fn toml_value_to_jinja(value: &toml::Value) -> JinjaValue {
    match value {
        toml::Value::String(s) => JinjaValue::from(s.as_str()),
        toml::Value::Integer(i) => JinjaValue::from(*i),
        toml::Value::Float(f) => JinjaValue::from(*f),
        toml::Value::Boolean(b) => JinjaValue::from(*b),
        toml::Value::Array(arr) => {
            let jinja_arr: Vec<JinjaValue> = arr.iter().map(toml_value_to_jinja).collect();
            JinjaValue::from(jinja_arr)
        }
        toml::Value::Table(table) => {
            let mut jinja_map = HashMap::new();
            for (k, v) in table {
                jinja_map.insert(k.clone(), toml_value_to_jinja(v));
            }
            JinjaValue::from(jinja_map)
        }
        toml::Value::Datetime(dt) => JinjaValue::from(dt.to_string()),
    }
}

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
            if s.contains("{{") || s.contains("{%") {
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
