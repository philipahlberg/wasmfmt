pub(crate) mod export;
pub(crate) mod expression;
pub(crate) mod func;
pub(crate) mod global;
pub(crate) mod import;
pub(crate) mod index;
pub(crate) mod instruction;
pub(crate) mod memory;
pub(crate) mod module;
pub(crate) mod primitives;
pub(crate) mod start;
pub(crate) mod table;
pub(crate) mod r#type;
pub(crate) mod utils;

use super::{Fmt, Formatter, Options};
use wast::{
    parser::{parse, ParseBuffer},
    Wat,
};

/// Format `.wat` source code.
/// Uses tabs for indentation.
/// Resolves symbolic identifiers and unfolds instruction expressions.
/// Extracts inline exports and type definitions.
/// Encodes all number literals in decimal notation.
pub fn fmt(source: &str, options: Options) -> String {
    let buffer = ParseBuffer::new(source).expect("parse buffer");
    let mut wat = parse::<Wat>(&buffer).expect("parse");
    if options.resolve_names {
        wat.module.resolve().expect("name resolution");
    }
    let mut formatter = Formatter::new();
    formatter.fmt(&wat);
    formatter.into()
}

impl<'src> Fmt for &Wat<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.fmt(&self.module);
    }
}

#[cfg(test)]
mod test {
    use super::{fmt, Options};
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;
    use wast::{
        parser::{self, ParseBuffer},
        Error as WastError, Wat,
    };

    fn parse(input: &str) -> Result<(), WastError> {
        let buffer = ParseBuffer::new(input).unwrap();
        parser::parse::<Wat>(&buffer).map(|_| ())
    }

    mod default {
        use super::{assert_eq, assert_matches, fmt, parse, Options};

        const OPTIONS: Options = Options {
            resolve_names: false,
        };

        #[test]
        fn data() {
            let input = include_str!("../../tests/data/input/data.wat");
            let expected = include_str!("../../tests/data/output/default/data.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn elem() {
            let input = include_str!("../../tests/data/input/elem.wat");
            let expected = include_str!("../../tests/data/output/default/elem.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn exports() {
            let input = include_str!("../../tests/data/input/exports.wat");
            let expected = include_str!("../../tests/data/output/default/exports.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f32_bitwise() {
            let input = include_str!("../../tests/data/input/f32_bitwise.wat");
            let expected = include_str!("../../tests/data/output/default/f32_bitwise.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f32_cmp() {
            let input = include_str!("../../tests/data/input/f32_cmp.wat");
            let expected = include_str!("../../tests/data/output/default/f32_cmp.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f32() {
            let input = include_str!("../../tests/data/input/f32.wat");
            let expected = include_str!("../../tests/data/output/default/f32.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f64_bitwise() {
            let input = include_str!("../../tests/data/input/f64_bitwise.wat");
            let expected = include_str!("../../tests/data/output/default/f64_bitwise.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f64_cmp() {
            let input = include_str!("../../tests/data/input/f64_cmp.wat");
            let expected = include_str!("../../tests/data/output/default/f64_cmp.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f64() {
            let input = include_str!("../../tests/data/input/f64.wat");
            let expected = include_str!("../../tests/data/output/default/f64.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn fac() {
            let input = include_str!("../../tests/data/input/fac.wat");
            let expected = include_str!("../../tests/data/output/default/fac.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn global() {
            let input = include_str!("../../tests/data/input/global.wat");
            let expected = include_str!("../../tests/data/output/default/global.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn i32() {
            let input = include_str!("../../tests/data/input/i32.wat");
            let expected = include_str!("../../tests/data/output/default/i32.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn i64() {
            let input = include_str!("../../tests/data/input/i64.wat");
            let expected = include_str!("../../tests/data/output/default/i64.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn imports() {
            let input = include_str!("../../tests/data/input/imports.wat");
            let expected = include_str!("../../tests/data/output/default/imports.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn memory_grow() {
            let input = include_str!("../../tests/data/input/memory_grow.wat");
            let expected = include_str!("../../tests/data/output/default/memory_grow.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn memory() {
            let input = include_str!("../../tests/data/input/memory.wat");
            let expected = include_str!("../../tests/data/output/default/memory.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn start() {
            let input = include_str!("../../tests/data/input/start.wat");
            let expected = include_str!("../../tests/data/output/default/start.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn table() {
            let input = include_str!("../../tests/data/input/table.wat");
            let expected = include_str!("../../tests/data/output/default/table.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }
    }

    mod resolved {
        use super::{assert_eq, assert_matches, fmt, parse, Options};

        const OPTIONS: Options = Options {
            resolve_names: true,
        };

        #[test]
        fn data() {
            let input = include_str!("../../tests/data/input/data.wat");
            let expected = include_str!("../../tests/data/output/resolved/data.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn elem() {
            let input = include_str!("../../tests/data/input/elem.wat");
            let expected = include_str!("../../tests/data/output/resolved/elem.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn exports() {
            let input = include_str!("../../tests/data/input/exports.wat");
            let expected = include_str!("../../tests/data/output/resolved/exports.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f32_bitwise() {
            let input = include_str!("../../tests/data/input/f32_bitwise.wat");
            let expected = include_str!("../../tests/data/output/resolved/f32_bitwise.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f32_cmp() {
            let input = include_str!("../../tests/data/input/f32_cmp.wat");
            let expected = include_str!("../../tests/data/output/resolved/f32_cmp.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f32() {
            let input = include_str!("../../tests/data/input/f32.wat");
            let expected = include_str!("../../tests/data/output/resolved/f32.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f64_bitwise() {
            let input = include_str!("../../tests/data/input/f64_bitwise.wat");
            let expected = include_str!("../../tests/data/output/resolved/f64_bitwise.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f64_cmp() {
            let input = include_str!("../../tests/data/input/f64_cmp.wat");
            let expected = include_str!("../../tests/data/output/resolved/f64_cmp.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn f64() {
            let input = include_str!("../../tests/data/input/f64.wat");
            let expected = include_str!("../../tests/data/output/resolved/f64.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn fac() {
            let input = include_str!("../../tests/data/input/fac.wat");
            let expected = include_str!("../../tests/data/output/resolved/fac.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn global() {
            let input = include_str!("../../tests/data/input/global.wat");
            let expected = include_str!("../../tests/data/output/resolved/global.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn i32() {
            let input = include_str!("../../tests/data/input/i32.wat");
            let expected = include_str!("../../tests/data/output/resolved/i32.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn i64() {
            let input = include_str!("../../tests/data/input/i64.wat");
            let expected = include_str!("../../tests/data/output/resolved/i64.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn imports() {
            let input = include_str!("../../tests/data/input/imports.wat");
            let expected = include_str!("../../tests/data/output/resolved/imports.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn memory_grow() {
            let input = include_str!("../../tests/data/input/memory_grow.wat");
            let expected = include_str!("../../tests/data/output/resolved/memory_grow.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn memory() {
            let input = include_str!("../../tests/data/input/memory.wat");
            let expected = include_str!("../../tests/data/output/resolved/memory.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn start() {
            let input = include_str!("../../tests/data/input/start.wat");
            let expected = include_str!("../../tests/data/output/resolved/start.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }

        #[test]
        fn table() {
            let input = include_str!("../../tests/data/input/table.wat");
            let expected = include_str!("../../tests/data/output/resolved/table.wat");
            let actual = fmt(input, OPTIONS.clone());
            assert_eq!(actual, expected);
            assert_matches!(parse(&actual), Ok(..));
        }
    }
}
