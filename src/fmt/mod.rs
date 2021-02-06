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

use wast::{
    parser::{parse, ParseBuffer},
    Wat,
};

/// A formatter used to format individual AST nodes.
pub struct Formatter {
    buffer: String,
    indentation: usize,
}

impl Formatter {
    /// Construct a new formatter with an empty buffer and zero initial indentation.
    pub fn new() -> Self {
        Self::default()
    }

    fn indent(&mut self) {
        self.indentation += 1;
    }

    fn deindent(&mut self) {
        self.indentation -= 1;
    }

    fn start_line(&mut self) {
        self.buffer.push_str(&"\t".repeat(self.indentation));
    }

    fn end_line(&mut self) {
        self.buffer.push('\n');
    }

    fn write(&mut self, string: &str) {
        self.buffer.push_str(string);
    }

    fn write_line(&mut self, string: &str) {
        self.start_line();
        self.write(string);
        self.end_line();
    }

    fn fmt<T: Fmt>(&mut self, v: T) {
        v.fmt(self);
    }
}

impl Default for Formatter {
    fn default() -> Self {
        Self {
            buffer: String::default(),
            indentation: usize::default(),
        }
    }
}

impl Into<String> for Formatter {
    fn into(self) -> String {
        self.buffer
    }
}

/// The `Fmt` trait allows individual WebAssembly AST nodes
/// to be formatted separately.
pub trait Fmt {
    fn fmt(&self, formatter: &mut Formatter);
}

/// Format `.wat` source code.
/// Uses tabs for indentation.
/// Resolves symbolic identifiers and unfolds instruction expressions.
/// Extracts inline exports and type definitions.
/// Encodes all number literals in decimal notation.
pub fn fmt(source: &str) -> String {
    let buffer = ParseBuffer::new(source).unwrap();
    let mut wat = parse::<Wat>(&buffer).unwrap();
    // TODO: Handle error
    wat.module.resolve().unwrap();
    let mut formatter = Formatter::new();
    wat.fmt(&mut formatter);
    formatter.into()
}

impl<'src> Fmt for Wat<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        self.module.fmt(formatter)
    }
}

#[cfg(test)]
mod test {
    use super::fmt;
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

    #[test]
    fn data() {
        let input = include_str!("../../tests/data/input/data.wat");
        let expected = include_str!("../../tests/data/output/data.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn elem() {
        let input = include_str!("../../tests/data/input/elem.wat");
        let expected = include_str!("../../tests/data/output/elem.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn fac() {
        let input = include_str!("../../tests/data/input/fac.wat");
        let expected = include_str!("../../tests/data/output/fac.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn global() {
        let input = include_str!("../../tests/data/input/global.wat");
        let expected = include_str!("../../tests/data/output/global.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn i32() {
        let input = include_str!("../../tests/data/input/i32.wat");
        let expected = include_str!("../../tests/data/output/i32.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn imports() {
        let input = include_str!("../../tests/data/input/imports.wat");
        let expected = include_str!("../../tests/data/output/imports.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn memory_grow() {
        let input = include_str!("../../tests/data/input/memory_grow.wat");
        let expected = include_str!("../../tests/data/output/memory_grow.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn memory() {
        let input = include_str!("../../tests/data/input/memory.wat");
        let expected = include_str!("../../tests/data/output/memory.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn start() {
        let input = include_str!("../../tests/data/input/start.wat");
        let expected = include_str!("../../tests/data/output/start.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }

    #[test]
    fn table() {
        let input = include_str!("../../tests/data/input/table.wat");
        let expected = include_str!("../../tests/data/output/table.wat");
        let actual = fmt(input);
        assert_eq!(actual, expected);
        assert_matches!(parse(&actual), Ok(..));
    }
}
