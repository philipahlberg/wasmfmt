use super::utils::id_is_gensym;
use super::{Fmt, Formatter};
use wast::{Global, GlobalKind, GlobalType};

impl<'src> Fmt for &Global<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(global ");
        if let Some(id) = &self.id {
            if !id_is_gensym(id) {
                formatter.fmt(id);
                formatter.write(" ");
            }
        };

        if !self.exports.names.is_empty() {
            formatter.fmt(&self.exports);
            formatter.write(" ");
        };

        if let GlobalKind::Import(inline_import) = &self.kind {
            formatter.fmt(inline_import);
        };

        formatter.fmt(&self.ty);
        if let GlobalKind::Inline(expression) = &self.kind {
            formatter.write(" ");
            formatter.fmt(expression);
        };
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &GlobalType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        if self.mutable {
            formatter.write("(mut ");
            formatter.fmt(&self.ty);
            formatter.write(")");
        } else {
            formatter.fmt(&self.ty);
        }
    }
}
