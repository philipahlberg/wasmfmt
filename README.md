# wasmfmt

[![crates.io version][cratesio-version]][cratesio-crate] [![crates.io downloads][cratesio-downloads]][cratesio-crate]

A tool for formatting `.wat` code.

Available as a command-line tool and a library.

For more information on how to use the library, see the [API docs][docsrs-crate].

## Install

To get `wasmfmt`, you have two options:

1. Download a prebuilt binary from [the releases page][releases]. Binaries are distributed for the following platforms:
    - Linux (x86_64 and ARM64)
    - MacOS (x86_64 and ARM64)
    - Windows (x86_64)
2. Build the tool manually, either from [crates.io][cratesio] or directly from this repository.

To install from [crates.io][cratesio-crate]:

```sh
cargo install wasmfmt
```

To install from source:

```sh
cargo install --path /path/to/wasmfmt
```

## Use

To format a file in-place, use `wasmfmt fix`:

```sh
wasmfmt fix /path/to/file.wat
```

To determine if a file is formatted properly, use `wasmfmt check`:

```sh
wasmfmt check /path/to/file.wat
```

To simply see the formatted version of a file, use `wasmfmt print`:

```sh
wasmfmt print /path/to/file.wat
```

For more information on how to use the tool, use `wasmfmt help`:
```sh
wasmfmt help
```

## Build

To build the binary, use [Cargo][cargo]:

```sh
cargo build --release
```

The generated binary is located in `target/release/wasmfmt`.


## Test

To run the tests, use [Cargo][cargo]:

```sh
cargo test
```

## License

`wasmfmt` is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.

[docsrs-crate]: https://docs.rs/wasmfmt
[cratesio]: https://crates.io
[cratesio-crate]: https://crates.io/crates/wasmfmt
[cratesio-version]: https://img.shields.io/crates/v/wasmfmt
[cratesio-downloads]: https://img.shields.io/crates/d/wasmfmt
[cargo]: https://github.com/rust-lang/cargo
[releases]: https://github.com/philipahlberg/wasmfmt/releases
