use super::{Fmt, Formatter};
use super::utils::id_is_gensym;
use wast::{Type, TypeDef, FunctionType};

impl<'src> Fmt for &Type<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(type ");
        if let Some(id) = &self.id {
            if !id_is_gensym(id) {
                formatter.fmt(id);
                formatter.write(" ");
            }
        }
        formatter.fmt(&self.def);
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &TypeDef<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            TypeDef::Func(functy) => functy.fmt(formatter),
            TypeDef::Struct(..) => unimplemented!(),
            TypeDef::Array(..) => unimplemented!(),
            TypeDef::Module(..) => unimplemented!(),
            TypeDef::Instance(..) => unimplemented!(),
        };
    }
}

impl<'src> Fmt for &FunctionType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(func");
        if !self.params.is_empty() {
            formatter.write(" ");
        }
        formatter.fmt(&*self.params);
        if !self.results.is_empty() {
            formatter.write(" ");
        }
        formatter.fmt(&*self.results);
        formatter.write(")");
    }
}
