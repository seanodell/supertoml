use std::env;
use std::fs;
use std::path::{Path, MAIN_SEPARATOR};

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set");

    generate_toml_tests(&out_dir);
    generate_cli_tests(&out_dir);
}

fn generate_toml_tests(out_dir: &std::ffi::OsStr) {
    let dest_path = Path::new(&out_dir).join("generated_tests.rs");

    let test_files = glob::glob("tests/toml_test_cases/*.toml")
        .expect("Failed to read test pattern")
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let mut generated_tests = String::new();

    for test_file in test_files {
        let file_stem = test_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let test_name = format!("test_{}", file_stem.replace("-", "_"));
        let test_path = if MAIN_SEPARATOR != '/' {
            test_file.to_string_lossy().replace(MAIN_SEPARATOR, "/")
        } else {
            test_file.to_string_lossy().to_string()
        };

        generated_tests.push_str(&format!(
            r#"
#[test]
fn {}() {{
    run_test_file("{}");
}}
"#,
            test_name, test_path
        ));
    }

    fs::write(&dest_path, generated_tests).expect("Failed to write generated tests file");
    println!("cargo:rerun-if-changed=tests/toml_test_cases");
}

fn generate_cli_tests(out_dir: &std::ffi::OsStr) {
    let dest_path = Path::new(&out_dir).join("generated_cli_tests.rs");

    let test_dirs = fs::read_dir("tests/cli_test_cases")
        .map(|entries| {
            let mut dirs: Vec<_> = entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| p.is_dir())
                .collect();
            dirs.sort();
            dirs
        })
        .unwrap_or_default();

    let mut generated_tests = String::new();

    for test_dir in test_dirs {
        let dir_name = test_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let test_name = format!("cli_{}", dir_name.replace("-", "_"));
        let test_path = if MAIN_SEPARATOR != '/' {
            test_dir.to_string_lossy().replace(MAIN_SEPARATOR, "/")
        } else {
            test_dir.to_string_lossy().to_string()
        };

        generated_tests.push_str(&format!(
            r#"
#[test]
fn {}() {{
    let test_dir = std::path::PathBuf::from("{}");
    if let Err(e) = run_cli_test_case(&test_dir) {{
        panic!("Test failed: {{}}", e);
    }}
}}
"#,
            test_name, test_path
        ));
    }

    fs::write(&dest_path, generated_tests).expect("Failed to write generated CLI tests file");
    println!("cargo:rerun-if-changed=tests/cli_test_cases");
}
