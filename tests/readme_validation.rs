use std::fs;
use supertoml::{
    format_as_dotenv, format_as_exports, format_as_json, format_as_tfvars, format_as_toml, Plugin,
    Resolver,
};
use tempfile::NamedTempFile;

#[derive(Debug, Clone)]
struct OutputFormat {
    name: &'static str,
    format_fn: fn(
        &std::collections::HashMap<String, toml::Value>,
    ) -> Result<String, supertoml::SuperTomlError>,
    start_marker: &'static str,
    end_marker: &'static str,
    assert_fn: fn(&str, &str, &str),
}

fn get_output_formats() -> Vec<OutputFormat> {
    vec![
        OutputFormat {
            name: "toml",
            format_fn: format_as_toml,
            start_marker: "Extract the fully processed production configuration:\n\n```bash\nsupertoml app.toml prod --output toml\n```\n\n**Output:**\n```toml\n",
            end_marker: "\n```\n\nFor JSON output:",
            assert_fn: assert_string_equivalent,
        },
        OutputFormat {
            name: "json",
            format_fn: format_as_json,
            start_marker: "For JSON output:\n\n```bash\nsupertoml app.toml prod --output json\n```\n\n**Output:**\n```json\n",
            end_marker: "\n```\n\nFor environment variables (dotenv):",
            assert_fn: assert_json_equivalent,
        },
        OutputFormat {
            name: "dotenv",
            format_fn: format_as_dotenv,
            start_marker: "For environment variables (dotenv):\n\n```bash\nsupertoml app.toml prod --output dotenv\n```\n\n**Output:**\n```\n",
            end_marker: "\n```\n\nFor shell exports:",
            assert_fn: assert_dotenv_equivalent,
        },
        OutputFormat {
            name: "exports",
            format_fn: format_as_exports,
            start_marker: "For shell exports:\n\n```bash\nsupertoml app.toml prod --output exports\n```\n\n**Output:**\n```bash\n",
            end_marker: "\n```\n\nFor Terraform variables:",
            assert_fn: assert_exports_equivalent,
        },
        OutputFormat {
            name: "tfvars",
            format_fn: format_as_tfvars,
            start_marker: "For Terraform variables:\n\n```bash\nsupertoml app.toml prod --output tfvars\n```\n\n**Output:**\n```hcl\n",
            end_marker: "\n```\n\n### Variable Resolution Order",
            assert_fn: assert_tfvars_equivalent,
        },
    ]
}

fn get_resolved_values_for_testing() -> std::collections::HashMap<String, toml::Value> {
    // Extract the TOML example from README
    let readme_content = fs::read_to_string("README.md").expect("Failed to read README.md");

    let toml_example =
        extract_toml_example(&readme_content).expect("Failed to extract TOML example from README");

    // Create a temporary file with the TOML content
    let temp_file = NamedTempFile::new().expect("Failed to create temporary file");

    fs::write(&temp_file, toml_example).expect("Failed to write TOML to temporary file");

    // Test the example using the standard plugins (same as CLI)
    let mut resolver = Resolver::new(vec![
        &supertoml::plugins::BeforePlugin as &dyn Plugin,
        &supertoml::plugins::TemplatingPlugin as &dyn Plugin,
        &supertoml::plugins::AfterPlugin as &dyn Plugin,
    ]);

    resolver
        .resolve_table(temp_file.path().to_str().unwrap(), "prod")
        .expect("Failed to resolve prod table from README example")
}

/// Test TOML output format
#[test]
fn test_readme_toml_output() {
    test_output_format("toml");
}

/// Test JSON output format
#[test]
fn test_readme_json_output() {
    test_output_format("json");
}

/// Test dotenv output format
#[test]
fn test_readme_dotenv_output() {
    test_output_format("dotenv");
}

/// Test exports output format
#[test]
fn test_readme_exports_output() {
    test_output_format("exports");
}

/// Test tfvars output format
#[test]
fn test_readme_tfvars_output() {
    test_output_format("tfvars");
}

fn test_output_format(format_name: &str) {
    let output_formats = get_output_formats();
    let format = output_formats
        .iter()
        .find(|f| f.name == format_name)
        .unwrap_or_else(|| panic!("Unknown format: {}", format_name));

    let readme_content = fs::read_to_string("README.md").expect("Failed to read README.md");

    let expected_output =
        extract_output_from_readme(&readme_content, format.start_marker, format.end_marker)
            .unwrap_or_else(|| panic!("Failed to extract expected {} from README", format.name));

    let resolved_values = get_resolved_values_for_testing();

    let actual_output = (format.format_fn)(&resolved_values)
        .unwrap_or_else(|e| panic!("Failed to format as {}: {}", format.name, e));

    (format.assert_fn)(
        &actual_output,
        &expected_output,
        &format!("{} output doesn't match README example", format.name),
    );
}

fn extract_output_from_readme(
    readme_content: &str,
    start_marker: &str,
    end_marker: &str,
) -> Option<String> {
    let start_pos = readme_content.find(start_marker)?;
    let search_from = &readme_content[start_pos + start_marker.len()..];
    let end_pos = search_from.find(end_marker)?;

    Some(search_from[..end_pos].trim().to_string())
}

fn extract_toml_example(readme_content: &str) -> Option<String> {
    // Find the advanced features example TOML block
    let start_marker = "SuperTOML's power comes from its built-in plugin system that enables template processing and dependency resolution. Here's a comprehensive example:\n\n```toml\n";
    let end_marker = "```\n\nExtract the fully processed production configuration:";

    let start_pos = readme_content.find(start_marker)?;
    let content_start = start_pos + start_marker.len();

    let search_from = &readme_content[content_start..];
    let end_pos = search_from.find(end_marker)?;

    Some(search_from[..end_pos].to_string())
}

fn assert_string_equivalent(actual: &str, expected: &str, message: &str) {
    assert_eq!(actual.trim(), expected.trim(), "{}", message);
}

fn assert_json_equivalent(actual: &str, expected: &str, message: &str) {
    // Parse both JSON strings and compare the parsed values
    let actual_value: serde_json::Value =
        serde_json::from_str(actual.trim()).expect("Failed to parse actual JSON output");

    let expected_value: serde_json::Value =
        serde_json::from_str(expected.trim()).expect("Failed to parse expected JSON from README");

    assert_eq!(
        actual_value, expected_value,
        "{}\nActual JSON:\n{}\nExpected JSON:\n{}",
        message, actual, expected
    );
}

fn assert_tfvars_equivalent(actual: &str, expected: &str, message: &str) {
    // For tfvars, we'll compare line by line after sorting since order might vary
    let mut actual_lines: Vec<&str> = actual.trim().lines().collect();
    let mut expected_lines: Vec<&str> = expected.trim().lines().collect();

    actual_lines.sort();
    expected_lines.sort();

    assert_eq!(
        actual_lines, expected_lines,
        "{}\nActual tfvars:\n{}\nExpected tfvars:\n{}",
        message, actual, expected
    );
}

fn assert_dotenv_equivalent(actual: &str, expected: &str, message: &str) {
    // For dotenv, we'll compare line by line after sorting since order might vary
    let mut actual_lines: Vec<&str> = actual.trim().lines().collect();
    let mut expected_lines: Vec<&str> = expected.trim().lines().collect();

    actual_lines.sort();
    expected_lines.sort();

    assert_eq!(
        actual_lines, expected_lines,
        "{}\nActual dotenv:\n{}\nExpected dotenv:\n{}",
        message, actual, expected
    );
}

fn assert_exports_equivalent(actual: &str, expected: &str, message: &str) {
    // For exports, we'll compare line by line after sorting since order might vary
    let mut actual_lines: Vec<&str> = actual.trim().lines().collect();
    let mut expected_lines: Vec<&str> = expected.trim().lines().collect();

    actual_lines.sort();
    expected_lines.sort();

    assert_eq!(
        actual_lines, expected_lines,
        "{}\nActual exports:\n{}\nExpected exports:\n{}",
        message, actual, expected
    );
}
