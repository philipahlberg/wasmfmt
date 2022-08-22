use super::{Fmt, Formatter};
use wast::core::{Export, ExportKind, InlineExport};

impl<'src> Fmt for &Export<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(export ");
        formatter.fmt(self.name);
        formatter.write(" ");
        formatter.write("(");
        formatter.fmt(self.kind);
        formatter.write(" ");
        formatter.fmt(&self.item);
        formatter.write(")");
        formatter.write(")");
        formatter.end_line();
    }
}

impl Fmt for ExportKind {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ExportKind::Func => formatter.write("func"),
            ExportKind::Table => formatter.write("table"),
            ExportKind::Memory => formatter.write("memory"),
            ExportKind::Global => formatter.write("global"),
            ExportKind::Tag => formatter.write("tag"),
        }
    }
}

impl<'src> Fmt for &InlineExport<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        let mut names = self.names.iter();
        if let Some(name) = names.next() {
            formatter.write("(export ");
            formatter.fmt(*name);
            formatter.write(")");
        }
        for name in names {
            formatter.write(" ");
            formatter.write("(export ");
            formatter.fmt(*name);
            formatter.write(")");
        }
    }
}
