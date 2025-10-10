use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Get the version from Cargo.toml
fn get_cargo_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Helper function to get the path to the compiled supertoml binary
fn supertoml_bin() -> PathBuf {
    let mut path = std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get parent directory")
        .parent()
        .expect("Failed to get grandparent directory")
        .to_path_buf();

    path.push("supertoml");

    // Handle Windows .exe extension
    if cfg!(target_os = "windows") && !path.exists() {
        path.set_extension("exe");
    }

    path
}

/// Helper function to create a temporary test file
fn create_test_file(name: &str, content: &str) -> PathBuf {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(name);
    fs::write(&file_path, content).expect("Failed to write test file");
    file_path
}

#[test]
fn test_version_flag() {
    let output = Command::new(supertoml_bin())
        .arg("--version")
        .output()
        .expect("Failed to execute supertoml");

    assert!(
        output.status.success(),
        "supertoml --version should succeed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_version = get_cargo_version();
    assert!(
        stdout.contains("supertoml"),
        "Version output should contain 'supertoml'"
    );
    assert!(
        stdout.contains(&expected_version),
        "Version output should contain version number {}",
        expected_version
    );
}

#[test]
fn test_help_flag() {
    let output = Command::new(supertoml_bin())
        .arg("--help")
        .output()
        .expect("Failed to execute supertoml");

    assert!(output.status.success(), "supertoml --help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Usage:"),
        "Help output should contain usage information"
    );
}

#[test]
fn test_missing_arguments() {
    let output = Command::new(supertoml_bin())
        .output()
        .expect("Failed to execute supertoml");

    assert!(
        !output.status.success(),
        "supertoml with no arguments should fail"
    );
}

#[test]
fn test_basic_file_processing() {
    let test_content = r#"
[database]
host = "localhost"
port = 5432
"#;

    let input_file = create_test_file("cli_test_basic.toml", test_content);

    let output = Command::new(supertoml_bin())
        .arg(input_file.to_str().unwrap())
        .arg("database")
        .output()
        .expect("Failed to execute supertoml");

    assert!(
        output.status.success(),
        "Basic file processing should succeed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("host"), "Output should contain 'host'");
    assert!(
        stdout.contains("localhost"),
        "Output should contain 'localhost'"
    );

    // Cleanup
    fs::remove_file(input_file).ok();
}

#[test]
fn test_nonexistent_file() {
    let output = Command::new(supertoml_bin())
        .arg("/nonexistent/file.toml")
        .arg("table")
        .output()
        .expect("Failed to execute supertoml");

    assert!(
        !output.status.success(),
        "Processing nonexistent file should fail"
    );
}

#[test]
fn test_invalid_toml() {
    let test_content = "this is not valid toml {{{";
    let input_file = create_test_file("cli_test_invalid.toml", test_content);

    let output = Command::new(supertoml_bin())
        .arg(input_file.to_str().unwrap())
        .arg("table")
        .output()
        .expect("Failed to execute supertoml");

    assert!(
        !output.status.success(),
        "Processing invalid TOML should fail"
    );

    // Cleanup
    fs::remove_file(input_file).ok();
}

#[test]
fn test_nonexistent_table() {
    let test_content = r#"
[database]
host = "localhost"
"#;

    let input_file = create_test_file("cli_test_nonexistent_table.toml", test_content);

    let output = Command::new(supertoml_bin())
        .arg(input_file.to_str().unwrap())
        .arg("nonexistent")
        .output()
        .expect("Failed to execute supertoml");

    assert!(
        !output.status.success(),
        "Requesting nonexistent table should fail"
    );

    // Cleanup
    fs::remove_file(input_file).ok();
}

#[test]
fn test_absolute_path() {
    // Create imported file content
    let imported_content = r#"
name = "test-server"
port = 8080
"#;

    // Create main file content with import (no file path, relative to current file)
    let test_content = r#"
[server]
__import__ = "cli_test_absolute_import.toml"
"#;

    let imported_file = create_test_file("cli_test_absolute_import.toml", imported_content);
    let input_file = create_test_file("cli_test_absolute.toml", test_content);

    // Ensure cleanup happens even if test fails
    struct Cleanup(PathBuf, PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            fs::remove_file(&self.0).ok();
            fs::remove_file(&self.1).ok();
        }
    }
    let _cleanup = Cleanup(input_file.clone(), imported_file.clone());

    let absolute_path = input_file
        .canonicalize()
        .expect("Failed to get absolute path");

    let output = Command::new(supertoml_bin())
        .arg(absolute_path.to_str().unwrap())
        .arg("server")
        .output()
        .expect("Failed to execute supertoml");

    assert!(
        output.status.success(),
        "Processing with absolute path should succeed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("name"), "Output should contain 'name'");
    assert!(
        stdout.contains("test-server"),
        "Output should contain 'test-server'"
    );
}

#[test]
fn test_relative_path() {
    // Create imported file content
    let imported_content = r#"
enabled = true
timeout = 30
"#;

    // Create main file content with import (no file path, relative to current file)
    let test_content = r#"
[config]
__import__ = "cli_test_relative_import.toml"
"#;

    // Create test file in current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let imported_file = current_dir.join("cli_test_relative_import.toml");
    let input_file = current_dir.join("cli_test_relative.toml");
    fs::write(&imported_file, imported_content).expect("Failed to write imported file");
    fs::write(&input_file, test_content).expect("Failed to write test file");

    // Ensure cleanup happens even if test fails
    struct Cleanup(PathBuf, PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            fs::remove_file(&self.0).ok();
            fs::remove_file(&self.1).ok();
        }
    }
    let _cleanup = Cleanup(input_file.clone(), imported_file.clone());

    let output = Command::new(supertoml_bin())
        .arg("cli_test_relative.toml")
        .arg("config")
        .current_dir(&current_dir)
        .output()
        .expect("Failed to execute supertoml");

    assert!(
        output.status.success(),
        "Processing with relative path should succeed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("enabled"),
        "Output should contain 'enabled'"
    );
    assert!(stdout.contains("true"), "Output should contain 'true'");
}
