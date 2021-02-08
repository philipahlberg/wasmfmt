# wasmfmt

A tool for formatting your `.wat` code according to style rules.

It can be used as either a command-line tool or as a library.

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

## Usage

To format a file in-place, use `wasmfmt fix`:

```sh
wasmfmt fix examples/fac.wat
```

To determine if a file is already formatted, use `wasmfmt check`:

```sh
wasmfmt check examples/fact.wat
```

This is useful e. g. when running in a CI environment as part of an automated style check.


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

## License

`wasmfmt` is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.
