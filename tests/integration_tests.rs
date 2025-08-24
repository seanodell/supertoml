
use supertoml::{
    extract_table, format_as_dotenv, format_as_exports, format_as_json, format_as_toml,
    load_toml_file, SuperTomlError,
};

#[derive(Debug)]
struct TestCase {
    name: String,
    description: String,
    table: String,
    expected_toml: Option<String>,
    expected_json: Option<String>,
    expected_dotenv: Option<String>,
    expected_exports: Option<String>,
}

fn load_test_case(test_file: &str) -> Result<TestCase, SuperTomlError> {
    let toml_value = load_toml_file(test_file)?;
    let root_table = toml_value
        .as_table()
        .ok_or_else(|| SuperTomlError::InvalidTableType("root".to_string()))?;

    let test_table = root_table
        .get("test")
        .ok_or_else(|| SuperTomlError::TableNotFound("test".to_string()))?
        .as_table()
        .ok_or_else(|| SuperTomlError::InvalidTableType("test".to_string()))?;

    let name = test_table
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SuperTomlError::TableNotFound("test.name".to_string()))?
        .to_string();

    let description = test_table
        .get("description")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SuperTomlError::TableNotFound("test.description".to_string()))?
        .to_string();

    let table = test_table
        .get("table")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SuperTomlError::TableNotFound("test.table".to_string()))?
        .to_string();

    let expected = root_table.get("expected").and_then(|v| v.as_table());

    let get_expected_content = |format: &str| -> Option<String> {
        expected?
            .get(format)?
            .as_table()?
            .get("content")?
            .as_str()
            .map(|s| s.trim().to_string())
    };

    Ok(TestCase {
        name,
        description,
        table,
        expected_toml: get_expected_content("toml"),
        expected_json: get_expected_content("json"),
        expected_dotenv: get_expected_content("dotenv"),
        expected_exports: get_expected_content("exports"),
    })
}

fn run_test_file(test_file: &str) {
    let test_case = load_test_case(test_file)
        .unwrap_or_else(|e| panic!("Failed to load test case {}: {}", test_file, e));

    println!(
        "Running test: {} - {}",
        test_case.name, test_case.description
    );

    let toml_value = load_toml_file(test_file)
        .unwrap_or_else(|e| panic!("Failed to load test file {}: {}", test_file, e));

    let table = extract_table(&toml_value, &test_case.table).unwrap_or_else(|e| {
        panic!(
            "Failed to extract table '{}' from {}: {}",
            test_case.table, test_file, e
        )
    });

    if let Some(expected) = test_case.expected_toml {
        let actual = format_as_toml(&table).unwrap();
        assert_eq!(
            actual.trim(),
            expected,
            "TOML output mismatch in {}",
            test_file
        );
    }

    if let Some(expected) = test_case.expected_json {
        let actual = format_as_json(&table).unwrap();
        assert_eq!(
            actual.trim(),
            expected,
            "JSON output mismatch in {}",
            test_file
        );
    }

    if let Some(expected) = test_case.expected_dotenv {
        let actual = format_as_dotenv(&table).unwrap();
        assert_eq!(
            actual.trim(),
            expected,
            "Dotenv output mismatch in {}",
            test_file
        );
    }

    if let Some(expected) = test_case.expected_exports {
        let actual = format_as_exports(&table).unwrap();
        assert_eq!(
            actual.trim(),
            expected,
            "Exports output mismatch in {}",
            test_file
        );
    }
}

// Include the generated tests
include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
