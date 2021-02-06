use super::{Fmt, Formatter};
use super::utils::{id_is_gensym, func_kind_is_empty, fmt_long_expression};
use wast::{Func, TypeUse, FunctionType, ItemRef, Id, NameAnnotation, ValType, FuncKind, Local, kw};

impl<'src> Fmt for &Func<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(func ");
        if let Some(id) = &self.id {
            if !id_is_gensym(id) {
                formatter.fmt(id);
                formatter.write(" ");
            }
        }
        formatter.fmt(&self.ty);
        if !func_kind_is_empty(&self.kind) {
            formatter.end_line();
            formatter.indent();
            formatter.fmt(&self.kind);
            formatter.deindent();
            formatter.start_line();
        }
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &TypeUse<'src, FunctionType<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        if let Some(index) = &self.index {
            formatter.fmt(index);
        };

        if let Some(functy) = &self.inline {
            if !functy.params.is_empty() {
                if self.index.is_some() {
                    formatter.write(" ");
                }
                formatter.fmt(&*functy.params);
            }
            if !functy.results.is_empty() {
                if self.index.is_some() || !functy.params.is_empty() {
                    formatter.write(" ");
                }
                formatter.fmt(&*functy.results);
            }
        };
    }
}

impl<'src> Fmt for &ItemRef<'src, kw::r#type> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.write("(type ");
        formatter.fmt(self.unwrap_index());
        formatter.write(")");
    }
}

type Param<'src> = (
    Option<Id<'src>>,
    Option<NameAnnotation<'src>>,
    ValType<'src>,
);

// TODO: ID, Name
impl<'src> Fmt for &[Param<'src>] {
    fn fmt(&self, formatter: &mut Formatter) {
        if !self.is_empty() {
            formatter.write("(param");
            for param in self.iter() {
                formatter.write(" ");
                formatter.fmt(&param.2);
            }
            formatter.write(")");
        }
    }
}

impl<'src> Fmt for &[ValType<'src>] {
    fn fmt(&self, formatter: &mut Formatter) {
        if !self.is_empty() {
            formatter.write("(result");
            for result in self.iter() {
                formatter.write(" ");
                formatter.fmt(result);
            }
            formatter.write(")");
        }
    }
}

impl<'src> Fmt for &ValType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ValType::I32 => formatter.write("i32"),
            ValType::I64 => formatter.write("i64"),
            ValType::F32 => formatter.write("f32"),
            ValType::F64 => formatter.write("f64"),
            ValType::V128 => unimplemented!(),
            ValType::Ref(..) => unimplemented!(),
            ValType::Rtt(..) => unimplemented!(),
        };
    }
}

impl<'src> Fmt for &FuncKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            FuncKind::Import(import) => formatter.fmt(import),
            FuncKind::Inline { locals, expression } => {
                formatter.fmt(locals);
                fmt_long_expression(expression, formatter);
            }
        };
    }
}

impl<'src> Fmt for &Vec<Local<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        if !self.is_empty() {
            formatter.start_line();
            formatter.write("(local");
            for local in self.iter() {
                formatter.write(" ");
                formatter.fmt(&local.ty);
            }
            formatter.write(")");
            formatter.end_line();
        }
    }
}
