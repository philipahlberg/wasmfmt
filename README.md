# wasmfmt

[![crates.io version][cratesio-version]][cratesio-crate] [![crates.io downloads][cratesio-downloads]][cratesio-crate]

A tool for formatting `.wat` code.

Available as a command-line tool and a library.

For more information on how to use the library, see the [API docs][docsrs-crate].

## Install

To install the tool, first install [Cargo][cargo].

Then, to install from [crates.io][cratesio]:

```sh
cargo install wasmfmt
```

Alternatively, to install from source:

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
