# wasmfmt

A tool for formatting your `.wat` code according to style rules.

It can be used as either a command-line tool or a Rust library.

For more information on how to use the library, see the [API docs](https://docs.rs/wasmfmt).


## Install

To install the tool, use [Cargo](https://github.com/rust-lang/cargo).

To install from [crates.io](https://crates.io/):

```sh
cargo install wasmfmt
```

To install from source:

```sh
cargo install --path .
```


## Build

To build the binary, use [Cargo](https://github.com/rust-lang/cargo):

```sh
cargo build --release
```

The generated binary is located in `target/release/wasmfmt`.


## Test

To run the tests, use [Cargo](https://github.com/rust-lang/cargo):

```sh
cargo test
```


## Usage

The tool has two primary operating modes, specified by the `--mode` flag.

The default is `--mode fix`, where the input file is overwritten with the formatted code:

```sh
wasmfmt examples/fac.wat --mode fix # `--mode fix` is optional
```

The alternative is `--mode check`, where a diff of the source code and the formatted code is printed to stdout:

```sh
wasmfmt examples/fact.wat --mode check
```

This is useful e. g. when running in a CI environment as part of an automated style check.


## Options

| Option   | Description                                             | Default             |
| -------- | ------------------------------------------------------- | ------------------- |
| `file`   | The file to operate on.                                 | Reads from `stdin`. |
| `--mode` | The operating mode; overwrite the file, or show a diff. | `fix`               |


## License

`wasmfmt` is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.
