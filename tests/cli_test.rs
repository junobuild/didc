use std::fs::{read_to_string, remove_file};
use std::path::Path;
use std::process::Command;

fn normalize_output(output: &str) -> String {
    output
        .lines()
        .map(str::trim)
        .collect::<Vec<&str>>()
        .join("\n")
}

fn run_test_for_target(target: &str, expected_output_file: &str, output_file: Option<&str>) {
    let input_path = "tests/data/satellite.did";

    let binary_path = Path::new("target/debug/junobuild-didc");

    let mut command = Command::new(binary_path);
    command
        .arg("--input")
        .arg(input_path)
        .arg("--target")
        .arg(target);

    if let Some(output_path) = output_file {
        command.arg("--output").arg(output_path);
    }

    let output = command.output().expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 in output");

    let expected_output =
        read_to_string(expected_output_file).expect("Failed to read expected output file");

    let normalized_stdout = normalize_output(&stdout);
    let normalized_expected_output = normalize_output(&expected_output);

    if output_file.is_none() {
        assert_eq!(
            normalized_stdout, normalized_expected_output,
            "Mismatch for target: {}",
            target
        );
    }

    if let Some(output_path) = output_file {
        let file_output = read_to_string(output_path).expect("Failed to read from output file");

        let normalized_file_output = normalize_output(&file_output);

        assert_eq!(
            normalized_file_output, normalized_expected_output,
            "Mismatch in file output for target: {}",
            target
        );

        remove_file(output_path).expect("Failed to remove output file");
    }
}

#[test]
fn test_js_target() {
    run_test_for_target("js", "tests/data/satellite.js", None);
}

#[test]
fn test_ts_target() {
    run_test_for_target("ts", "tests/data/satellite.ts", None);
}

#[test]
fn test_js_target_with_output_file() {
    run_test_for_target(
        "js",
        "tests/data/satellite.js",
        Some("target/test/output.js"),
    );
}

#[test]
fn test_ts_target_with_output_file() {
    run_test_for_target(
        "ts",
        "tests/data/satellite.ts",
        Some("target/test/output.ts"),
    );
}
