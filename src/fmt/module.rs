use super::{Fmt, Formatter};
use super::start::Start;
use wast::{Module, ModuleKind, ModuleField};

impl<'src> Fmt for Module<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write_line("(module");
        formatter.indent();
        formatter.fmt(&self.kind);
        formatter.deindent();
        formatter.write_line(")");
    }
}

impl<'src> Fmt for &ModuleKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ModuleKind::Text(fields) => {
                formatter.fmt(fields);
            }
            ModuleKind::Binary(..) => unimplemented!(),
        }
    }
}

impl<'src> Fmt for &Vec<ModuleField<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        for field in self.iter() {
            formatter.fmt(field);
        }
    }
}

impl<'src> Fmt for &ModuleField<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ModuleField::Type(ty) => formatter.fmt(ty),
            ModuleField::Func(func) => formatter.fmt(func),
            ModuleField::Global(global) => formatter.fmt(global),
            ModuleField::Memory(mem) => formatter.fmt(mem),
            ModuleField::Data(data) => formatter.fmt(data),
            ModuleField::Table(table) => formatter.fmt(table),
            ModuleField::Elem(element) => formatter.fmt(element),
            ModuleField::Export(export) => formatter.fmt(export),
            ModuleField::Import(import) => formatter.fmt(import),
            ModuleField::Start(index) => formatter.fmt(&Start::new(*index.unwrap_index())),
            ModuleField::Custom(..) => todo!(),
            ModuleField::Event(..) => unimplemented!(),
            ModuleField::Instance(..) => unimplemented!(),
            ModuleField::NestedModule(..) => unimplemented!(),
            ModuleField::Alias(..) => unimplemented!(),
        };
    }
}
