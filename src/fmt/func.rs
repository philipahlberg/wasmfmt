use super::utils::{fmt_long_expression, id_is_gensym, ty_use_is_empty, inline_export_is_empty, inline_import_is_empty};
use super::{Fmt, Formatter};
use wast::{
    kw, Func, FuncKind, FunctionType, Id, ItemRef, Local, NameAnnotation, TypeUse, ValType,
};

impl<'src> Fmt for &Func<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(func");
        if let Some(id) = &self.id {
            if !id_is_gensym(id) {
                formatter.write(" ");
                formatter.fmt(id);
            }
        }
        if !inline_export_is_empty(&self.exports) {
            formatter.write(" ");
            formatter.fmt(&self.exports);
        }
        if let FuncKind::Import(inline_import) = &self.kind {
            if !inline_import_is_empty(inline_import) {
                formatter.write(" ");
                formatter.fmt(inline_import);
            }
        }
        if !ty_use_is_empty(&self.ty) {
            formatter.write(" ");
            formatter.fmt(&self.ty);
        }
        if let FuncKind::Inline { locals, expression } = &self.kind {
            if !locals.is_empty() || !expression.instrs.is_empty() {
                formatter.end_line();
                formatter.indent();
                formatter.fmt(locals);
                fmt_long_expression(expression, formatter);
                formatter.deindent();
                formatter.start_line();
            }
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

impl<'src> Fmt for &[Param<'src>] {
    fn fmt(&self, formatter: &mut Formatter) {
        if !self.is_empty() {
            if params_can_be_abbreviated(self) {
                formatter.write("(param");
                for param in self.iter() {
                    formatter.write(" ");
                    formatter.fmt(&param.2);
                }
                formatter.write(")");
            } else {
                let mut params = self.iter();
                if let Some((id, _, ty)) = params.next() {
                    formatter.write("(param ");
                    if let Some(id) = id {
                        formatter.fmt(id);
                        formatter.write(" ");
                    }
                    formatter.fmt(ty);
                    formatter.write(")");
                }
                for (id, _, ty) in params {
                    formatter.write(" (param ");
                    if let Some(id) = id {
                        formatter.fmt(id);
                        formatter.write(" ");
                    }
                    formatter.fmt(ty);
                    formatter.write(")");
                }
            }
        }
    }
}

fn params_can_be_abbreviated(params: &[Param]) -> bool {
    params.iter().all(param_is_anonymous)
}

fn param_is_anonymous(param: &Param) -> bool {
    param.0.is_none()
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

impl<'src> Fmt for &Vec<Local<'src>> {
    fn fmt(&self, formatter: &mut Formatter) {
        if !self.is_empty() {
            if locals_can_be_abbreviated(self) {
                formatter.start_line();
                formatter.write("(local");
                for local in self.iter() {
                    formatter.write(" ");
                    formatter.fmt(&local.ty);
                }
                formatter.write(")");
                formatter.end_line();
            } else {
                for local in self.iter() {
                    formatter.start_line();
                    formatter.write("(local ");
                    if let Some(id) = &local.id {
                        formatter.fmt(id);
                        formatter.write(" ");
                    }
                    formatter.fmt(&local.ty);
                    formatter.write(")");
                    formatter.end_line();
                }
            }
        }
    }
}

fn locals_can_be_abbreviated(locals: &[Local]) -> bool {
    locals.iter().all(local_is_anonymous)
}

fn local_is_anonymous(local: &Local) -> bool {
    local.id.is_none()
}
