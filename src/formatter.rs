use crate::error::SuperTomlError;
use crate::loader::TomlTable;
use std::collections::HashMap;

fn sorted_keys(values: &HashMap<String, toml::Value>) -> Vec<&String> {
    let mut keys: Vec<&String> = values.keys().collect();
    keys.sort();
    keys
}

pub fn format_as_toml(values: &HashMap<String, toml::Value>) -> Result<String, SuperTomlError> {
    let mut table = TomlTable::new();
    for key in sorted_keys(values) {
        table.insert(key.clone(), values[key].clone());
    }

    let value = toml::Value::Table(table);
    toml::to_string(&value).map_err(|e| SuperTomlError::SerializationError(e.to_string()))
}

pub fn format_as_json(values: &HashMap<String, toml::Value>) -> Result<String, SuperTomlError> {
    let json_value = resolved_values_to_json_value(values);
    serde_json::to_string_pretty(&json_value)
        .map_err(|e| SuperTomlError::SerializationError(e.to_string()))
}

pub fn format_as_dotenv(values: &HashMap<String, toml::Value>) -> Result<String, SuperTomlError> {
    let lines: Vec<String> = sorted_keys(values)
        .into_iter()
        .map(|key| format!("{}={}", key, value_to_string(&values[key])))
        .collect();
    Ok(lines.join("\n"))
}

pub fn format_as_exports(values: &HashMap<String, toml::Value>) -> Result<String, SuperTomlError> {
    let lines: Vec<String> = sorted_keys(values)
        .into_iter()
        .map(|key| {
            format!(
                "export \"{}={}\"",
                key,
                value_to_exports_string(&values[key])
            )
        })
        .collect();
    Ok(lines.join("\n"))
}

pub fn format_as_tfvars(values: &HashMap<String, toml::Value>) -> Result<String, SuperTomlError> {
    let lines: Vec<String> = sorted_keys(values)
        .into_iter()
        .map(|key| format!("{} = {}", key, value_to_tfvars_string(&values[key])))
        .collect();
    Ok(lines.join("\n"))
}

fn resolved_values_to_json_value(values: &HashMap<String, toml::Value>) -> serde_json::Value {
    let mut json_map = serde_json::Map::new();
    for key in sorted_keys(values) {
        json_map.insert(key.clone(), toml_value_to_json(&values[key]));
    }
    serde_json::Value::Object(json_map)
}

fn toml_value_to_json(value: &toml::Value) -> serde_json::Value {
    match value {
        toml::Value::String(s) => serde_json::Value::String(s.clone()),
        toml::Value::Integer(i) => serde_json::Value::Number((*i).into()),
        toml::Value::Float(f) => serde_json::Value::Number(
            serde_json::Number::from_f64(*f).unwrap_or_else(|| serde_json::Number::from(0)),
        ),
        toml::Value::Boolean(b) => serde_json::Value::Bool(*b),
        toml::Value::Array(arr) => {
            let json_arr: Vec<serde_json::Value> = arr.iter().map(toml_value_to_json).collect();
            serde_json::Value::Array(json_arr)
        }
        toml::Value::Table(table) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in table {
                json_map.insert(k.clone(), toml_value_to_json(v));
            }
            serde_json::Value::Object(json_map)
        }
        toml::Value::Datetime(dt) => serde_json::Value::String(dt.to_string()),
    }
}

fn value_to_string(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Datetime(dt) => dt.to_string(),
        toml::Value::Array(_) | toml::Value::Table(_) => {
            serde_json::to_string(&toml_value_to_json(value)).unwrap_or_else(|_| "null".to_string())
        }
    }
}

fn value_to_exports_string(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Datetime(dt) => dt.to_string(),
        toml::Value::Array(_) | toml::Value::Table(_) => {
            // For exports, we need to escape double quotes in JSON for shell safety
            let json_str = serde_json::to_string(&toml_value_to_json(value))
                .unwrap_or_else(|_| "null".to_string());
            json_str.replace('"', "\\\"")
        }
    }
}

fn value_to_tfvars_string(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => format!("\"{}\"", s),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Datetime(dt) => format!("\"{}\"", dt),
        toml::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_tfvars_string).collect();
            format!("[{}]", items.join(", "))
        }
        toml::Value::Table(table) => {
            let mut pairs: Vec<String> = Vec::new();
            let mut keys: Vec<&String> = table.keys().collect();
            keys.sort();
            for key in keys {
                pairs.push(format!("{} = {}", key, value_to_tfvars_string(&table[key])));
            }
            format!("{{{}}}", pairs.join(", "))
        }
    }
}
