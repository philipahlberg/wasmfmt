Version 0.1.0 - Unreleased
==========================

This is the initial release of the `wasmfmt` tool and accompanying library.
With this release, the following features are available:

- The `wasmfmt` CLI can read input from a file, or from stdin.
- The `--output` option designates a file into which the formatted code will be written.
- The `--mode` option has two possible values: `fix` (the default) and `check`.
    - `fix` outputs the formatted source code.
    - `check` outputs an error if the source code is not properly formatted.
- The `wasmfmt` library provides the following:
    - A `Fmt` trait for formatting Wasm AST nodes.
    - A `fmt` function for formatting source code.
    - A `Diff` struct for formatting source code differences.

Below is a brief highlight of the PRs that went into this release:

- [The library function `fmt` has been rewritten for improved ergonomics][27]
- [The README now contains information on how to install, build, test, and use the tool][29]
- [The library now has basic documentation][30]
- [The CLI `--help` output has been significantly improved][32]
- [The CLI option `--mode check` is now usable][33]
- [The RELEASES file, which documents new features and fixes for every release, now exists][38]

[27]: https://github.com/philipahlberg/wasmfmt/pull/27
[29]: https://github.com/philipahlberg/wasmfmt/pull/29
[30]: https://github.com/philipahlberg/wasmfmt/pull/30
[32]: https://github.com/philipahlberg/wasmfmt/pull/32
[33]: https://github.com/philipahlberg/wasmfmt/pull/33
[38]: https://github.com/philipahlberg/wasmfmt/pull/38
