use supertoml::{
    extract_table, format_as_dotenv, format_as_exports, format_as_json, format_as_toml,
    load_toml_file, SuperTomlError, TomlTableExt, Resolver, Plugin,
};
use supertoml::plugins::NoopPlugin;

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
    let test_table = extract_table(&toml_value, "test")?;

    let name: String = test_table.get_field("name")?;
    let description: String = test_table.get_field("description")?;
    let table: String = test_table.get_field("table")?;

    let get_expected_content = |format: &str| -> Option<String> {
        let expected_table = extract_table(&toml_value, "expected").ok()?;
        let format_table = extract_table(&toml::Value::Table(expected_table), format).ok()?;
        format_table
            .get_field::<String>("content")
            .ok()
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

    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(NoopPlugin)];
    let mut resolver = Resolver::new(plugins);
    let resolved_values = resolver.resolve_table(test_file, &test_case.table).unwrap_or_else(|e| {
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

// Include the generated tests
include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
