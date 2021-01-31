use assert_matches::assert_matches;
use pretty_assertions::assert_eq;
use std::env;
use std::fs;
use std::io::{Error, Write};
use std::process::{Command, Stdio};
use wast::{
    parser::{self, ParseBuffer},
    Error as WastError, Wat,
};

const BIN: &'static str = env!("CARGO_BIN_EXE_wasmfmt");

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

fn wasmfmt_stdin(input: &str) -> Result<String, String> {
    let mut process = Command::new(BIN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    let stdin = process
        .stdin
        .as_mut()
        .expect("failed to get stdin handle from process");

    stdin
        .write_all(input.as_bytes())
        .expect("failed to write to stdin");

    stdin.flush().unwrap();

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
fn fix_mode_works() {
    let expected = include_str!("data/output/i32.wat");
    let actual = wasmfmt(&["tests/data/input/i32.wat"]).expect("failed to format i32.wat");
    assert_eq!(actual, expected);
    assert_matches!(parse(&actual), Ok(..));
}

#[test]
fn check_mode_works() {
    let source = include_str!("data/input/i32.wat");
    let formatted = include_str!("data/output/i32.wat");
    let result =
        wasmfmt(&["tests/data/input/i32.wat", "--mode", "check"]).expect("failed to check i32.wat");
    assert!(result.contains("Difference found."));
    assert!(result.contains("Source:"));
    assert!(result.contains(source));
    assert!(result.contains("Formatted:"));
    assert!(result.contains(formatted));
}

#[test]
fn writes_to_output_file() -> Result<(), Error> {
    let formatted = include_str!("data/output/i32.wat");
    let tmp = env::temp_dir();
    let out = tmp.join("out.wat");
    let result = wasmfmt(&[
        "tests/data/input/i32.wat",
        "--output",
        out.to_str().unwrap(),
    ])
    .expect("failed to format i32.wat");
    let content = fs::read_to_string(&out)?;
    assert_eq!(content, formatted);
    assert_eq!(result, String::new());
    fs::remove_file(out)?;
    Ok(())
}

#[test]
fn reads_from_stdin_if_no_file_is_provided() -> Result<(), Error> {
    let source = include_str!("data/input/i32.wat");
    let expected = include_str!("data/output/i32.wat");
    let actual = wasmfmt_stdin(source).expect("failed to format i32.wat");
    assert_eq!(actual, expected);
    assert_matches!(parse(&actual), Ok(..));
    Ok(())
}
