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

    pub(crate) fn indent(&mut self) {
        self.indentation += 1;
    }

    pub(crate) fn deindent(&mut self) {
        self.indentation -= 1;
    }

    pub(crate) fn start_line(&mut self) {
        self.buffer.push_str(&"\t".repeat(self.indentation));
    }

    pub(crate) fn end_line(&mut self) {
        self.buffer.push('\n');
    }

    pub(crate) fn write(&mut self, string: &str) {
        self.buffer.push_str(string);
    }

    pub(crate) fn write_line(&mut self, string: &str) {
        self.start_line();
        self.write(string);
        self.end_line();
    }

    pub(crate) fn fmt<T: Fmt>(&mut self, v: T) {
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
