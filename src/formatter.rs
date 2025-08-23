use crate::error::SuperTomlError;
use crate::loader::TomlTable;

pub fn format_as_toml(table: &TomlTable) -> Result<String, SuperTomlError> {
    let value = toml::Value::Table(table.clone());
    Ok(toml::to_string(&value).unwrap())
}

pub fn format_as_json(table: &TomlTable) -> Result<String, SuperTomlError> {
    let json_value = toml_to_json_value(table);
    Ok(serde_json::to_string_pretty(&json_value).unwrap())
}

pub fn format_as_dotenv(table: &TomlTable) -> Result<String, SuperTomlError> {
    let mut lines = Vec::new();
    for (key, value) in table {
        let string_value = value_to_string(value);
        lines.push(format!("{}={}", key, string_value));
    }
    Ok(lines.join("\n"))
}

pub fn format_as_exports(table: &TomlTable) -> Result<String, SuperTomlError> {
    let mut lines = Vec::new();
    for (key, value) in table {
        let string_value = value_to_string(value);
        lines.push(format!("export \"{}={}\"", key, string_value));
    }
    Ok(lines.join("\n"))
}

fn toml_to_json_value(table: &TomlTable) -> serde_json::Value {
    let mut json_map = serde_json::Map::new();
    for (key, value) in table {
        json_map.insert(key.clone(), toml_value_to_json(value));
    }
    serde_json::Value::Object(json_map)
}

fn toml_value_to_json(value: &toml::Value) -> serde_json::Value {
    match value {
        toml::Value::String(s) => serde_json::Value::String(s.clone()),
        toml::Value::Integer(i) => serde_json::Value::Number((*i).into()),
        toml::Value::Float(f) => serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap()),
        toml::Value::Boolean(b) => serde_json::Value::Bool(*b),
        toml::Value::Array(arr) => {
            let json_arr: Vec<serde_json::Value> = arr.iter().map(toml_value_to_json).collect();
            serde_json::Value::Array(json_arr)
        },
        toml::Value::Table(table) => {
            let mut json_map = serde_json::Map::new();
            for (k, v) in table {
                json_map.insert(k.clone(), toml_value_to_json(v));
            }
            serde_json::Value::Object(json_map)
        },
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
