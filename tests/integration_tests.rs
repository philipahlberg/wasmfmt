use std::env;
use std::fs;
use std::io::{Error, Write};
use std::process::{Command, Stdio};

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

#[test]
fn fix_add_desugar() {
    let expected = include_str!("data/output/add_desugar.wat");
    let actual =
        wasmfmt(&["tests/data/input/add_desugar.wat"]).expect("failed to format add_desugar.wat");
    assert_eq!(actual, expected);
}

#[test]
fn fix_add_sugar() {
    let expected = include_str!("data/output/add_sugar.wat");
    let actual =
        wasmfmt(&["tests/data/input/add_desugar.wat"]).expect("failed to format add_sugar.wat");
    assert_eq!(actual, expected);
}

#[test]
fn fix_fac_desugar() {
    let expected = include_str!("data/output/fac_desugar.wat");
    let actual =
        wasmfmt(&["tests/data/input/fac_desugar.wat"]).expect("failed to format fac_desugar.wat");
    assert_eq!(actual, expected);
}

#[test]
fn fix_fac_sugar() {
    let expected = include_str!("data/output/fac_sugar.wat");
    let actual =
        wasmfmt(&["tests/data/input/fac_sugar.wat"]).expect("failed to format fac_sugar.wat");
    assert_eq!(actual, expected);
}

#[test]
fn fix_global() {
    let expected = include_str!("data/output/global.wat");
    let actual = wasmfmt(&["tests/data/input/global.wat"]).expect("failed to format global.wat");
    assert_eq!(actual, expected);
}

#[test]
fn fix_memory_grow() {
    let expected = include_str!("data/output/memory_grow.wat");
    let actual =
        wasmfmt(&["tests/data/input/memory_grow.wat"]).expect("failed to format memory_grow.wat");
    assert_eq!(actual, expected);
}

#[test]
fn fix_start() {
    let expected = include_str!("data/output/start.wat");
    let actual = wasmfmt(&["tests/data/input/start.wat"]).expect("failed to format start.wat");
    assert_eq!(actual, expected);
}

#[test]
fn check_add_sugar() {
    let source = include_str!("data/input/add_sugar.wat");
    let formatted = include_str!("data/output/add_sugar.wat");
    let result = wasmfmt(&["tests/data/input/add_sugar.wat", "--mode", "check"])
        .expect("failed to check add_sugar.wat");
    assert!(result.contains("Difference found."));
    assert!(result.contains("Source:"));
    assert!(result.contains(source));
    assert!(result.contains("Formatted:"));
    assert!(result.contains(formatted));
}

#[test]
fn writes_to_output_file() -> Result<(), Error> {
    let formatted = include_str!("data/output/add_sugar.wat");
    let tmp = env::temp_dir();
    let out = tmp.join("out.wat");
    let result = wasmfmt(&[
        "tests/data/input/add_sugar.wat",
        "--output",
        out.to_str().unwrap(),
    ])
    .expect("failed to format add_sugar.wat");
    let content = fs::read_to_string(&out)?;
    assert_eq!(content, formatted);
    assert_eq!(result, String::new());
    fs::remove_file(out)?;
    Ok(())
}

#[test]
fn reads_from_stdin_if_no_file_is_provided() -> Result<(), Error> {
    let source = include_str!("data/input/add_sugar.wat");
    let formatted = include_str!("data/output/add_sugar.wat");
    let result = wasmfmt_stdin(source).expect("failed to format add_sugar.wat");
    assert_eq!(result, formatted);
    Ok(())
}
