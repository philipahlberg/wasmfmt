## Version 0.2.4 - *Not yet released*
=============================

### Added

- [Add GitHub release actions][78]

[78]: https://github.com/philipahlberg/wasmfmt/pull/78

## Version 0.2.3 - 2022-08-19
=============================

### Fixed

- [Update dependencies][371b7]

[371b7]: https://github.com/philipahlberg/wasmfmt/commit/371b75da58a22a375d2269716ea3c9d4b8f4fa72

## Version 0.2.2 - 2021-02-10
=============================

### Fixed

- [Added the missing `f32.mul` instruction][82]
- [Fixed the numeric `f64` instructions][83]

[82]: https://github.com/philipahlberg/wasmfmt/pull/82
[83]: https://github.com/philipahlberg/wasmfmt/pull/83

## Version 0.2.1 - 2021-02-09
=============================

### Fixed

- [Updated README.md to reflect API changes][76]

[76]: https://github.com/philipahlberg/wasmfmt/pull/76

## Version 0.2.0 - 2021-02-08
=============================

### Added

- [Added the `--resolve-names` flag][73]
- [Added the `fix`, `check` and `print` subcommands][74]

### Changed

- [Updated `wast` to `v32.0`][71]


[71]: https://github.com/philipahlberg/wasmfmt/pull/71
[73]: https://github.com/philipahlberg/wasmfmt/pull/73
[74]: https://github.com/philipahlberg/wasmfmt/pull/74

## Version 0.1.1 - 2021-02-02
==========================

### Fixed

- [Block instructions are no longer parenthesised][68]
- [Generated identifiers are no longer rendered][70]


[68]: https://github.com/philipahlberg/wasmfmt/pull/68
[70]: https://github.com/philipahlberg/wasmfmt/pull/70

## Version 0.1.0 - 2021-02-01
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
