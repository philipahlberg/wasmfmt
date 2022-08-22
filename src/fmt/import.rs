use super::utils::id_is_gensym;
use super::{Fmt, Formatter};
use wast::core::{Import, InlineImport, ItemKind, ItemSig};

impl<'src> Fmt for &Import<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(import ");
        formatter.fmt(self.module);
        formatter.write(" ");
        formatter.fmt(self.field);
        formatter.write(" ");
        formatter.fmt(&self.item);
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &ItemSig<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        // TODO: This should be an impl on `ItemKind`
        match &self.kind {
            ItemKind::Func(ty_use) => {
                formatter.write("(func ");
                if let Some(id) = &self.id {
                    if !id_is_gensym(id) {
                        formatter.fmt(id);
                        formatter.write(" ");
                    }
                }
                formatter.fmt(ty_use);
                formatter.write(")");
            }
            ItemKind::Table(table_ty) => {
                formatter.write("(table ");
                if let Some(id) = &self.id {
                    if !id_is_gensym(id) {
                        formatter.fmt(id);
                        formatter.write(" ");
                    }
                }
                formatter.fmt(table_ty);
                formatter.write(")");
            }
            ItemKind::Memory(memory_ty) => {
                formatter.write("(memory ");
                if let Some(id) = &self.id {
                    if !id_is_gensym(id) {
                        formatter.fmt(id);
                        formatter.write(" ");
                    }
                }
                formatter.fmt(memory_ty);
                formatter.write(")");
            }
            ItemKind::Global(global_ty) => {
                formatter.write("(global ");
                if let Some(id) = &self.id {
                    if !id_is_gensym(id) {
                        formatter.fmt(id);
                        formatter.write(" ");
                    }
                }
                formatter.fmt(global_ty);
                formatter.write(")");
            }
            ItemKind::Tag(..) => unimplemented!(),
        }
    }
}

impl<'src> Fmt for &InlineImport<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(import ");
        formatter.fmt(self.module);
        formatter.write(" ");
        formatter.fmt(self.field);
        formatter.write(")");
    }
}
