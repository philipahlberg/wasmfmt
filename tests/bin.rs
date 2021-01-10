use std::process::{Command, Stdio};
use std::io::Write;

const BIN: &'static str = env!("CARGO_BIN_EXE_WASMFMT");

fn run(input: &str) -> Result<String, String> {
	let mut process = Command::new(BIN)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to spawn process");

	let stdin = process.stdin.as_mut().expect("failed to get stdin handle from process");
	stdin.write_all(input.as_bytes()).expect("failed to write to process' stdin");
	let output = process.wait_with_output().expect("failed to get process output");

	let stdout = String::from_utf8(output.stdout).expect("stdout");
	let stderr = String::from_utf8(output.stderr).expect("stderr");

	if !stderr.is_empty() {
		Err(stderr)
	} else {
		Ok(stdout)
	}
}

#[test]
fn add_desugar() {
	let input = include_str!("data/input/add_desugar.wat");
	let expected = include_str!("data/output/add_desugar.wat");
	let actual = run(input).expect("failed to format add_desugar.wat");
	assert_eq!(actual, expected);
}

#[test]
fn add_sugar() {
	let input = include_str!("data/input/add_sugar.wat");
	let expected = include_str!("data/output/add_sugar.wat");
	let actual = run(input).expect("failed to format add_sugar.wat");
	assert_eq!(actual, expected);
}

#[test]
fn fac_desugar() {
	let input = include_str!("data/input/fac_desugar.wat");
	let expected = include_str!("data/output/fac_desugar.wat");
	let actual = run(input).expect("failed to format fac_desugar.wat");
	assert_eq!(actual, expected);
}

#[test]
fn fac_sugar() {
	let input = include_str!("data/input/fac_sugar.wat");
	let expected = include_str!("data/output/fac_sugar.wat");
	let actual = run(input).expect("failed to format fac_sugar.wat");
	assert_eq!(actual, expected);
}
