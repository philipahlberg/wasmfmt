Version 0.1.0 - 2021-02-01
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

- [The README now contains information on how to install, build, test, and use the tool][29]
- [The library now has basic documentation][30]
- [The CLI option `--mode check` has been added][33]
- [The library now supports globals][40]
- [The library now supports start functions][41]
- [The library now supports memories and data segments][44]
- [The library now supports tables and element segments][48]
- [The library now supports imports][50]
- [The library now supports exports][51]

[29]: https://github.com/philipahlberg/wasmfmt/pull/29
[30]: https://github.com/philipahlberg/wasmfmt/pull/30
[33]: https://github.com/philipahlberg/wasmfmt/pull/33
[40]: https://github.com/philipahlberg/wasmfmt/pull/40
[41]: https://github.com/philipahlberg/wasmfmt/pull/41
[44]: https://github.com/philipahlberg/wasmfmt/pull/44
[48]: https://github.com/philipahlberg/wasmfmt/pull/48
[50]: https://github.com/philipahlberg/wasmfmt/pull/50
[51]: https://github.com/philipahlberg/wasmfmt/pull/51
