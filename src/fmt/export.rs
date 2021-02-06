use super::{Fmt, Formatter};
use wast::{Export, ExportKind, InlineExport, ItemRef};

impl<'src> Fmt for &Export<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(export ");
        formatter.fmt(self.name);
        formatter.write(" ");
        formatter.fmt(&self.index);
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &ItemRef<'src, ExportKind> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ItemRef::Outer { kind, module, idx } => {
                formatter.write("(");
                formatter.fmt(kind);
                formatter.write(" ");
                formatter.fmt(module);
                formatter.write(" ");
                formatter.fmt(idx);
                formatter.write(")");
            }
            ItemRef::Item { kind, idx, exports } => {
                formatter.write("(");
                formatter.fmt(kind);
                formatter.write(" ");
                formatter.fmt(idx);
                if !exports.is_empty() {
                    formatter.write(" ");
                    formatter.fmt(exports);
                }
                formatter.write(")");
            }
        }
    }
}

impl<'src> Fmt for &ExportKind {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ExportKind::Func => {
                formatter.write("func");
            }
            ExportKind::Type => {
                formatter.write("type");
            }
            ExportKind::Global => {
                formatter.write("global");
            }
            ExportKind::Memory => {
                formatter.write("memory");
            }
            ExportKind::Table => {
                formatter.write("table");
            }
            ExportKind::Event => unimplemented!(),
            ExportKind::Module => unimplemented!(),
            ExportKind::Instance => unimplemented!(),
        };
    }
}

impl<'src> Fmt for &InlineExport<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(export");
        for name in &self.names {
            formatter.write(" ");
            formatter.write(name);
        }
        formatter.write(")");
    }
}
