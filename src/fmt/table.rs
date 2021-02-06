use super::utils::{expr_is_const, id_is_gensym, index_is_default};
use super::{Fmt, Formatter};
use wast::{
    Elem, ElemKind, ElemPayload, HeapType, Index, Limits, RefType, Table, TableKind, TableType,
};

impl<'src> Fmt for &Table<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(table ");
        if let Some(id) = &self.id {
            if !id_is_gensym(id) {
                formatter.fmt(id);
                formatter.write(" ");
            }
        }
        if !self.exports.names.is_empty() {
            formatter.fmt(&self.exports);
            formatter.write(" ");
        }
        formatter.fmt(&self.kind);
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &TableKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            TableKind::Import { import, ty } => {
                formatter.fmt(import);
                formatter.write(" ");
                formatter.fmt(ty);
            }
            TableKind::Normal(ty) => {
                formatter.fmt(ty);
            }
            TableKind::Inline { elem, payload } => {
                formatter.fmt(elem);
                if !elem_payload_is_empty(payload) {
                    formatter.write(" ");
                    formatter.fmt(payload);
                }
            }
        }
    }
}

impl<'src> Fmt for &TableType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.fmt(&self.limits);
        formatter.write(" ");
        formatter.fmt(&self.elem);
    }
}

impl<'src> Fmt for &Limits {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.fmt(self.min);
        if let Some(max) = self.max {
            formatter.write(" ");
            formatter.fmt(max);
        }
    }
}

impl<'src> Fmt for &RefType<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self.heap {
            HeapType::Func => {
                formatter.write("funcref");
            }
            HeapType::Extern => unimplemented!(),
            HeapType::Any => unimplemented!(),
            HeapType::Exn => unimplemented!(),
            HeapType::Eq => unimplemented!(),
            HeapType::I31 => unimplemented!(),
            HeapType::Index(..) => unimplemented!(),
        }
    }
}

impl<'src> Fmt for &Elem<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        formatter.start_line();
        formatter.write("(elem ");
        if let Some(id) = &self.id {
            if !id_is_gensym(id) {
                formatter.fmt(id);
                formatter.write(" ");
            }
        }
        formatter.fmt(&self.kind);
        if !elem_payload_is_empty(&self.payload) {
            formatter.write(" ");
            formatter.fmt(&self.payload);
        }
        formatter.write(")");
        formatter.end_line();
    }
}

impl<'src> Fmt for &ElemKind<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ElemKind::Passive => todo!(),
            ElemKind::Declared => todo!(),
            ElemKind::Active { table, offset } => {
                let index = table.unwrap_index();
                if !index_is_default(index) {
                    formatter.fmt(index);
                    formatter.write(" ");
                }
                if expr_is_const(offset) {
                    formatter.fmt(offset);
                } else {
                    formatter.write("(offset ");
                    formatter.fmt(offset);
                    formatter.write(")");
                }
            }
        };
    }
}

impl<'src> Fmt for &ElemPayload<'src> {
    fn fmt(&self, formatter: &mut Formatter) {
        match self {
            ElemPayload::Indices(refs) => {
                let indices: Vec<Index<'src>> = refs
                    .iter()
                    .map(|item_ref| *item_ref.unwrap_index())
                    .collect();
                formatter.fmt(&indices);
            }
            ElemPayload::Exprs { .. } => unimplemented!(),
        }
    }
}

fn elem_payload_is_empty(payload: &ElemPayload) -> bool {
    match payload {
        ElemPayload::Indices(indices) => indices.is_empty(),
        ElemPayload::Exprs { exprs, .. } => exprs.is_empty(),
    }
}
