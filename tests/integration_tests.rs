use assert_matches::assert_matches;
use pretty_assertions::assert_eq;
use std::env;
use std::fs;
use std::io::Error;
use std::process::{Command, Stdio};
use wast::{
    parser::{self, ParseBuffer},
    Error as WastError, Wat,
};

const BIN: &str = env!("CARGO_BIN_EXE_wasmfmt");

fn wasmfmt(args: &[&str]) -> Result<String, String> {
    let process = Command::new(BIN)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    let output = process
        .wait_with_output()
        .expect("failed to get process output");

    let stdout = String::from_utf8(output.stdout).expect("stdout");
    let stderr = String::from_utf8(output.stderr).expect("stderr");

    if !stderr.is_empty() {
        Err(stderr)
    } else {
        Ok(stdout)
    }
}

fn parse(input: &str) -> Result<(), WastError> {
    let buffer = ParseBuffer::new(input).unwrap();
    parser::parse::<Wat>(&buffer).map(|_| ())
}

#[test]
fn fix_works() -> Result<(), Error> {
    let original = include_str!("data/input/i32.wat");
    let expected = include_str!("data/output/default/i32.wat");

    let temp_path = env::temp_dir()
        .join("out.wat")
        .into_os_string()
        .into_string()
        .unwrap();
    fs::write(&temp_path, original)?;
    let _ = wasmfmt(&["fix", &temp_path]).expect("failed to format i32.wat");
    let actual = fs::read_to_string(&temp_path).expect("read temp file");
    fs::remove_file(&temp_path)?;

    assert_eq!(actual, expected);
    assert_matches!(parse(&actual), Ok(..));
    Ok(())
}

#[test]
fn check_works() {
    let source = include_str!("data/input/i32.wat");
    let formatted = include_str!("data/output/default/i32.wat");
    let result = wasmfmt(&["check", "tests/data/input/i32.wat"]).expect("failed to check i32.wat");
    assert!(result.contains("Difference found."));
    assert!(result.contains("Source:"));
    assert!(result.contains(source));
    assert!(result.contains("Formatted:"));
    assert!(result.contains(formatted));
}

#[test]
fn print_works() {
    let formatted = include_str!("data/output/default/i32.wat");
    let result = wasmfmt(&["print", "tests/data/input/i32.wat"]).expect("failed to print i32.wat");
    assert_eq!(result, formatted);
}
