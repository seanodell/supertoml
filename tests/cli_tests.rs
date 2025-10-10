use std::path::PathBuf;
use std::process::Command;

fn run_cli_test_case(test_dir: &PathBuf) -> Result<(), String> {
    let script_path = test_dir.join("run.sh");

    if !script_path.exists() {
        return Err(format!("run.sh not found"));
    }

    let absolute_script_path = script_path
        .canonicalize()
        .map_err(|e| format!("Failed to resolve script path: {}", e))?;

    let output = Command::new("bash")
        .arg(&absolute_script_path)
        .current_dir(&test_dir)
        .output()
        .map_err(|e| format!("Failed to execute run.sh: {}", e))?;

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        return Err(format!("Script exited with non-zero status"));
    }

    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/generated_cli_tests.rs"));
