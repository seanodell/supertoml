use supertoml::{
    format_as_dotenv, format_as_exports, format_as_json, format_as_toml,
    SuperTomlError, Resolver, Plugin,
};
use supertoml::loader::load_toml_file;

#[derive(Debug)]
struct TestCase {
    name: String,
    description: String,
    table: String,
    expected_toml: Option<String>,
    expected_json: Option<String>,
    expected_dotenv: Option<String>,
    expected_exports: Option<String>,
    expected_error: Option<String>,
}

fn load_test_case(test_file: &str) -> Result<TestCase, SuperTomlError> {
    let toml_value = load_toml_file(test_file)?;
    let root_table = toml_value.as_table()
        .ok_or_else(|| SuperTomlError::InvalidTableType("root".to_string()))?;
    
    let test_table = root_table.get("test")
        .ok_or_else(|| SuperTomlError::TableNotFound("test".to_string()))?
        .as_table()
        .ok_or_else(|| SuperTomlError::InvalidTableType("test".to_string()))?;

    let name = test_table.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SuperTomlError::TableNotFound("name".to_string()))?
        .to_string();
    let description = test_table.get("description")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SuperTomlError::TableNotFound("description".to_string()))?
        .to_string();
    let table = test_table.get("table")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SuperTomlError::TableNotFound("table".to_string()))?
        .to_string();

    let get_expected_content = |format: &str| -> Option<String> {
        let expected_table = root_table.get("expected")?.as_table()?;
        let format_table = expected_table.get(format)?.as_table()?;
        format_table.get("content")?.as_str()?.trim().to_string().into()
    };

    let expected_error = test_table.get("expected_error")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    Ok(TestCase {
        name,
        description,
        table,
        expected_toml: get_expected_content("toml"),
        expected_json: get_expected_content("json"),
        expected_dotenv: get_expected_content("dotenv"),
        expected_exports: get_expected_content("exports"),
        expected_error,
    })
}

fn run_test_file(test_file: &str) {
    let test_case = load_test_case(test_file)
        .unwrap_or_else(|e| panic!("Failed to load test case {}: {}", test_file, e));

    println!(
        "Running test: {} - {}",
        test_case.name, test_case.description
    );

    let mut resolver = Resolver::new(vec![
        &NoopPlugin as &dyn Plugin,
        &ReferencePlugin as &dyn Plugin,
    ]);
    
    let result = resolver.resolve_table(test_file, &test_case.table);

    if let Some(expected_error) = &test_case.expected_error {
        match result {
            Ok(_) => panic!("Expected error matching '{}' but got success", expected_error),
            Err(e) => {
                let error_str = e.to_string();
                if !regex::Regex::new(expected_error).unwrap().is_match(&error_str) {
                    panic!(
                        "Error '{}' does not match expected pattern '{}'",
                        error_str, expected_error
                    );
                }
            }
        }
    } else {
        let resolved_values = result.unwrap_or_else(|e| {
            panic!(
                "Failed to resolve table '{}' from {}: {}",
                test_case.table, test_file, e
            )
        });

        if let Some(expected) = test_case.expected_toml {
            let actual = format_as_toml(&resolved_values).unwrap();
            assert_eq!(
                actual.trim(),
                expected,
                "TOML output mismatch in {}",
                test_file
            );
        }

        if let Some(expected) = test_case.expected_json {
            let actual = format_as_json(&resolved_values).unwrap();
            assert_eq!(
                actual.trim(),
                expected,
                "JSON output mismatch in {}",
                test_file
            );
        }

        if let Some(expected) = test_case.expected_dotenv {
            let actual = format_as_dotenv(&resolved_values).unwrap();
            assert_eq!(
                actual.trim(),
                expected,
                "Dotenv output mismatch in {}",
                test_file
            );
        }

        if let Some(expected) = test_case.expected_exports {
            let actual = format_as_exports(&resolved_values).unwrap();
            assert_eq!(
                actual.trim(),
                expected,
                "Exports output mismatch in {}",
                test_file
            );
        }
    }
}

// Include the generated tests
include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
