use std::env;
use std::fs;
use std::path::{Path, MAIN_SEPARATOR};

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set");
    let dest_path = Path::new(&out_dir).join("generated_tests.rs");

    let test_files = glob::glob("tests/test_cases/*.toml")
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
    println!("cargo:rerun-if-changed=tests/test_cases");
}
